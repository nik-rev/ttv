use crossterm::event::{self, Event};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};
use std::{
    env,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result, anyhow};
use ffmpeg::{format, frame};
use image::{DynamicImage, RgbImage};

/// Returns a list of frames for this video
fn get_frames_of_video_at_path(video_path: &Path) -> Result<Vec<DynamicImage>> {
    ffmpeg::init().map_err(|e| anyhow!("Failed to initialize FFmpeg: {e}"))?;

    let mut input =
        format::input(video_path).map_err(|e| anyhow!("Failed to open input file: {e}"))?;

    let stream = input
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(anyhow!("No video stream found"))?;

    let stream_index = stream.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(stream.parameters())
        .map_err(|e| anyhow!("Failed to create decoder context: {e}"))?;
    let mut decoder = context_decoder
        .decoder()
        .video()
        .map_err(|e| anyhow!("Failed to create video decoder: {e}"))?;

    let mut decoded_frame = frame::Video::empty();
    let mut rgb_frame = frame::Video::empty();

    let width = decoder.width();
    let height = decoder.height();

    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        width,
        height,
        ffmpeg::format::Pixel::RGB24,
        width,
        height,
        ffmpeg::software::scaling::Flags::BILINEAR,
    )
    .map_err(|e| anyhow!("Failed to create scaler: {e}"))?;

    let mut frames = vec![];

    for (stream, packet) in input.packets() {
        if stream.index() == stream_index {
            decoder
                .send_packet(&packet)
                .map_err(|e| anyhow!("Failed to send packet to decoder: {e}"))?;

            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                scaler
                    .run(&decoded_frame, &mut rgb_frame)
                    .map_err(|e| anyhow!("Failed to convert frame to RGB: {e}"))?;
                let width = rgb_frame.width();
                let height = rgb_frame.height();

                let pixels_with_padding = rgb_frame.data(0);

                // bytes from one row to the next one
                // pixel_stride =  3 (RGB) + pixel_padding
                let pixel_stride = rgb_frame.stride(0);

                // 3 bytes per pixel
                let bytes_per_row = width as usize * 3;

                let mut pixels = Vec::with_capacity(height as usize * bytes_per_row);

                for y in 0..height as usize {
                    let pixel_start = y * pixel_stride;
                    // Omit padding at the end of each row
                    let pixel_end = pixel_start + bytes_per_row;
                    pixels.extend_from_slice(&pixels_with_padding[pixel_start..pixel_end]);
                }

                frames.push(image::DynamicImage::ImageRgb8(
                    RgbImage::from_raw(width, height, pixels).context(
                        "Failed to construct RgbImage from a frame after processing stride",
                    )?,
                ));
            }
        }
    }

    Ok(frames)
}

struct App {
    frames: Vec<StatefulProtocol>,
}

const FPS: u32 = 400;
const TICK_RATE: Duration = Duration::from_millis(1000 / FPS as u64);

fn main() -> Result<()> {
    let _ = color_eyre::install();

    let video = env::args()
        .nth(1)
        .context("Please provide a video file to play (as the 1st argument)!")?;

    let mut terminal = ratatui::init();
    let start = Instant::now();
    let picker = Picker::from_query_stdio()?;

    let frames = get_frames_of_video_at_path(Path::new(&video))?;

    let frames = frames
        .iter()
        .map(|frame| picker.new_resize_protocol(frame.clone()))
        .collect();

    let mut app = App { frames };

    loop {
        if event::poll(TICK_RATE)? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        terminal.draw(|f| {
            let frame_index =
                (start.elapsed().as_millis() / TICK_RATE.as_millis()) as usize % app.frames.len();

            let image = StatefulImage::default();
            f.render_stateful_widget(image, f.area(), &mut app.frames[frame_index]);
        })?;
    }

    for mut img in app.frames {
        img.last_encoding_result()
            .context("Failed to get the last encoding result")??;
    }
    Ok(())
}

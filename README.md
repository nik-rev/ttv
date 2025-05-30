# tty

This project is an attempt to create a media player that runs in the terminal!

Status: It Works™, but it's very flickery and inefficient. The implementation is also incredibly naive - it gets array of frames from the video using `ffmpeg` and renders it using `ratatui`.

## Showcase

https://github.com/user-attachments/assets/a23f7873-5551-4dc8-a20c-bb5f323fb8c7

## Building

Follow the instructions on building [ffmpeg_next](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building), as this crate depends on it. Then just do `cargo run --release -- video.mp4`.

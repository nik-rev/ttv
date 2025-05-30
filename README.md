# ttv

This project is an attempt to create a media player that runs **in the terminal**!

Status: It Works™, but it's very flickery and inefficient. The implementation is also incredibly naive - it gets array of frames from the video using [`ffmpeg`](https://ffmpeg.org/) and renders each frame using [`ratatui-image`](https://crates.io/crates/ratatui-image).

## Showcase

https://github.com/user-attachments/assets/a23f7873-5551-4dc8-a20c-bb5f323fb8c7

It's flickery as terminals need to re-draw everything for each frame. It'll be nice to get rid of the flicker, I think to accomplish this instead of doing `frame A -> clear screen -> frame B` we can directly go from `frame A -> frame B`. Likely to accomplish this we will need changes from the side of the terminal emulator.

## Building

Follow the instructions on building [ffmpeg_next](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building), as this crate depends on it. Then just do `cargo run --release -- video.mp4`.

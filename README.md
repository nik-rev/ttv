# ttv

This project is an attempt to create a media player that runs **in the terminal**!

Status: It Works™, but it's very flickery and inefficient. The implementation is also incredibly naive - it gets array of frames from the video using [`ffmpeg`](https://ffmpeg.org/) and renders each frame using [`ratatui-image`](https://crates.io/crates/ratatui-image).

## Showcase


https://github.com/user-attachments/assets/d1e80f9c-1efd-4ee8-91fd-9a5341646511

In the demo I'm using [`Rio`](https://github.com/raphamorim/rio) which has by far the best performance out of all other terminals.

It's flickery as terminals need to re-draw everything for each frame. It'll be nice to get rid of the flicker.

## Building

Follow the instructions on building [ffmpeg_next](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building), as this crate depends on it. Then just do `cargo run --release -- demo.mp4`.

To download the demo video:

```
yt-dlp -f mp4 https://www.youtube.com/watch?v=WO2b03Zdu4Q -o demo.mp4
```

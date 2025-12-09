# ttv

This project is a proof-of-concept demo of a video playing in the terminal!

## Showcase

https://github.com/user-attachments/assets/8d39aa5d-35c7-46ab-b607-d58cf8f3f318

## Building

Follow the instructions on building [ffmpeg_next](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building), as this crate depends on it. Then just do:

```rs
cargo run --release -- demo.mp4
```

To download the demo video:

```
yt-dlp -f mp4 https://www.youtube.com/watch?v=WO2b03Zdu4Q -o demo.mp4
```
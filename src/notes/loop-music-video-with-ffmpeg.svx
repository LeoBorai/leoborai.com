---
title: "Join Songs and Loop a Video with FFmpeg"
description: "Using FFmpeg to build a endless-like video with music background"
categories: [ffmpeg, audio, video, macos]
icon: dev
date: 2026-10-18
preview_image_url: ""
published: true
---

On my spare time I am learning how to produce electronic music with Logic Pro and recently
I wanted to make one of those "infinity" like videos, such as those for Lo-Fi beats.

I had my tracks ready, and the video so I leveraged my developer skills to shine on the making
of this video so I didn't had to use any commercial software for this purpose thanks to [FFmpeg][1].

## Joinning the songs

The following command served me well for having a single MP3 file with the whole mix:

```bash
ffmpeg -i song1.mp3 -i song2.mp3 -i song3.mp3 -i song4.mp3 \
    -filter_complex "[0:a][1:a][2:a][3:a]concat=n=4:v=0:a=1[a]" \
    -map "[a]" combined.mp3
```

Let's destrcucture it:

- `-i songN.mp3` declares every input file. FFmpeg numbers them from `0`.
- `[0:a][1:a][2:a][3:a]` selects the audio stream of each input, in order.
- `concat=n=4:v=0:a=1` joins `4` segments into one, with no video streams (`v=0`) and one audio
  stream (`a=1`). `n` must match the number of inputs.
- `[a]` names the resulting stream, and `-map "[a]"` writes it to `combined.mp3`.

The songs play back to back, in the order of the `-i` flags.
If you add or remove a song, update both the list of `[N:a]` selectors and `n`.

## Loop the video

To repeat the video until `combined.mp3` ends I ran:

```bash
ffmpeg -stream_loop -1 -i video.mov -i combined.mp3 \
    -map 0:v:0 -map 1:a:0 -c:v libx264 -c:a aac -shortest output.mp4
```

Let's destrcucture it:

- `-stream_loop -1` loops the next input (`video.mov`) forever. It must come _before_ the `-i` it
  applies to.
- `-map 0:v:0 -map 1:a:0` takes the video from the first input and the audio from the second, so the
  original audio of the clip, if any, is ignored.
- `-c:v libx264 -c:a aac` encodes to H.264 and AAC, which play nearly everywhere.
- `-shortest` stops encoding when the shortest input ends. Since the video loops forever, the
  audio is the shortest one, and the output is exactly as long as the combined songs.

Without `-shortest` FFmpeg would never stop, because the looped video has no end.

## Result

The `output.mp4` video lasts as long as the combined songs, the clip repeats in the background.

[1]: https://ffmpeg.org/

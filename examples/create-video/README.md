# Create Video Example

Create a short video using the Videos API, wait for it to finish rendering, and download the resulting MP4 file.

## Prerequisites

- Rust 1.75 or newer
- An OpenAI API key with access to the Videos API

## Usage

```bash
cd examples/create-video
OPENAI_API_KEY="sk-..." cargo run
```

The example saves the resulting clip under `videos/<video_id>.mp4`.

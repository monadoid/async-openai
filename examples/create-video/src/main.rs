use std::time::Duration;

use async_openai::{
    error::OpenAIError,
    types::{
        CreateVideoRequestArgs, RetrieveVideoContentParamsArgs, VideoContentVariant, VideoStatus,
    },
    Client,
};
use tokio::{fs, time::sleep};

const OUTPUT_DIR: &str = "videos";

#[tokio::main]
async fn main() -> Result<(), OpenAIError> {
    let client = Client::new();

    let request = CreateVideoRequestArgs::default()
        .prompt("A calico cat playing a piano on stage")
        .build()
        .unwrap();

    let mut video = client.videos().create(request).await?;
    println!(
        "Created video job: {} (status: {:?})",
        video.id, video.status
    );

    let mut attempts = 0;
    while !matches!(video.status, VideoStatus::Completed | VideoStatus::Failed) {
        attempts += 1;
        if attempts > 60 {
            println!(
                "Video still in {:?} after {} attempts, exiting.",
                video.status, attempts
            );
            return Ok(());
        }

        sleep(Duration::from_secs(10)).await;

        video = client.videos().retrieve(&video.id).await?;
        println!(
            "Checked video job: {} (status: {:?})",
            video.id, video.status
        );
    }

    if video.status == VideoStatus::Failed {
        println!(
            "Video job failed: {}",
            video
                .error
                .map(|err| format!("{:?}", err))
                .unwrap_or_else(|| "unknown error".into())
        );
        return Ok(());
    }

    let params = RetrieveVideoContentParamsArgs::default()
        .variant(VideoContentVariant::Video)
        .build()
        .unwrap();

    let bytes = client.videos().download_content(&video.id, &params).await?;

    fs::create_dir_all(OUTPUT_DIR)
        .await
        .map_err(|e| OpenAIError::FileSaveError(e.to_string()))?;
    let output_path = format!("{OUTPUT_DIR}/{}.mp4", video.id);
    fs::write(&output_path, &bytes)
        .await
        .map_err(|e| OpenAIError::FileSaveError(e.to_string()))?;
    println!("Saved video to {output_path}");

    Ok(())
}

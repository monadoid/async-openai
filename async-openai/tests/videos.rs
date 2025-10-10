use async_openai::{
    error::OpenAIError,
    types::{CreateVideoRequestArgs, VideoInput},
    Client,
};
use tokio_test::assert_err;

#[tokio::test]
async fn create_video_with_missing_reference_returns_file_error() {
    let client = Client::new();

    let request = CreateVideoRequestArgs::default()
        .prompt("A calico cat playing piano")
        .input_reference(VideoInput::from("does_not_exist.png"))
        .build()
        .unwrap();

    let result = client.videos().create(request).await;
    assert_err!(&result);
    assert!(matches!(result.unwrap_err(), OpenAIError::FileReadError(_)));
}

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY and access to the Videos API"]
async fn create_video_live() -> Result<(), OpenAIError> {
    // Ensure the key is set up before running so the test fails fast if the environment is missing.
    std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set to run the live video test");

    let client = Client::new();

    let request = CreateVideoRequestArgs::default()
        .prompt("A calico cat playing a piano on stage")
        .build()
        .unwrap();

    let response = client.videos().create(request).await?;
    assert_eq!(response.object, "video");
    assert!(response.id.starts_with("video_"));

    Ok(())
}

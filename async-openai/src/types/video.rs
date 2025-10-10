use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::OpenAIError;

use super::InputSource;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum VideoModel {
    #[default]
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "sora-2-pro")]
    Sora2Pro,
    #[serde(untagged)]
    Other(String),
}

impl std::fmt::Display for VideoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sora2 => write!(f, "sora-2"),
            Self::Sora2Pro => write!(f, "sora-2-pro"),
            Self::Other(model) => write!(f, "{model}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VideoStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VideoSize {
    #[serde(rename = "720x1280")]
    S720x1280,
    #[serde(rename = "1280x720")]
    S1280x720,
    #[serde(rename = "1024x1792")]
    S1024x1792,
    #[serde(rename = "1792x1024")]
    S1792x1024,
}

impl std::fmt::Display for VideoSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S720x1280 => write!(f, "720x1280"),
            Self::S1280x720 => write!(f, "1280x720"),
            Self::S1024x1792 => write!(f, "1024x1792"),
            Self::S1792x1024 => write!(f, "1792x1024"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VideoSeconds {
    #[serde(rename = "4")]
    S4,
    #[serde(rename = "8")]
    S8,
    #[serde(rename = "12")]
    S12,
}

impl std::fmt::Display for VideoSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S4 => write!(f, "4"),
            Self::S8 => write!(f, "8"),
            Self::S12 => write!(f, "12"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VideoOrder {
    Asc,
    #[default]
    Desc,
}

impl std::fmt::Display for VideoOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VideoContentVariant {
    #[default]
    Video,
    Thumbnail,
    Spritesheet,
}

impl std::fmt::Display for VideoContentVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Video => write!(f, "video"),
            Self::Thumbnail => write!(f, "thumbnail"),
            Self::Spritesheet => write!(f, "spritesheet"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VideoInput {
    pub source: InputSource,
}

#[derive(Debug, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateVideoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateVideoRequest {
    /// Text prompt that describes the video to generate.
    pub prompt: String,
    /// The video generation model to use. Defaults to `sora-2`.
    #[builder(default)]
    pub model: Option<VideoModel>,
    /// Optional image reference that guides generation.
    #[builder(default)]
    pub input_reference: Option<VideoInput>,
    /// Clip duration in seconds. Defaults to 4 seconds.
    #[builder(default)]
    pub seconds: Option<VideoSeconds>,
    /// Output resolution formatted as width x height. Defaults to `720x1280`.
    #[builder(default)]
    pub size: Option<VideoSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "CreateVideoRemixRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct CreateVideoRemixRequest {
    /// Updated text prompt that directs the remix generation.
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "ListVideosRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct ListVideosRequest {
    /// Number of items to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order of results by timestamp. Use `asc` for ascending order or `desc` for descending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<VideoOrder>,
    /// Identifier for the last item from the previous pagination request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder, PartialEq)]
#[builder(name = "RetrieveVideoContentParamsArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "OpenAIError"))]
pub struct RetrieveVideoContentParams {
    /// Which downloadable asset to return. Defaults to the MP4 video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<VideoContentVariant>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoResource {
    /// Unique identifier for the video job.
    pub id: String,
    /// The object type, which is always `video`.
    pub object: String,
    /// The video generation model that produced the job.
    pub model: VideoModel,
    /// Current lifecycle status of the video job.
    pub status: VideoStatus,
    /// Approximate completion percentage for the generation task.
    pub progress: u32,
    /// Unix timestamp (seconds) for when the job was created.
    pub created_at: i64,
    /// Unix timestamp (seconds) for when the job completed, if finished.
    pub completed_at: Option<i64>,
    /// Unix timestamp (seconds) for when the downloadable assets expire, if set.
    pub expires_at: Option<i64>,
    /// The resolution of the generated video.
    pub size: VideoSize,
    /// Duration of the generated clip in seconds.
    pub seconds: VideoSeconds,
    /// Identifier of the source video if this video is a remix.
    pub remixed_from_video_id: Option<String>,
    /// Error payload that explains why generation failed, if applicable.
    pub error: Option<VideoError>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoListResource {
    pub object: String,
    pub data: Vec<VideoResource>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeletedVideoResource {
    pub object: String,
    pub deleted: bool,
    pub id: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub(crate) struct CreateVideoJsonRequest<'a> {
    pub prompt: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<&'a VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<VideoSeconds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<VideoSize>,
}

impl<'a> From<&'a CreateVideoRequest> for CreateVideoJsonRequest<'a> {
    fn from(value: &'a CreateVideoRequest) -> Self {
        Self {
            prompt: value.prompt.as_str(),
            model: value.model.as_ref(),
            seconds: value.seconds,
            size: value.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::AsyncTryFrom;
    use bytes::Bytes;
    use serde_json::json;

    #[test]
    fn list_videos_request_serializes() {
        let request = ListVideosRequestArgs::default()
            .limit(10_u32)
            .order(VideoOrder::Asc)
            .after("video_123")
            .build()
            .unwrap();

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 10);
        assert_eq!(json["order"], "asc");
        assert_eq!(json["after"], "video_123");
    }

    #[test]
    fn create_video_request_builder_defaults() {
        let request = CreateVideoRequestArgs::default()
            .prompt("Cats playing piano")
            .build()
            .unwrap();

        assert_eq!(request.prompt, "Cats playing piano");
        assert!(request.model.is_none());
        assert!(request.input_reference.is_none());
        assert!(request.seconds.is_none());
        assert!(request.size.is_none());

        let json = serde_json::to_value(CreateVideoJsonRequest::from(&request)).unwrap();
        assert_eq!(json["prompt"], "Cats playing piano");
        assert!(json.get("model").is_none());
        assert!(json.get("seconds").is_none());
        assert!(json.get("size").is_none());
    }

    #[tokio::test]
    async fn create_video_request_multipart_supports_binary_reference() {
        let bytes = Bytes::from_static(b"stub-image");
        let request = CreateVideoRequestArgs::default()
            .prompt("A scenic landscape")
            .input_reference(VideoInput::from_bytes("reference.png".into(), bytes))
            .model(VideoModel::Sora2Pro)
            .seconds(VideoSeconds::S12)
            .size(VideoSize::S1280x720)
            .build()
            .unwrap();

        let _form = <reqwest::multipart::Form as AsyncTryFrom<_>>::try_from(request)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn retrieve_video_content_params_serializes() {
        let params = RetrieveVideoContentParamsArgs::default()
            .variant(VideoContentVariant::Thumbnail)
            .build()
            .unwrap();
        let as_value = serde_json::to_value(&params).unwrap();
        assert_eq!(as_value, json!({ "variant": "thumbnail" }));
    }
}

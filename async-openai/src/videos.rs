use bytes::Bytes;
use serde::Serialize;

use crate::{
    config::Config,
    error::OpenAIError,
    types::{
        CreateVideoJsonRequest, CreateVideoRemixRequest, CreateVideoRequest, DeletedVideoResource,
        RetrieveVideoContentParams, VideoListResource, VideoResource,
    },
    Client,
};

/// Generate videos with OpenAI's video models.
pub struct Videos<'c, C: Config> {
    client: &'c Client<C>,
}

impl<'c, C: Config> Videos<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self { client }
    }

    /// Create a video generation job.
    pub async fn create(&self, request: CreateVideoRequest) -> Result<VideoResource, OpenAIError> {
        if request.input_reference.is_some() {
            self.client.post_form("/videos", request).await
        } else {
            let json_request = CreateVideoJsonRequest::from(&request);
            self.client.post("/videos", json_request).await
        }
    }

    /// List video generation jobs.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn list<Q>(&self, query: &Q) -> Result<VideoListResource, OpenAIError>
    where
        Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/videos", &query).await
    }

    /// Retrieve a video generation job.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, video_id: &str) -> Result<VideoResource, OpenAIError> {
        self.client
            .get(format!("/videos/{video_id}").as_str())
            .await
    }

    /// Delete a video generation job.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, video_id: &str) -> Result<DeletedVideoResource, OpenAIError> {
        self.client
            .delete(format!("/videos/{video_id}").as_str())
            .await
    }

    /// Download the rendered video content or alternate assets.
    pub async fn download_content(
        &self,
        video_id: &str,
        query: &RetrieveVideoContentParams,
    ) -> Result<Bytes, OpenAIError> {
        self.client
            .get_raw_with_query(format!("/videos/{video_id}/content").as_str(), query)
            .await
    }

    /// Create a remix of a completed video job.
    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn remix(
        &self,
        video_id: &str,
        request: CreateVideoRemixRequest,
    ) -> Result<VideoResource, OpenAIError> {
        self.client
            .post(format!("/videos/{video_id}/remix").as_str(), request)
            .await
    }
}

use crate::{Error, Result};
use reqwest::{header, RequestBuilder, StatusCode};
use std::time::Duration;

pub struct RequestSender {
    request_builder: RequestBuilder,
}

impl RequestSender {
    fn cloned_request_builder(&self) -> Result<RequestBuilder> {
        let error = Error::internal("Failed to clone request builder");

        self.request_builder.try_clone().ok_or(error)
    }

    pub fn new(request_builder: RequestBuilder) -> Self {
        Self { request_builder }
    }

    pub async fn send(self) -> Result<reqwest::Response> {
        let request_builder = self.cloned_request_builder()?;
        let response = request_builder.send().await?;

        let StatusCode::TOO_MANY_REQUESTS = response.status() else {
            return Ok(response);
        };

        let duration = retry_after(response.headers())?;
        sleep(duration).await;

        let response = self.request_builder.send().await?;
        Ok(response)
    }
}

fn retry_after(headers: &header::HeaderMap) -> Result<Duration> {
    let Some(value) = headers.get(header::RETRY_AFTER) else {
        return Err(Error::internal("Missing Retry-After header"));
    };

    let seconds = value
        .to_str()
        .map_err(Error::unexpected)?
        .parse::<u64>()
        .map_err(Error::unexpected)?;

    Ok(Duration::from_secs(seconds))
}

async fn sleep(duration: Duration) {
    tracing::info!(
        "Too many requests. Retrying in {} seconds",
        duration.as_secs()
    );

    tokio::time::sleep(duration).await;
}

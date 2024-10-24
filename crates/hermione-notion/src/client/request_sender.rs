use reqwest::{header, RequestBuilder, StatusCode};
use std::time::Duration;

pub struct RequestSender {
    request_builder: RequestBuilder,
}

impl RequestSender {
    fn cloned_request_builder(&self) -> eyre::Result<RequestBuilder> {
        self.request_builder
            .try_clone()
            .ok_or(eyre::Error::msg("Failed to clone request builder"))
    }

    pub fn new(request_builder: RequestBuilder) -> Self {
        Self { request_builder }
    }

    pub async fn send(self) -> eyre::Result<reqwest::Response> {
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

fn retry_after(headers: &header::HeaderMap) -> eyre::Result<Duration> {
    let Some(value) = headers.get(header::RETRY_AFTER) else {
        return Err(eyre::Error::msg("Missing Retry-After header"));
    };

    let seconds = value.to_str()?.parse::<u64>()?;

    Ok(Duration::from_secs(seconds))
}

async fn sleep(duration: Duration) {
    tracing::info!(
        "Too many requests. Retrying in {} seconds",
        duration.as_secs()
    );

    tokio::time::sleep(duration).await;
}

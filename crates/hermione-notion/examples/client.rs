use anyhow::Result;
use hermione_notion::{json::Json, Client, Method, NewClientParameters, SendParameters};
use serde::Deserialize;
use std::{
    fs::{self, File},
    time::Duration,
};

#[derive(Deserialize)]
struct Settings {
    api_key: String,
    timeout_secs: u64,
    base_url_override: Option<String>,
    uri: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let settings_path = format!("{dir}/settings.json");
    let output_path = format!("{dir}/output.json");
    let body = File::open(format!("{dir}/body.json"))?;

    let file = fs::File::open(settings_path)?;

    let Settings {
        api_key,
        timeout_secs,
        uri,
        base_url_override,
    } = serde_json::from_reader(file)?;

    let client = Client::new(NewClientParameters {
        timeout: Duration::from_secs(timeout_secs),
        api_key: Some(api_key),
        base_url_override,
    })?;

    let output = post(client, &uri, body).await?;

    write_output(&output_path, &output)?;

    Ok(())
}

async fn post(client: Client, uri: &str, file: File) -> Result<Json> {
    let body: Json = serde_json::from_reader(&file)?;

    let parameters = SendParameters {
        api_key_override: None,
        body: Some(body),
        uri: &uri,
        method: Method::post(),
    };

    let output = client.send(parameters).await?;

    Ok(output)
}

fn write_output(path: &str, value: &Json) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer_pretty(&mut file, value)?;

    Ok(())
}

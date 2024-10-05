use anyhow::Result;
use hermione_notion::{Client, ClientParameters, PostParameters, QueryDatabaseParameters};
use serde::Deserialize;
use serde_json::Value;
use std::{fs, time::Duration};

#[derive(Deserialize)]
struct Settings {
    api_key: String,
    timeout_secs: u64,
    base_url_override: Option<String>,
    action: Action,
}

#[derive(Deserialize)]
enum Action {
    #[serde(rename = "query_database")]
    QueryDatabase(QueryDatabaseAction),

    #[serde(rename = "post")]
    Post(PostAction),
}

#[derive(Deserialize)]
struct QueryDatabaseAction {
    database_id: String,
    page_size: u8,
    start_cursor: Option<String>,
}

#[derive(Deserialize)]
struct PostAction {
    uri: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let settings_path = format!("{dir}/settings.json");
    let output_path = format!("{dir}/output.json");

    println!("Reading settings from {settings_path}");

    let file = fs::File::open(settings_path)?;

    let Settings {
        api_key,
        timeout_secs,
        action,
        base_url_override,
    } = serde_json::from_reader(file)?;

    let client = Client::new(ClientParameters {
        timeout: Duration::from_secs(timeout_secs),
        api_key: Some(api_key),
        base_url_override,
    })?;

    let output = match action {
        Action::QueryDatabase(action) => query_database(client, action).await?,
        Action::Post(action) => post(client, action).await?,
    };

    write_output(&output_path, &output)?;

    Ok(())
}

async fn query_database(client: Client, action: QueryDatabaseAction) -> Result<Value> {
    let QueryDatabaseAction {
        database_id,
        page_size,
        start_cursor,
    } = action;

    let parameters = QueryDatabaseParameters {
        api_key_override: None,
        page_size,
        start_cursor: start_cursor.as_deref(),
        database_id: &database_id,
    };

    let output = client.query_database(parameters).await?;

    Ok(output)
}

async fn post(client: Client, action: PostAction) -> Result<Value> {
    let PostAction { uri } = action;

    let file = fs::File::open("body.json")?;
    let body: Value = serde_json::from_reader(file)?;

    let parameters = PostParameters {
        api_key_override: None,
        body: Some(body),
        uri: &uri,
    };

    let output = client.post(parameters).await?;

    Ok(output)
}

fn write_output(path: &str, value: &Value) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer_pretty(&mut file, value)?;

    Ok(())
}

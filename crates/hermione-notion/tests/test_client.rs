use hermione_notion::{
    Client, Method, NewClientParameters, QueryDatabaseParameters, SendParameters,
};
use hermione_notion_serde::de;
use httpmock::prelude::*;
use serde::Deserialize;

type Result<T> = eyre::Result<T>;

#[derive(Deserialize)]
struct Product {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    name: String,

    #[serde(
        rename(deserialize = "Description"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    description: String,
}

#[tokio::test]
async fn it_makes_custom_post_request() -> Result<()> {
    let mock_server = MockServer::start_async().await;
    let base_url = mock_server.base_url();

    let mock = mock_server.mock(|when, then| {
        when.path("/custom/path").method(POST);
        then.body_from_file("tests/fixtures/database_query.json")
            .status(200);
    });

    let client = Client::new(NewClientParameters {
        base_url_override: Some(base_url.clone()),
        api_key: Some("".to_string()),
        ..Default::default()
    })?;

    let parameters = SendParameters {
        api_key_override: None,
        body: None,
        uri: "/custom/path",
        method: Method::post(),
    };

    client.send(parameters).await?;

    mock.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn it_queries_database() -> Result<()> {
    let mock_server = MockServer::start_async().await;
    let base_url = mock_server.base_url();

    let mock = mock_server.mock(|when, then| {
        when.path("/databases/1111/query").method(POST);
        then.body_from_file("tests/fixtures/database_query.json")
            .status(200);
    });

    let client = Client::new(NewClientParameters {
        base_url_override: Some(base_url.clone()),
        api_key: Some("".to_string()),
        ..Default::default()
    })?;

    let parameters = QueryDatabaseParameters::default();
    client.query_database::<Product>("1111", parameters).await?;

    mock.assert_async().await;

    Ok(())
}

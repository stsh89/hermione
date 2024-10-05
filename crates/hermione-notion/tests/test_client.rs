use hermione_notion::{Client, ClientParameters, PostParameters, QueryDatabaseParameters};
use httpmock::prelude::*;

type Result<T> = eyre::Result<T>;

#[tokio::test]
async fn it_makes_custom_post_request() -> Result<()> {
    let mock_server = MockServer::start_async().await;
    let base_url = mock_server.base_url();

    let mock = mock_server.mock(|when, then| {
        when.path("/custom/path").method(POST);
        then.body_from_file("tests/fixtures/database_query.json")
            .status(200);
    });

    let client = Client::new(ClientParameters {
        base_url_override: Some(base_url.clone()),
        api_key: Some("".to_string()),
        ..Default::default()
    })?;
    let parameters = PostParameters {
        api_key_override: None,
        body: None,
        uri: "/custom/path",
    };

    client.post(parameters).await?;

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

    let client = Client::new(ClientParameters {
        base_url_override: Some(base_url.clone()),
        api_key: Some("".to_string()),
        ..Default::default()
    })?;

    let parameters = QueryDatabaseParameters {
        api_key_override: None,
        page_size: 100,
        start_cursor: None,
        database_id: "1111",
    };

    client.query_database(parameters).await?;

    mock.assert_async().await;

    Ok(())
}

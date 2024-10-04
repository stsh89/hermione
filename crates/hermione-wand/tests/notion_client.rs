use hermione_wand::notion::{Client, NewClientParameters, QueryDatabaseParameters};
use httpmock::prelude::*;

type Result<T> = eyre::Result<T>;

#[tokio::test]
async fn it_queries_database() -> Result<()> {
    let mock_server = MockServer::start_async().await;

    let mock = mock_server.mock(|when, then| {
        when.path("/databases/1111/query").method(POST);
        then.body_from_file("tests/fixtures/database_query.json")
            .status(200);
    });

    let new_client_parameters =
        NewClientParameters::default().with_base_url(&mock_server.base_url());
    let client = Client::new(new_client_parameters)?;

    let response = client
        .query_database(QueryDatabaseParameters::default().with_database_id("1111"))
        .await?;

    assert_eq!(response.next_cursor, None);
    assert_eq!(response.records.len(), 1);

    mock.assert_async().await;

    Ok(())
}

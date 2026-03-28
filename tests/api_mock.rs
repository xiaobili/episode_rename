use mockito::Server;
use openlist_tui::api::client::OpenListClient;

#[tokio::test]
async fn test_batch_rename() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/api/fs/batch_rename")
        .with_status(200)
        .with_body(r#"{"code":200}"#)
        .create();

    let client = OpenListClient::new(server.url(), None);
    assert!(client.batch_rename("/test", vec![]).await.is_ok());
    mock.assert();
}

#[tokio::test]
async fn test_empty_directory() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/api/fs/list")
        .with_status(200)
        .with_body(r#"{"code":200,"data":{"content":[]}}"#)
        .create();

    let client = OpenListClient::new(server.url(), None);
    let result = client.list_directory("/empty").await.unwrap();
    assert!(result.is_empty());
    mock.assert();
}

#[tokio::test]
async fn test_network_error() {
    let mut server = Server::new_async().await;
    let _mock = server
        .mock("POST", "/api/fs/list")
        .with_status(408)
        .create();

    let client = OpenListClient::new(server.url(), None);
    assert!(client.list_directory("/test").await.is_err());
}

#[tokio::test]
async fn test_login_success() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/api/auth/login")
        .with_status(200)
        .with_body(r#"{"code":200,"data":{"token":"test_token_123"}}"#)
        .create();

    let client = OpenListClient::new(server.url(), None);
    let result = client.login("admin", "password").await.unwrap();
    assert_eq!(result, "test_token_123");
    mock.assert();
}

#[tokio::test]
async fn test_rename_single() {
    let mut server = Server::new_async().await;
    let mock = server
        .mock("POST", "/api/fs/rename")
        .with_status(200)
        .with_body(r#"{"code":200}"#)
        .create();

    let client = OpenListClient::new(server.url(), None);
    assert!(client.rename_single("/test/file.mkv", "new_name.mkv").await.is_ok());
    mock.assert();
}

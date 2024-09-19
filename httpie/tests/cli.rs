use assert_cmd::Command;
use predicates::prelude::*;
use std::process::Command as StdCommand;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_request() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Hello, world!"))
        .mount(&mock_server)
        .await;

    let mut cmd = Command::cargo_bin("httpie").unwrap();
    cmd.arg("get")
        .arg(mock_server.uri())
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));
}

#[tokio::test]
async fn test_post_request() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Posted successfully"))
        .mount(&mock_server)
        .await;

    let mut cmd = Command::cargo_bin("httpie").unwrap();
    cmd.arg("post")
        .arg(mock_server.uri())
        .arg("key=value")
        .assert()
        .success()
        .stdout(predicate::str::contains("Posted successfully"));
}

#[test]
fn test_invalid_url() {
    let mut cmd = Command::cargo_bin("httpie").unwrap();
    cmd.arg("get")
        .arg("not-a-url")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid value 'not-a-url' for '<URL>'",
        ));
}

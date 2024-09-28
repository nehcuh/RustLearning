use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:6379").unwrap();
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=huchen").await?.print().await?;
    hc.do_get("/hello2/bullinbenniu").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "password": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "TicketAAA"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "TicketBBB"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/api/tickets").await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;
    Ok(())
}

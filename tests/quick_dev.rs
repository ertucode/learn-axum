#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/mike").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

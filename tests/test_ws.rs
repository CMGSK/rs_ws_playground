use anyhow::Result;
use serde_json::json;

//Listen and test on compile: cargo watch -q -c -w tests/ -x "test -q test_fn_name -- --nocapture"

#[tokio::test]
async fn test_ws() -> Result<()> {

    let hc = httpc_test::new_client("http://localhost:8000")?;
    hc.do_get("/hello/HatsuneMiku!!!/").await?.print().await?;

    hc.do_get("/tests/test.txt").await?.print().await?;

    let req = hc.do_post("/api/login/", 
        json!({
            "username": "miku",
            "pwd": "beam"
        }));
    req.await?.print().await?;

    Ok(())
}
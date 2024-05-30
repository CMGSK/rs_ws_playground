use anyhow::Result;

//Listen and test on compile: cargo watch -q -c -w tests/ -x "test -q test_fn_name -- --nocapture"

#[tokio::test]
async fn test_ws() -> Result<()> {

    let hc = httpc_test::new_client("http://localhost:8000")?;
    hc.do_get("/?name=Miku!!!!!").await?.print().await?;
    Ok(())
}
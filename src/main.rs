use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;
    let now = Instant::now();

    let resp = reqwest::get("https://testnet.binancefuture.com/fapi/v1/depth?symbol=btcusdt")
        .await?
        .text()
        .await?;

        let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:#?}", resp);
    Ok(())
}
#[tokio::main]
async fn main() -> hyper::Result<()> {
    bitflips::run().await?;
    Ok(())
}

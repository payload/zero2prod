#[tokio::main]
async fn main() -> hyper::Result<()> {
    let (listener, addr) = bitflips::bind_localhost("3000");
    println!("Listening on {addr}");
    bitflips::run(listener).await?;
    Ok(())
}

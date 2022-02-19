use eyre::Result;
use tinkc::Tink;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Tink::new(
        "http://[::1]:42113",
        "tinkc/examples/data/tls/ca.pem",
        "localhost",
    )
    .await?;
    let workflows: Vec<tinkc::Workflow> = client.workflows().await?;
    for workflow in workflows.iter() {
        println!("{:?}", workflow);
    }
    Ok(())
}

use eyre::Result;
use tinkc::{Tink, TinkCert, TinkConfigBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Tink::new(
        TinkConfigBuilder::default()
            .endpoint("http://[::1]:42113")
            .cert(TinkCert::File("tinkc/examples/data/tls/ca.pem"))
            .domain("localhost")
            .build()?,
    )
    .await?;
    let workflows: Vec<tinkc::Workflow> = client.workflows().await?;
    for workflow in workflows.iter() {
        println!("{:?}", workflow);
    }
    Ok(())
}

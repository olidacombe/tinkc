use derive_builder::Builder;
use eyre::Result;
pub mod grpc;
pub use grpc::hardware::{self, Hardware, HardwareServiceClient};
pub use grpc::workflow::{self, GetRequest, Workflow, WorkflowActionStatus, WorkflowServiceClient};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

#[derive(Clone)]
pub struct Tink {
    workflow_client: WorkflowServiceClient<Channel>,
    hardware_client: HardwareServiceClient<Channel>,
}

#[derive(Clone)]
pub enum TinkCert<'a> {
    File(&'a str),
    Str(&'a str),
    Vec(Vec<u8>),
}

#[derive(Builder)]
pub struct TinkConfig<'a> {
    endpoint: &'a str,
    domain: &'a str,
    cert: TinkCert<'a>,
}

impl Tink {
    pub async fn new(config: TinkConfig<'_>) -> Result<Self> {
        let pem = match config.cert {
            TinkCert::File(cert) => tokio::fs::read(cert).await?,
            TinkCert::Str(cert) => cert.as_bytes().to_vec(),
            TinkCert::Vec(cert) => cert,
        };
        let ca = Certificate::from_pem(pem);

        let tls = ClientTlsConfig::new()
            .ca_certificate(ca)
            .domain_name(config.domain);

        let channel = Channel::from_shared(config.endpoint.to_owned())?
            .tls_config(tls)?
            .connect()
            .await?;

        Ok(Self {
            workflow_client: WorkflowServiceClient::new(channel.clone()),
            hardware_client: HardwareServiceClient::new(channel),
        })
    }

    pub async fn workflows<T>(&mut self) -> Result<Vec<T>>
    where
        T: TryFrom<Workflow>,
    {
        let mut workflows = self
            .workflow_client
            .list_workflows(workflow::Empty {})
            .await?
            .into_inner();
        let mut resolved = Vec::<T>::new();
        while let Some(workflow) = workflows.message().await? {
            if let Ok(workflow) = workflow.try_into() {
                resolved.push(workflow);
            }
        }
        Ok(resolved)
    }

    pub async fn workflow_events<T>(&mut self, id: String) -> Result<Vec<T>>
    where
        T: TryFrom<WorkflowActionStatus>,
    {
        let mut stats = self
            .workflow_client
            .show_workflow_events(GetRequest { id })
            .await?
            .into_inner();
        let mut resolved = Vec::<T>::new();
        while let Some(status) = stats.message().await? {
            if let Ok(status) = status.try_into() {
                resolved.push(status)
            }
        }
        Ok(resolved)
    }

    pub async fn hardware<T>(&mut self) -> Result<Vec<T>>
    where
        T: TryFrom<Hardware>,
    {
        let mut hardware = self
            .hardware_client
            .all(hardware::Empty {})
            .await?
            .into_inner();
        let mut resolved = Vec::<T>::new();
        while let Some(hardware) = hardware.message().await? {
            if let Ok(hardware) = hardware.try_into() {
                resolved.push(hardware);
            }
        }
        Ok(resolved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn happy_tink_config_builder_file() -> Result<()> {
        TinkConfigBuilder::default()
            .cert(TinkCert::File("ca.crt"))
            .domain("example.com")
            .endpoint("localhost:9999")
            .build()?;
        Ok(())
    }

    #[test]
    fn happy_tink_config_builder_string() -> Result<()> {
        TinkConfigBuilder::default()
            .cert(TinkCert::Str("some content"))
            .domain("example.com")
            .endpoint("localhost:9999")
            .build()?;
        Ok(())
    }
}

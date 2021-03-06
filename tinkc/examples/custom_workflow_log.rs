use eyre::Result;
use serde::Deserialize;
use thiserror::Error;
use tinkc::{Tink, TinkCert, TinkConfigBuilder};

#[derive(Debug, Deserialize)]
struct Hardware {
    device: String,
    image: String,
}

#[derive(Debug)]
enum WorkflowState {
    Pending,
    Running,
    Failed,
    Timeout,
    Success,
}

#[derive(Error, Debug)]
enum WorkflowError {
    #[error("Invalid workflow state {0}")]
    InvalidWorkflowState(i32),
}

impl TryFrom<i32> for WorkflowState {
    type Error = WorkflowError;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        match i {
            0 => Ok(WorkflowState::Pending),
            1 => Ok(WorkflowState::Running),
            2 => Ok(WorkflowState::Failed),
            3 => Ok(WorkflowState::Timeout),
            4 => Ok(WorkflowState::Success),
            e => Err(WorkflowError::InvalidWorkflowState(e)),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    id: String,
    hardware: Hardware,
    state: WorkflowState,
}

impl TryFrom<tinkc::Workflow> for Workflow {
    type Error = eyre::Error;
    fn try_from(wf: tinkc::Workflow) -> std::result::Result<Workflow, Self::Error> {
        Ok(Workflow {
            state: wf.state.try_into()?,
            hardware: serde_json::from_str(&wf.hardware)?,
            id: wf.id,
        })
    }
}

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
    let workflows: Vec<Workflow> = client.workflows().await?;
    for workflow in workflows.iter() {
        println!("{:?}", workflow);
    }
    Ok(())
}

use chrono::NaiveDateTime;
use juniper_codegen::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, GraphQLObject)]
pub struct WorkflowHardware {
    device: String,
    image: String,
}

#[derive(Debug, GraphQLEnum)]
pub enum State {
    Pending,
    Running,
    Failed,
    Timeout,
    Success,
}

#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Invalid workflow state {0}")]
    InvalidWorkflowState(i32),
    #[error("Missing created_at")]
    MissingCreatedAt,
}

impl TryFrom<i32> for State {
    type Error = WorkflowError;

    fn try_from(i: i32) -> std::result::Result<Self, Self::Error> {
        match i {
            0 => Ok(State::Pending),
            1 => Ok(State::Running),
            2 => Ok(State::Failed),
            3 => Ok(State::Timeout),
            4 => Ok(State::Success),
            e => Err(WorkflowError::InvalidWorkflowState(e)),
        }
    }
}

#[derive(Debug, GraphQLObject)]
pub struct Workflow {
    id: String,
    hardware: WorkflowHardware,
    state: State,
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

#[derive(Debug, GraphQLObject)]
pub struct WorkflowActionStatus {
    task_name: String,
    action_name: String,
    action_status: State,
    seconds: i32,
    created_at: NaiveDateTime,
    message: String,
    worker_id: String,
}

impl TryFrom<tinkc::WorkflowActionStatus> for WorkflowActionStatus {
    type Error = eyre::Error;
    fn try_from(
        status: tinkc::WorkflowActionStatus,
    ) -> std::result::Result<WorkflowActionStatus, Self::Error> {
        let created_at = status.created_at.ok_or(WorkflowError::MissingCreatedAt)?;
        let created_at =
            NaiveDateTime::from_timestamp(created_at.seconds, created_at.nanos.try_into()?);
        Ok(WorkflowActionStatus {
            task_name: status.task_name,
            action_name: status.action_name,
            action_status: status.action_status.try_into()?,
            seconds: status.seconds.try_into()?,
            created_at,
            message: status.message,
            worker_id: status.worker_id,
        })
    }
}

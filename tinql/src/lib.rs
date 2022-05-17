use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use std::sync::Arc;
use tinkc::Tink;

mod hardware;
mod workflow;

use hardware::Hardware;
use workflow::{Workflow, WorkflowActionStatus};

#[derive(Clone)]
pub struct Context {
    tink: Arc<Tink>,
}

impl Context {
    pub fn new(tink: Tink) -> Self {
        Self {
            tink: Arc::new(tink),
        }
    }
}

impl juniper::Context for Context {}
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn workflows<'ctx>(&self, context: &'ctx Context) -> FieldResult<Vec<Workflow>> {
        let mut tink = context.tink.as_ref().clone();
        Ok(tink.workflows().await?)
    }

    async fn workflow_events<'ctx>(
        &self,
        context: &'ctx Context,
        id: String,
    ) -> FieldResult<Vec<WorkflowActionStatus>> {
        let mut tink = context.tink.as_ref().clone();
        Ok(tink.workflow_events(id).await?)
    }

    async fn hardware<'ctx>(&self, context: &'ctx Context) -> FieldResult<Vec<Hardware>> {
        let mut tink = context.tink.as_ref().clone();
        Ok(tink.hardware().await?)
    }

    async fn hardware_from_mac<'ctx>(
        &self,
        context: &'ctx Context,
        mac: String,
    ) -> FieldResult<Hardware> {
        let mut tink = context.tink.as_ref().clone();
        Ok(tink.hardware_from_mac(mac).await?)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use crate::domain::verification::entity::step::{
    StepInVerificationPlan, VerificationStepsCollection,
};
use async_trait::async_trait;

#[async_trait]
pub trait ContainerAPIClient {
    type R;
    type P;

    async fn inspect_container_status(&self, project_step: &StepInVerificationPlan) -> Self::R;
    async fn remove_existing_container(&self, project_step: &StepInVerificationPlan) -> Self::P;
    async fn start_container(&self, project_step: &StepInVerificationPlan) -> Self::P;
    async fn tail_container_logs(&self, project_step: &StepInVerificationPlan) -> Self::R;
}

pub struct VerificationRuntime<'a, T> {
    pub container_api_client: T,
    pub step_in_verification_plan: StepInVerificationPlan<'a>,
    pub verification_step_collection: VerificationStepsCollection<'a>,
}

#[async_trait]
pub trait VerificationStepRunner<R> {
    fn steps_names() -> Vec<&'static str>;
    async fn get_progress(&self) -> R;
    async fn get_report(&self) -> R;
    async fn start_running(&self) -> R;
}

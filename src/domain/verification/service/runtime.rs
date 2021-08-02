use crate::domain::verification::entity::verification_steps_collection::VerificationStepsCollection;
use async_trait::async_trait;

pub struct VerificationRuntime<'a, T> {
    pub container_api_client: T,
    pub target_hash: String,
    pub verification_step_collection: VerificationStepsCollection<'a>,
}

#[async_trait]
pub trait LLVMBitcodeGenerator<R, S> {
    async fn start_running_step(&self, step_name: String) -> R;
    async fn tail_logs_for_step(&self, step_name: String) -> S;
    async fn get_progress_for_step(&self, step_name: String) -> S;
}

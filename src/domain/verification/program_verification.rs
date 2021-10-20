use async_trait::async_trait;

#[async_trait]
pub trait ProgramVerification {
    type A;
    type R;

    fn new(target: Self::A) -> Self;

    async fn run_step(&self) -> Self::R;
    async fn step_report(&self) -> Self::R;
    async fn step_progress(&self) -> Self::R;
}

#[derive(Copy, Clone)]
pub struct VerificationTarget<'a> {
    pub step: &'a str,
    pub project_id: &'a str,
}

pub struct SmartContractVerification<'a> {
    pub target: VerificationTarget<'a>,
}

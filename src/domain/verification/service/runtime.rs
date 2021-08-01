use async_trait::async_trait;

pub struct VerificationRuntime<T> {
    pub container_api_client: T,
    pub target_hash: String,
}

#[async_trait]
pub trait LLVMBitcodeGenerator<R> {
    async fn start_llvm_bitcode_generation(&self) -> R;
}

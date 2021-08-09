
export default class VerificationEvents {
    static resetVerificationRuntime = 'verification-steps.verification-runtime'
    static failedVerificationStep = 'verification-steps.failure'
    static llvmBitcodeGeneration = 'llvm-bitcode-generation'
    static symbolicExecution = 'symbolic-execution'
}

export class AppEvents {
    static clearHistoryRequested = 'clear-history.requested'
}

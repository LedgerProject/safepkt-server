
export default class VerificationEvents {
    static resetVerificationRuntime = 'verification-steps.verification-runtime'
    static failedVerificationStep = 'verification-steps.failure'
    static llvmBitcodeGeneration = 'llvm-bitcode-generation'
    static symbolicExecution = 'symbolic-execution'
    static sourceRestoration = 'source-restoration'
    static programVerification = 'program-verification'
}

export class AppEvents {
    static clearHistoryRequested = 'clear-history.requested'
    static showEditorRequested = 'show-editor.requested'
    static symbolicExecutionRequested = 'symbolic-execution.requested'
    static programVerificationRequested = 'program-verification.requested'
    static sourceRestorationRequested = 'source-restoration.requested'
}

import {Project} from "~/types/project";

type LLVMBitcodeGenerationStepProgress = 'llvmBitcodeGenerationStepProgress'
type LLVMBitcodeGenerationStepReport = 'llvmBitcodeGenerationStepReport'
type SymbolicExecutionStepProgress = 'symbolicExecutionStepProgress'
type SymbolicExecutionStepReport = 'symbolicExecutionStepReport'

type VerificationStepPollingTarget =
    LLVMBitcodeGenerationStepProgress|
    LLVMBitcodeGenerationStepReport|
    SymbolicExecutionStepProgress|
    SymbolicExecutionStepReport

type UploadSourceStep = 'uploadSource'
type LLVMBitCodeGenerationStep = 'llvmBitcodeGeneration'
type SymbolicExecutionStep = 'symbolicExecution'

type VerificationStep =
    UploadSourceStep|
    LLVMBitCodeGenerationStep|
    SymbolicExecutionStep

type VerificationStepAssertion = (project: Project, pollingTarget: VerificationStepPollingTarget) => boolean

export {
    VerificationStep,
    VerificationStepAssertion,
    VerificationStepPollingTarget,
}
import {Project} from "~/types/project";

type LLVMBitcodeGenerationStepProgress = 'llvmBitcodeGenerationStepProgress'
type LLVMBitcodeGenerationStepReport = 'llvmBitcodeGenerationStepReport'
type SymbolicExecutionStepProgress = 'symbolicExecutionStepProgress'
type SymbolicExecutionStepReport = 'symbolicExecutionStepReport'
type SourceRestorationStepProgress = 'sourceRestorationStepProgress'
type SourceRestorationStepReport = 'sourceRestorationStepReport'

type VerificationStepPollingTarget =
    LLVMBitcodeGenerationStepProgress|
    LLVMBitcodeGenerationStepReport|
    SymbolicExecutionStepProgress|
    SymbolicExecutionStepReport|
    SourceRestorationStepProgress|
    SourceRestorationStepReport

type UploadSourceStep = 'uploadSource'
type LLVMBitCodeGenerationStep = 'llvmBitcodeGeneration'
type SymbolicExecutionStep = 'symbolicExecution'
type SourceRestorationStep = 'sourceRestoration'

type VerificationStep =
    UploadSourceStep|
    LLVMBitCodeGenerationStep|
    SymbolicExecutionStep|
    SourceRestorationStep

type VerificationStepAssertion = (project: Project, pollingTarget: VerificationStepPollingTarget) => boolean

export {
    VerificationStep,
    VerificationStepAssertion,
    VerificationStepPollingTarget,
}

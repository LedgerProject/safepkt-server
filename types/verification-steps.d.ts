import {Project} from "~/types/project";

type LLVMBitcodeGenerationStepProgress = 'llvmBitcodeGenerationStepProgress'
type LLVMBitcodeGenerationStepReport = 'llvmBitcodeGenerationStepReport'
type SymbolicExecutionStepProgress = 'symbolicExecutionStepProgress'
type SymbolicExecutionStepReport = 'symbolicExecutionStepReport'
type SourceRestorationStepProgress = 'sourceRestorationStepProgress'
type SourceRestorationStepReport = 'sourceRestorationStepReport'
type ProgramVerificationStepProgress = 'programVerificationStepProgress'
type ProgramVerificationStepReport = 'programVerificationStepReport'

type VerificationStepPollingTarget =
    LLVMBitcodeGenerationStepProgress|
    LLVMBitcodeGenerationStepReport|
    SymbolicExecutionStepProgress|
    SymbolicExecutionStepReport|
    SourceRestorationStepProgress|
    SourceRestorationStepReport|
    ProgramVerificationStepProgress|
    ProgramVerificationStepReport

type UploadSourceStep = 'uploadSource'
type LLVMBitCodeGenerationStep = 'llvmBitcodeGeneration'
type SymbolicExecutionStep = 'symbolicExecution'
type SourceRestorationStep = 'sourceRestoration'
type ProgramVerificationStep = 'programVerification'

type VerificationStep =
    UploadSourceStep|
    LLVMBitCodeGenerationStep|
    SymbolicExecutionStep|
    SourceRestorationStep|
    ProgramVerificationStep

type VerificationStepAssertion = (project: Project, pollingTarget: VerificationStepPollingTarget) => boolean

export {
    VerificationStep,
    VerificationStepAssertion,
    VerificationStepPollingTarget,
}

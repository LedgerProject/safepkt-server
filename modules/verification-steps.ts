
import {
  VerificationStepPollingTarget,
  VerificationStep as Step
} from '~/types/verification-steps'

export class VerificationStepProgress {
  static running = 'running'
  static completed = 'exited'
}

export class PollingTarget {
  static LLVMBitcodeGenerationStepProgress: VerificationStepPollingTarget = 'llvmBitcodeGenerationStepProgress'
  static LLVMBitcodeGenerationStepReport: VerificationStepPollingTarget = 'llvmBitcodeGenerationStepReport'
  static SymbolicExecutionStepProgress: VerificationStepPollingTarget = 'symbolicExecutionStepProgress'
  static SourceRestorationStepProgress: VerificationStepPollingTarget = 'sourceRestorationStepProgress'
  static SourceRestorationStepReport: VerificationStepPollingTarget = 'sourceRestorationStepReport'
  static ProgramVerificationStepProgress: VerificationStepPollingTarget = 'programVerificationStepProgress'
  static ProgramVerificationStepReport: VerificationStepPollingTarget = 'programVerificationStepReport'
}

export class VerificationStep {
  static uploadSourceStep: Step = 'uploadSource'
  static llvmBitcodeGenerationStep: Step = 'llvmBitcodeGeneration'
  static symbolicExecutionStep: Step = 'symbolicExecution'
  static sourceRestorationStep: Step = 'sourceRestoration'
  static programVerificationStep: Step = 'programVerification'

  get uploadSourceStep () {
    return VerificationStep.uploadSourceStep
  }

  get llvmBitcodeGenerationStep () {
    return VerificationStep.llvmBitcodeGenerationStep
  }

  get symbolicExecutionStep () {
    return VerificationStep.symbolicExecutionStep
  }

  get sourceRestorationStep () {
    return VerificationStep.sourceRestorationStep
  }

  get programVerificationStep () {
    return VerificationStep.programVerificationStep
  }
}

class UnexpectedStep extends Error {}

export { UnexpectedStep }

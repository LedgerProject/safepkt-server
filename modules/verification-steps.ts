
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
}

export class VerificationStep {
  static uploadSourceStep: Step = 'uploadSource'
  static llvmBitcodeGenerationStep: Step = 'llvmBitcodeGeneration'
  static symbolicExecutionStep: Step = 'symbolicExecution'
  static sourceRestoration: Step = 'sourceRestoration'

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
    return VerificationStep.sourceRestoration
  }
}

class UnexpectedStep extends Error {}

export { UnexpectedStep }

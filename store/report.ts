import { Module, VuexModule } from 'vuex-module-decorators'
import { VerificationStep } from '~/modules/verification-steps'
import { VerificationStep as VerificationStepType } from '~/types/verification-steps'

@Module({
  name: 'report',
  stateFactory: true,
  namespaced: true
})
class ReportStore extends VuexModule {
  get reportTitle (): (step: VerificationStepType) => string {
    return (step: VerificationStepType) => {
      switch (true) {
        case step === VerificationStep.uploadSourceStep:
          return 'Paste program'

        case step === VerificationStep.llvmBitcodeGenerationStep:
          return 'Generate LLVM Bitcode'

        case step === VerificationStep.symbolicExecutionStep:
          return 'Run symbolic execution'

        case step === VerificationStep.programVerificationStep:
          return 'Verify program'

        default:
          return ''
      }
    }
  }
}

export default ReportStore

import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import { Project } from '~/types/project'
import { VerificationStep, VerificationStepAssertion, VerificationStepPollingTarget } from '~/types/verification-steps'
import EventBus from '~/modules/event-bus'
import {
  PollingTarget,
  UnexpectedStep,
  VerificationStepProgress as Progress,
  VerificationStep as Step
} from '~/modules/verification-steps'
import { ProjectNotFound } from '~/mixins/project'
import {
  GETTER_IS_REPORT_VISIBLE as isLlvmBitcodeGenerationReportVisible,
  MUTATION_HIDE_REPORT as hideLlvmBitcodeGenerationReport,
  MUTATION_SHOW_REPORT as showLlvmBitcodeGenerationReport
} from '~/store/step/llvm-bitcode-generation'
import {
  GETTER_IS_REPORT_VISIBLE as isSymbolicExecutionReportVisible,
  MUTATION_HIDE_REPORT as hideSymbolicExecutionReport,
  MUTATION_SHOW_REPORT as showSymbolicExecutionReport
} from '~/store/step/symbolic-execution'
import {
  GETTER_IS_REPORT_VISIBLE as isProgramVerificationReportVisible,
  MUTATION_HIDE_REPORT as hideProgramVerificationReport,
  MUTATION_SHOW_REPORT as showProgramVerificationReport
} from '~/store/step/program-verification'
import {
  GETTER_ACTIVE_PROJECT,
  MUTATION_PUSH_ERROR
} from '~/store/verification-runtime'
import VerificationEvents from '~/modules/events'

const MUTATION_SET_VERIFICATION_STEP = 'setVerificationStep'

export {
  MUTATION_SET_VERIFICATION_STEP
}

@Module({
  name: 'verification-steps',
  stateFactory: true,
  namespaced: true
})
export default class VerificationStepsStore extends VuexModule {
  lockedResetButton: boolean = true
  verificationStep: VerificationStep = Step.uploadSourceStep

  @Mutation
  [MUTATION_SET_VERIFICATION_STEP] (step: VerificationStep) {
    this.verificationStep = step
  }

  @Mutation
  lockResetButton (): void {
    this.lockedResetButton = true
  }

  @Mutation
  unlockResetButton (): void {
    this.lockedResetButton = false
  }

  @Action
  reportError ({ error }: {error: Error}): void {
    this.context.commit('unlockResetButton')
    this.context.commit(
      `verification-runtime/${MUTATION_PUSH_ERROR}`,
      { error },
      { root: true }
    )
  }

  get canRunVerificationStep (): (step: VerificationStep) => boolean {
    return (step: VerificationStep) => {
      let canDo = false

      if (step === Step.uploadSourceStep) {
        canDo = this.context.rootGetters['step/upload-source/canUploadSource']()
      }

      if (step === Step.llvmBitcodeGenerationStep) {
        canDo = this.context.rootGetters['step/llvm-bitcode-generation/canRunLlvmBitcodeGenerationStep']()
      }

      if (step === Step.symbolicExecutionStep) {
        canDo = this.context.rootGetters['step/symbolic-execution/canRunSymbolicExecutionStep']()
      }

      if (step === Step.programVerificationStep) {
        canDo = this.context.rootGetters['step/symbolic-execution/canRunProgramVerificationStep']()
      }

      return canDo
    }
  }

  get isResetButtonLocked (): boolean {
    return this.lockedResetButton
  }

  get isResetButtonUnlocked (): boolean {
    return !this.isResetButtonLocked
  }

  get canResetVerificationRuntime (): boolean {
    const noVerificationStepRemaining = !this.canRunVerificationStep(Step.uploadSourceStep) &&
        !this.canRunVerificationStep(Step.llvmBitcodeGenerationStep)

    if (noVerificationStepRemaining) {
      if (!this.context.rootGetters['editor/isProjectIdValid']()) {
        return false
      }

      try {
        const project: Project|null = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
        if (project === null) {
          return false
        }

        return project.llvmBitcodeGenerationStepDone && this.isResetButtonUnlocked
      } catch (e) {
        if (!(e instanceof ProjectNotFound)) {
          this.context.commit(
            `verification-runtime/${MUTATION_PUSH_ERROR}`,
            { error: e },
            { root: true }
          )
        }

        return false
      }
    }

    return false
  }

  get verificationStepReportGetter (): (
    {
      project,
      step
    }: {
      project: Project,
      step: VerificationStep
    }
  ) => string {
    return ({ project, step }: {project: Project, step: VerificationStep}) => {
      if (step === Step.symbolicExecutionStep) {
        return project.symbolicExecutionStepReport.messages
      }

      if (step === Step.llvmBitcodeGenerationStep) {
        return project.llvmBitcodeGenerationStepReport.messages
      }

      if (step === Step.sourceRestorationStep) {
        return project.sourceRestorationStepReport.messages
      }

      if (step === Step.programVerificationStep) {
        return project.programVerificationStepReport.messages
      }

      return ''
    }
  }

  get isVerificationStepReportVisible (): (step: VerificationStep) => boolean {
    return (step: VerificationStep) => {
      if (step === Step.llvmBitcodeGenerationStep) {
        return this.context.rootGetters[`step/llvm-bitcode-generation/${isLlvmBitcodeGenerationReportVisible}`]
      }

      if (step === Step.symbolicExecutionStep) {
        return this.context.rootGetters[`step/symbolic-execution/${isSymbolicExecutionReportVisible}`]
      }

      if (step === Step.programVerificationStep) {
        return this.context.rootGetters[`step/symbolic-execution/${isProgramVerificationReportVisible}`]
      }

      throw new UnexpectedStep()
    }
  }

  get isVerificationStepProgressCompleted (): (project: Project, pollingTarget: VerificationStepPollingTarget) => VerificationStepAssertion {
    return (project: Project, pollingTarget: VerificationStepPollingTarget) => {
      return project[pollingTarget].raw_status &&
          project[pollingTarget].raw_status === Progress.completed
    }
  }

  get isVerificationStepSuccessful (): VerificationStepAssertion {
    return (project: Project, pollingTarget: VerificationStepPollingTarget) => {
      if (pollingTarget === PollingTarget.LLVMBitcodeGenerationStepReport) {
        return project.llvmBitcodeGenerationStepDone
      }

      if (pollingTarget === PollingTarget.SymbolicExecutionStepProgress) {
        return project.symbolicExecutionStepDone
      }

      if (pollingTarget === PollingTarget.ProgramVerificationStepProgress) {
        return project.programVerificationStepDone
      }

      if (pollingTarget === PollingTarget.SourceRestorationStepProgress) {
        return project.sourceRestorationStepDone
      }

      throw new UnexpectedStep(`Sorry, pollingTarget ${pollingTarget} is unexpected.`)
    }
  }

  @Action
  toggleVerificationStepReportVisibility (step: VerificationStep) : void {
    const isReportVisible = this.isVerificationStepReportVisible(step)

    if (step === Step.llvmBitcodeGenerationStep) {
      if (isReportVisible) {
        this.context.commit(
            `step/llvm-bitcode-generation/${hideLlvmBitcodeGenerationReport}`,
            {},
            { root: true }
        )
        return
      }

      this.context.commit(
          `step/llvm-bitcode-generation/${showLlvmBitcodeGenerationReport}`,
          {},
          { root: true }
      )
      return
    }

    if (step === Step.symbolicExecutionStep) {
      if (isReportVisible) {
        this.context.commit(
            `step/symbolic-execution/${hideSymbolicExecutionReport}`,
            {},
            { root: true }
        )
        return
      }

      this.context.commit(
          `step/symbolic-execution/${showSymbolicExecutionReport}`,
          {},
          { root: true }
      )
      return
    }

    if (step === Step.programVerificationStep) {
      if (isReportVisible) {
        this.context.commit(
            `step/program-verification/${hideProgramVerificationReport}`,
            {},
            { root: true }
        )
        return
      }

      this.context.commit(
          `step/program-verification/${showProgramVerificationReport}`,
          {},
          { root: true }
      )
      return
    }

    throw new UnexpectedStep('Can not toggle report visibility')
  }

  get verificationStepReportVisibilityToggler (): (step: VerificationStep) => () => void {
    return (step: VerificationStep) => {
      if (step === Step.llvmBitcodeGenerationStep) {
        return () => EventBus.$emit(VerificationEvents.llvmBitcodeGeneration)
      }

      if (step === Step.symbolicExecutionStep) {
        return () => EventBus.$emit(VerificationEvents.symbolicExecution)
      }

      if (step === Step.programVerificationStep) {
        return () => EventBus.$emit(VerificationEvents.programVerification)
      }

      throw new UnexpectedStep('Can not toggle report visibility')
    }
  }

  get nextStep (): () => VerificationStep {
    return () => this.verificationStep
  }
}

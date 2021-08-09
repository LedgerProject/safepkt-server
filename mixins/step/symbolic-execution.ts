import { Component, mixins, namespace } from 'nuxt-property-decorator'
import { VerificationStepPollingTarget } from '~/types/verification-steps'
import { PollingTarget, VerificationStep } from '~/modules/verification-steps'
import { ProjectNotFound } from '~/mixins/project'
import VerificationStepsMixin from '~/mixins/verification-steps'
import { Project } from '~/types/project'
import EventBus from '~/modules/event-bus'
import VerificationEvents from '~/modules/events'
import { ACTION_RESET_SYMBOLIC_EXECUTION } from '~/store/step/symbolic-execution'
import { GETTER_ACTIVE_PROJECT } from '~/store/verification-runtime'

const SymbolicExecutionStore = namespace('step/symbolic-execution')

@Component
class SymbolicExecutionMixin extends mixins(VerificationStepsMixin) {
  @SymbolicExecutionStore.Getter
  commandPreview!: (projectId: string) => string

  @SymbolicExecutionStore.Getter
  isSymbolicExecutionRunning!: boolean

  @SymbolicExecutionStore.Getter
  flags!: string

  @SymbolicExecutionStore.Getter
  canRunSymbolicExecutionStep!: () => boolean

  @SymbolicExecutionStore.Action
  [ACTION_RESET_SYMBOLIC_EXECUTION]!: (project: Project) => void

  @SymbolicExecutionStore.Action
  runSymbolicExecution!: ({ project, flags }:{project : Project, flags: string}) => void

  @SymbolicExecutionStore.Action
  pollSymbolicExecutionProgress!: (project: Project) => void

  @SymbolicExecutionStore.Mutation
  setAdditionalFlags!: (flags: string) => void

  created () {
    EventBus.$off(VerificationEvents.symbolicExecution)
    EventBus.$on(VerificationEvents.symbolicExecution, () => {
      this.toggleVerificationStepReportVisibility(VerificationStep.symbolicExecutionStep)
    })
  }

  beforeDestroyed () {
    EventBus.$off(VerificationEvents.symbolicExecution)
  }

  pollingSymbolicExecutionProgress?: ReturnType<typeof setInterval>

  startPollingSymbolicExecutionProgress (): void {
    const pollingTarget: VerificationStepPollingTarget = PollingTarget.SymbolicExecutionStepProgress

    this.pollingSymbolicExecutionProgress = setInterval(() => {
      let project: Project

      try {
        project = this[GETTER_ACTIVE_PROJECT]

        if (
          project.llvmBitcodeGenerationStepStarted ||
            !project.llvmBitcodeGenerationStepDone ||
            !project.symbolicExecutionStepStarted
        ) {
          // Early return when LLVM bitcode generation has not yet been started
          // nor it is done
          return
        }

        if (this.isVerificationStepSuccessful(project, pollingTarget)) {
          if (this.pollingSymbolicExecutionProgress) {
            clearInterval(this.pollingSymbolicExecutionProgress)
          }
          return
        }

        this.pollSymbolicExecutionProgress(project)
      } catch (e) {
        if (e instanceof ProjectNotFound) {
          // expected behavior
        } else if (this.pollingSymbolicExecutionProgress) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
          clearInterval(this.pollingSymbolicExecutionProgress)
        }
      }
    }, 1000)
  }

  async tryToRunSymbolicExecution () {
    this.$router.push({
      name: 'symbolic-execution',
      params: { projectId: this.projectId }
    })

    const project: Project = this[GETTER_ACTIVE_PROJECT]
    this[ACTION_RESET_SYMBOLIC_EXECUTION](project)
    this.startPollingSymbolicExecutionProgress()

    await this.runSymbolicExecution({
      project,
      flags: this.flags.trim()
    })
  }
}

export default SymbolicExecutionMixin

import { Component, mixins, namespace } from 'nuxt-property-decorator'
import { Project } from '~/types/project'
import { VerificationStepPollingTarget } from '~/types/verification-steps'
import { PollingTarget, VerificationStep } from '~/modules/verification-steps'
import { ProjectNotFound } from '~/mixins/project'
import VerificationStepsMixin from '~/mixins/verification-steps'
import EventBus from '~/modules/event-bus'
import VerificationEvents, { AppEvents } from '~/modules/events'
import { GETTER_ACTIVE_PROJECT } from '~/store/verification-runtime'

const ProgramVerification = namespace('step/program-verification')

@Component
class ProgramVerificationMixin extends mixins(VerificationStepsMixin) {
  @ProgramVerification.Getter
  canRunProgramVerificationStep!: () => boolean;

  @ProgramVerification.Action
  verifyProgram!: (project: Project) => void

  @ProgramVerification.Action
  pollProgramVerificationProgress!: (project: Project) => void

  pollingProgramVerificationProgress?: ReturnType<typeof setInterval>

  created () {
    EventBus.$off(VerificationEvents.programVerification)

    EventBus.$on(VerificationEvents.programVerification, () => {
      this.toggleVerificationStepReportVisibility(VerificationStep.programVerificationStep)
    })
  }

  beforeDestroyed () {
    EventBus.$off(VerificationEvents.programVerification)
  }

  startPollingProgramVerificationProgress () {
    const pollingTarget: VerificationStepPollingTarget = PollingTarget.ProgramVerificationStepProgress

    this.pollingProgramVerificationProgress = setInterval(() => {
      let project: Project

      try {
        project = this[GETTER_ACTIVE_PROJECT]

        if (!project.programVerificationStepStarted) {
          return
        }

        if (this.isVerificationStepProgressCompleted(project, pollingTarget)) {
          if (this.pollingProgramVerificationProgress) {
            EventBus.$emit(AppEvents.programVerificationRequested)
            clearInterval(this.pollingProgramVerificationProgress)
          }
          return
        }

        this.pollProgramVerificationProgress(project)
      } catch (e) {
        if (e instanceof ProjectNotFound) {
          // expected behavior
        } else if (this.pollingProgramVerificationProgress) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
          clearInterval(this.pollingProgramVerificationProgress)
        }
      }
    }, 1000)
  }

  pollingProgramVerificationReport?: ReturnType<typeof setInterval>

  async tryToVerifyProgram () {
    this.startPollingProgramVerificationProgress()

    await this.verifyProgram(this[GETTER_ACTIVE_PROJECT])
    this.$router.push({
      name: 'program-verification',
      params: { projectId: this.projectId }
    })
  }
}

export default ProgramVerificationMixin

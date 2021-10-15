import { Component, mixins, namespace } from 'nuxt-property-decorator'
import { Project } from '~/types/project'
import { VerificationStepPollingTarget } from '~/types/verification-steps'
import { PollingTarget } from '~/modules/verification-steps'
import { ProjectNotFound } from '~/mixins/project'
import VerificationStepsMixin from '~/mixins/verification-steps'
import EventBus from '~/modules/event-bus'
import VerificationEvents, { AppEvents } from '~/modules/events'
import { GETTER_ACTIVE_PROJECT } from '~/store/verification-runtime'

const SourceRestorationStore = namespace('step/source-restoration')

@Component
class SourceRestorationMixin extends mixins(VerificationStepsMixin) {
  @SourceRestorationStore.Action
  restoreSource!: ({ projectId }: {projectId: string}) => void

  @SourceRestorationStore.Action
  pollSourceRestorationProgress!: (project: Project) => void

  pollingSourceRestorationProgress?: ReturnType<typeof setInterval>

  created () {
    EventBus.$off(VerificationEvents.sourceRestoration)

    EventBus.$on(VerificationEvents.sourceRestoration, () => {
      EventBus.$emit(AppEvents.showEditorRequested)
    })
  }

  beforeDestroyed () {
    EventBus.$off(VerificationEvents.sourceRestoration)
  }

  startPollingSourceRestorationProgress () {
    const pollingTarget: VerificationStepPollingTarget = PollingTarget.SourceRestorationStepProgress

    this.pollingSourceRestorationProgress = setInterval(() => {
      let project: Project

      try {
        project = this[GETTER_ACTIVE_PROJECT]

        if (!(this.$route.name === 'source-restoration') || project === null) {
          return
        }

        if (this.isVerificationStepProgressCompleted(project, pollingTarget)) {
          if (this.pollingSourceRestorationProgress) {
            clearInterval(this.pollingSourceRestorationProgress)
          }
          return
        }

        this.pollSourceRestorationProgress(project)
      } catch (e) {
        if (e instanceof ProjectNotFound) {
          throw e
        } else if (this.pollingSourceRestorationProgress) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
          clearInterval(this.pollingSourceRestorationProgress)
        }
      }
    }, 1000)
  }

  async tryToRestoreSource () {
    this.startPollingSourceRestorationProgress()

    await this.restoreSource({ projectId: this.$route.params.projectId })
  }
}

export default SourceRestorationMixin

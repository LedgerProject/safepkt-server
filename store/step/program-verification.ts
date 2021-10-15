import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Vue from 'vue'
import { Project } from '~/types/project'
import { HttpMethod } from '~/config'
import { VerificationStepProgress } from '~/modules/verification-steps'
import EventBus from '~/modules/event-bus'
import VerificationEvents from '~/modules/events'
import { ProjectNotFound } from '~/mixins/project'
import { stableStringify } from '~/modules/json'
import { MUTATION_HIDE_EDITOR } from '~/store/step/upload-source'
import {
  GETTER_ACTIVE_PROJECT,
  MUTATION_ADD_PROJECT,
  MUTATION_PUSH_ERROR
} from '~/store/verification-runtime'
import { MUTATION_UNLOCK_RESET_BUTTON } from '~/store/verification-steps'

const ACTION_RESET_PROGRAM_VERIFICATION = 'resetProgramVerification'
const GETTER_IS_REPORT_VISIBLE = 'isReportVisible'
const MUTATION_HIDE_REPORT = 'hideReport'
const MUTATION_SHOW_REPORT = 'showReport'

export {
  ACTION_RESET_PROGRAM_VERIFICATION,
  GETTER_IS_REPORT_VISIBLE,
  MUTATION_HIDE_REPORT,
  MUTATION_SHOW_REPORT
}

@Module({
  name: 'program-verification',
  stateFactory: true,
  namespaced: true
})
class ProgramVerificationStore extends VuexModule {
  step: {
    isReportVisible: boolean
  } = {
    isReportVisible: false
  }

  get [GETTER_IS_REPORT_VISIBLE] (): boolean {
    return this.step.isReportVisible
  }

  @Mutation
  [MUTATION_HIDE_REPORT] (): void {
    this.step = {
      ...this.step,
      isReportVisible: false
    }
  }

  @Mutation
  [MUTATION_SHOW_REPORT] (): void {
    this.step = {
      ...this.step,
      isReportVisible: true
    }
  }

  public get canRunProgramVerificationStep (): () => boolean {
    return () => {
      if (!this.context.rootGetters['editor/isProjectIdValid']()) {
        return false
      }

      try {
        const project: Project|null = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
        if (project === null) {
          return false
        }

        return this.canVerifyProgramForProject({ project })
      } catch (e) {
        if (!(e instanceof ProjectNotFound)) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
        }

        return false
      }
    }
  }

  get canVerifyProgramForProject (): ({ project }: {project: Project}) => boolean {
    return ({ project }: {project: Project}) => {
      // there is no on-going program verification
      const canDo = !project.programVerificationStepStarted &&
        project.programVerificationStepDone

      if (typeof canDo === 'undefined') {
        return false
      }

      return canDo
    }
  }

  @Action
  async verifyProgram (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.startProgramVerification.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.startProgramVerification.method

    try {
      this.context.commit(`step/upload-source/${MUTATION_HIDE_EDITOR}`, {}, { root: true })
      this.context.commit(MUTATION_SHOW_REPORT)

      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
        typeof json.error !== 'undefined'
      ) {
        Vue.notify({
          title: 'Warning',
          text: `Sorry, the program verification has failed for project having id ${project.id}.`,
          type: 'warn'
        })

        return
      }

      Vue.notify({
        title: 'Success',
        text: [
          `program verification has been successfully triggered for project having id ${project.id}:`,
          json.message
        ].join('\n'),
        type: 'success'
      })

      const projectState: Project = {
        ...project,
        programVerificationStepStarted: true
      }

      this.context.commit(
        `verification-runtime/${MUTATION_ADD_PROJECT}`,
        projectState,
        { root: true }
      )
    } catch (e) {
      if (!(e instanceof ProjectNotFound)) {
        Vue.notify({
          title: 'Oops',
          text: 'Sorry, something went wrong when trying to verify program.',
          type: 'error'
        })

        this.context.commit(
          `verification-runtime/${MUTATION_PUSH_ERROR}`,
          { error: e },
          { root: true }
        )
      }
    }
  }

  @Action
  async pollProgramVerificationProgress (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getProgramVerificationProgress.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getProgramVerificationProgress.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const programVerificationStepDone = json.raw_status === VerificationStepProgress.completed

      const projectState: Project = {
        ...project,
        programVerificationStepProgress: json,
        programVerificationStepDone
      }

      await this.context.dispatch('pollProgramVerificationReport', project)
      projectState.programVerificationStepReport = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`].programVerificationStepReport

      if (programVerificationStepDone) {
        projectState.programVerificationStepStarted = false
        this.context.commit(
          `verification-steps/${MUTATION_UNLOCK_RESET_BUTTON}`,
          {},
          { root: true }
        )
      }

      const currentProjectState = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
      if (stableStringify(currentProjectState) !== stableStringify(projectState)) {
        this.context.commit(
          `verification-runtime/${MUTATION_ADD_PROJECT}`,
          projectState,
          { root: true }
        )
      }
    } catch (e) {
      if (!(e instanceof ProjectNotFound)) {
        Vue.notify({
          title: 'Oops',
          text: 'Sorry, something went wrong when trying to poll the program verification progress.',
          type: 'error'
        })

        this.context.commit(
          `verification-runtime/${MUTATION_PUSH_ERROR}`,
          { error: e },
          { root: true }
        )
      }
    }
  }

  @Action
  async pollProgramVerificationReport (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getProgramVerificationReport.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getProgramVerificationReport.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.messages === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const projectState: Project = {
        ...project,
        programVerificationStepReport: {
          ...json,
          messages: json.messages
        }
      }

      const currentProjectState = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
      if (stableStringify(currentProjectState) !== stableStringify(projectState)) {
        this.context.commit(
            `verification-runtime/${MUTATION_ADD_PROJECT}`,
            projectState,
            { root: true }
        )
      }

      if (json.messages.includes('FAILED:')) {
        EventBus.$emit(VerificationEvents.failedVerificationStep, { error: new Error(json.messages) })
      }
    } catch (e) {
      if (!(e instanceof ProjectNotFound)) {
        Vue.notify({
          title: 'Oops',
          text: 'Sorry, something went wrong when trying to poll the program verification report.',
          type: 'error'
        })

        this.context.commit(
          `verification-runtime/${MUTATION_PUSH_ERROR}`,
          { error: e },
          { root: true }
        )
      }
    }
  }

  @Action
  [ACTION_RESET_PROGRAM_VERIFICATION] (project: Project): void {
    const projectState: Project = {
      ...project
    }

    project.programVerificationStepStarted = false
    project.programVerificationStepProgress = {}
    project.programVerificationStepReport = {}
    project.programVerificationStepDone = false

    this.context.commit(
        `verification-runtime/${MUTATION_ADD_PROJECT}`,
        projectState,
        { root: true }
    )
  }
}

export default ProgramVerificationStore

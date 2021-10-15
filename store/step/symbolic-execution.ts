import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Vue from 'vue'
import { VerificationStepProgress } from '~/modules/verification-steps'
import { Project } from '~/types/project'
import { HttpMethod, Routes } from '~/config'
import { ProjectNotFound } from '~/mixins/project'
import EventBus from '~/modules/event-bus'
import VerificationEvents from '~/modules/events'
import { stableStringify } from '~/modules/json'
import {
  GETTER_ACTIVE_PROJECT,
  MUTATION_ADD_PROJECT,
  MUTATION_PUSH_ERROR
} from '~/store/verification-runtime'
import { MUTATION_HIDE_EDITOR } from '~/store/step/upload-source'
import { MUTATION_UNLOCK_RESET_BUTTON } from '~/store/verification-steps'

const ACTION_RESET_SYMBOLIC_EXECUTION = 'resetSymbolicExecution'
const GETTER_IS_REPORT_VISIBLE = 'isReportVisible'
const MUTATION_HIDE_REPORT = 'hideReport'
const MUTATION_SHOW_REPORT = 'showReport'

export {
  ACTION_RESET_SYMBOLIC_EXECUTION,
  GETTER_IS_REPORT_VISIBLE,
  MUTATION_HIDE_REPORT,
  MUTATION_SHOW_REPORT
}

@Module({
  name: 'symbolic-execution',
  stateFactory: true,
  namespaced: true
})
class SymbolicExecutionStore extends VuexModule {
  step: {
    commandFlags: string,
    isReportVisible: boolean
  } = {
    commandFlags: '',
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

  get flags (): string {
    return this.step.commandFlags
  }

  get isSymbolicExecutionRunning (): boolean {
    if (!this.context.rootGetters['editor/isProjectIdValid']()) {
      return false
    }

    const project: Project|null = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
    if (project === null) {
      return false
    }

    return project.symbolicExecutionStepStarted &&
      !project.symbolicExecutionStepDone
  }

  get commandPreview (): (projectId: string) => string {
    return (projectId: string) => {
      if (this.flags.length === 0) {
        return `klee --libc=klee --silent-klee-assume --warnings-only-to-file ${projectId}.bc`
      }

      return `klee --libc=klee ${this.flags.trim()} ${projectId}.bc`
    }
  }

  @Mutation
  setAdditionalFlags (flags: string): void {
    this.step = {
      ...this.step,
      ...{ commandFlags: flags }
    }
  }

  public get canRunSymbolicExecutionStep (): () => boolean {
    return () => {
      let project: Project|null

      try {
        project = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
        if (project === null) {
          return false
        }

        return project.llvmBitcodeGenerationStepDone &&
            !project.symbolicExecutionStepStarted // No symbolic execution has started
      } catch (e) {
        if (!(e instanceof ProjectNotFound)) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
        }

        return false
      }
    }
  }

  @Action
  [ACTION_RESET_SYMBOLIC_EXECUTION] (project: Project): void {
    const projectState: Project = {
      ...project
    }

    projectState.symbolicExecutionStepStarted = false
    projectState.symbolicExecutionStepProgress = {}
    projectState.symbolicExecutionStepReport = {}
    projectState.symbolicExecutionStepDone = false

    this.context.commit(
        `verification-runtime/${MUTATION_ADD_PROJECT}`,
        projectState,
        { root: true }
    )
  }

  @Action
  async runSymbolicExecution ({ project, flags }: {project: Project, flags: string}) {
    const { baseUrl, routes }: { baseUrl: string, routes: Routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.startSymbolicExecution.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.startSymbolicExecution.method
    const body: BodyInit = JSON.stringify({ flags })

    try {
      this.context.commit(`step/upload-source/${MUTATION_HIDE_EDITOR}`, {}, { root: true })
      this.context.commit(`step/llvm-bitcode-generation/${MUTATION_HIDE_REPORT}`, {}, { root: true })
      this.context.commit(`${MUTATION_SHOW_REPORT}`)

      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, body))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        Vue.notify({
          title: 'Warning',
          text: `Sorry, the symbolic execution has failed for project having id ${project.id}.`,
          type: 'warn'
        })

        return
      }

      Vue.notify({
        title: 'Success',
        text: [
          `Symbolic execution has been successfully triggered for project having id ${project.id}.`,
          json.message
        ].join('\n'),
        type: 'success'
      })

      const projectState: Project = {
        ...project,
        symbolicExecutionStepStarted: true,
        symbolicExecutionStepDone: false
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
          text: 'Sorry, something went wrong when trying to run the symbolic execution.',
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
  async pollSymbolicExecutionProgress (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getSymbolicExecutionProgress.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getSymbolicExecutionProgress.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const symbolicExecutionStepDone = json.raw_status === VerificationStepProgress.completed

      const projectState: Project = {
        ...project,
        symbolicExecutionStepProgress: json,
        symbolicExecutionStepDone
      }

      await this.context.dispatch('pollSymbolicExecutionReport', project)
      projectState.symbolicExecutionStepReport = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`].symbolicExecutionStepReport

      if (symbolicExecutionStepDone) {
        projectState.symbolicExecutionStepStarted = false
        this.context.commit(
          `verification-steps/${MUTATION_UNLOCK_RESET_BUTTON}`,
          {},
          { root: true }
        )
      }

      const currentProjectState = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]

      if (
        symbolicExecutionStepDone ||
        stableStringify(currentProjectState) !== stableStringify(projectState)
      ) {
        this.context.commit(
            `verification-runtime/${MUTATION_ADD_PROJECT}`,
            projectState,
            { root: true }
        )
      }
    } catch (e) {
      Vue.notify({
        title: 'Oops',
        text: 'Sorry, something went wrong when trying to poll the symbolic execution progress.',
        type: 'error'
      })
      if (!(e instanceof ProjectNotFound)) {
        this.context.commit(
          `verification-runtime/${MUTATION_PUSH_ERROR}`,
          { error: e },
          { root: true }
        )
      }
    }
  }

  @Action
  async pollSymbolicExecutionReport (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getSymbolicExecutionReport.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getSymbolicExecutionReport.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.messages === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const currentReport = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`].symbolicExecutionStepReport
      if (stableStringify(json) !== stableStringify(currentReport)) {
        const projectState: Project = {
          ...project,
          symbolicExecutionStepReport: json
        }

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
          text: 'Sorry, something went wrong when trying to poll the symbolic execution report.',
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
}

export default SymbolicExecutionStore

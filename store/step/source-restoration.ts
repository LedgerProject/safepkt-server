import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Vue from 'vue'
import { HttpMethod } from '~/config'
import { Project } from '~/types/project'
import { VerificationStep, VerificationStepProgress } from '~/modules/verification-steps'
import { MUTATION_SET_VERIFICATION_STEP } from '~/store/verification-steps'

import {
  GETTER_ACTIVE_PROJECT,
  MUTATION_ADD_PROJECT,
  MUTATION_PUSH_ERROR
} from '~/store/verification-runtime'

import { stableStringify } from '~/modules/json'
import { ProjectNotFound } from '~/mixins/project'

import {
  ACTION_ENCODE_SOURCE,
  MUTATION_SET_PROJECT_ID,
  MUTATION_SET_PROJECT_NAME
} from '~/store/editor'

const ACTION_REFRESH_EDITOR = 'refreshEditor'
const ACTION_RESET_SOURCE_RESTORATION = 'resetSourceRestoration'
const GETTER_IS_REPORT_VISIBLE = 'isReportVisible'
const MUTATION_HIDE_REPORT = 'hideReport'
const MUTATION_SHOW_REPORT = 'showReport'

export {
  ACTION_REFRESH_EDITOR,
  ACTION_RESET_SOURCE_RESTORATION,
  GETTER_IS_REPORT_VISIBLE,
  MUTATION_HIDE_REPORT,
  MUTATION_SHOW_REPORT
}

@Module({
  name: 'restore-source',
  stateFactory: true,
  namespaced: true
})
class RestoreSourceStore extends VuexModule {
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

  @Action
  async restoreSource ({ projectId }: {projectId: string}) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.startSourceRestoration.url}`
      .replace('{{ projectId }}', projectId)
    const method: HttpMethod = routes.startSourceRestoration.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        Vue.notify({
          title: 'Warning',
          text: 'Sorry, the source restoration has failed.',
          type: 'warn'
        })

        return
      }

      Vue.notify({
        title: 'Success',
        text: `Source restoration was successfully started for project id ${json.project_id}.`,
        type: 'success'
      })

      const projectRevision = (new Date()).getTime()

      const projectState: Project = {
        id: projectId,
        revision: projectRevision, // Use timestamp as project revision
        name: projectId, // Consider temporarily project id as project name
        source: '',
        llvmBitcodeGenerationStepStarted: false,
        llvmBitcodeGenerationStepReport: {},
        llvmBitcodeGenerationStepProgress: {},
        llvmBitcodeGenerationStepDone: false,
        symbolicExecutionStepStarted: false,
        symbolicExecutionStepReport: {},
        symbolicExecutionStepProgress: {},
        symbolicExecutionStepDone: false,
        sourceRestorationStepStarted: true, // Key change for source restoration
        sourceRestorationStepReport: {},
        sourceRestorationStepProgress: {},
        sourceRestorationStepDone: false,
        programVerificationStepStarted: false,
        programVerificationStepReport: {},
        programVerificationStepProgress: {},
        programVerificationStepDone: false
      }

      this.context.dispatch(ACTION_REFRESH_EDITOR, { project: projectState })

      this.context.commit(
          `verification-runtime/${MUTATION_ADD_PROJECT}`,
          projectState,
          { root: true }
      )
    } catch (e) {
      this.context.commit(
          `verification-runtime/${MUTATION_PUSH_ERROR}`,
          { error: e },
          { root: true }
      )

      Vue.notify({
        title: 'Oops',
        text: 'Sorry, something went wrong when trying to restore some source code.',
        type: 'error'
      })
    }
  }

  @Action
  async pollSourceRestorationProgress (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getSourceRestorationProgress.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getSourceRestorationProgress.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const sourceRestorationStepDone = json.raw_status === VerificationStepProgress.completed

      const projectState: Project = {
        ...project,
        sourceRestorationStepProgress: json,
        sourceRestorationStepDone
      }

      await this.context.dispatch('pollSourceRestorationReport', project)
      projectState.sourceRestorationStepReport = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`].sourceRestorationStepReport

      if (sourceRestorationStepDone) {
        projectState.sourceRestorationStepStarted = false
        this.context.commit(
            `verification-steps/${MUTATION_SET_VERIFICATION_STEP}`,
            VerificationStep.programVerificationStep,
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
          text: 'Sorry, something went wrong when trying to poll the source restoration progress.',
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
  async pollSourceRestorationReport (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getSourceRestorationReport.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getSourceRestorationReport.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.error !== 'undefined' ||
          typeof json.raw_log === 'undefined'
      ) {
        return
      }

      const quote = '\\"'
      const projectNamePattern = new RegExp(
        `.*${quote}project_name${quote}:\\s*${quote}([^"]+)${quote}.*`,
        'g'
      )
      const matches = [...json.raw_log.matchAll(projectNamePattern)]

      let projectName: string = project.id
      if (Array.isArray(matches[0]) && matches[0].length > 1) {
        projectName = matches[0][1]
      }

      const projectRevision = (new Date()).getTime()

      const projectState: Project = {
        ...project,
        name: projectName,
        revision: projectRevision,
        source: btoa(json.raw_log),
        sourceRestorationStepReport: {
          ...json,
          messages: json.raw_log
        }
      }

      this.context.dispatch(ACTION_REFRESH_EDITOR, { project: projectState })

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
          text: 'Sorry, something went wrong when trying to poll the source restoration report.',
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
  [ACTION_REFRESH_EDITOR] ({ project }: {project: Project}): void {
    this.context.commit(
      `step/source-restoration/${MUTATION_HIDE_REPORT}`,
      {},
      { root: true }
    )
    this.context.dispatch(
      `editor/${ACTION_ENCODE_SOURCE}`,
      atob(project.source),
      { root: true }
    )
    this.context.commit(
      `editor/${MUTATION_SET_PROJECT_ID}`,
      { projectId: project.id, revision: project.revision },
      { root: true }
    )
    this.context.commit(
      `editor/${MUTATION_SET_PROJECT_NAME}`,
      project.name,
      { root: true }
    )
  }

  @Action
  [ACTION_RESET_SOURCE_RESTORATION] (project: Project): void {
    const projectState: Project = {
      ...project
    }

    project.sourceRestorationStepStarted = false
    project.sourceRestorationStepProgress = {}
    project.sourceRestorationStepReport = {}
    project.sourceRestorationStepDone = false

    this.context.commit(
        `verification-runtime/${MUTATION_ADD_PROJECT}`,
        projectState,
        { root: true }
    )
  }
}

export default RestoreSourceStore

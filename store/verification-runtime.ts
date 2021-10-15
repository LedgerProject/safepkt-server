import Vue from 'vue'
import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import { Project } from '~/types/project'
import Config, { HttpMethod, Routes } from '~/config'
import {
  VerificationStep as VerificationStepMod
} from '~/modules/verification-steps'
import { ProjectNotFound } from '~/mixins/project'
import { ACTION_RESET_LLVM_BITCODE_GENERATION, MUTATION_HIDE_REPORT as hideLlvmBitcodeGenerationReport } from '~/store/step/llvm-bitcode-generation'
import { ACTION_RESET_SYMBOLIC_EXECUTION, MUTATION_HIDE_REPORT as hideSymbolicExecutionReport } from '~/store/step/symbolic-execution'
import { ACTION_RESET_PROGRAM_VERIFICATION, MUTATION_HIDE_REPORT as hideProgramVerificationReport } from '~/store/step/program-verification'
import {
  GETTER_PROJECT_REVISION,
  MUTATION_SET_BASE64_ENCODED_SOURCE,
  MUTATION_SET_PROJECT_ID,
  MUTATION_SET_PROJECT_NAME
} from '~/store/editor'

import {
  MUTATION_LOCK_RESET_BUTTON,
  MUTATION_SET_VERIFICATION_STEP
} from '~/store/verification-steps'

import { MUTATION_SHOW_EDITOR } from '~/store/step/upload-source'
import EventBus from '~/modules/event-bus'
import VerificationEvents from '~/modules/events'

const ACTION_EMPTY_HISTORY = 'emptyHistory'
const MUTATION_REMOVE_REVISION = 'removeRevision'
const ACTION_RESET_PROJECTS = 'resetProject'
const ACTION_RESET_VERIFICATION_RUNTIME = 'resetVerificationRuntime'
const ACTION_REVERT_TO_REVISION = 'revertToRevision'
const GETTER_ACTIVE_PROJECT = 'activeProject'
const GETTER_IS_HISTORY_EMPTY = 'isHistoryEmpty'
const GETTER_PROJECT_BY_REVISION = 'projectByRevision'
const GETTER_PROJECT_BY_REVISION_GETTER = 'projectByRevisionGetter'
const MUTATION_ADD_PROJECT = 'addProject'
const MUTATION_PUSH_ERROR = 'pushError'
const MUTATION_RESET_PROJECTS = 'resetProjects'

export {
  ACTION_EMPTY_HISTORY,
  ACTION_RESET_PROJECTS,
  ACTION_RESET_VERIFICATION_RUNTIME,
  ACTION_REVERT_TO_REVISION,
  GETTER_ACTIVE_PROJECT,
  GETTER_IS_HISTORY_EMPTY,
  GETTER_PROJECT_BY_REVISION,
  GETTER_PROJECT_BY_REVISION_GETTER,
  MUTATION_ADD_PROJECT,
  MUTATION_REMOVE_REVISION,
  MUTATION_PUSH_ERROR,
  MUTATION_RESET_PROJECTS
}

@Module({
  name: 'verification-runtime',
  stateFactory: true,
  namespaced: true
})
export default class VerificationRuntimeStore extends VuexModule {
  projects: Project[] = []

  @Mutation
  [MUTATION_ADD_PROJECT] (project: Project): void {
    const indexedProjects = this.projects
      .reduce((acc: { [x: string]: any }, p: Project) => {
        acc[p.revision] = p
        return acc
      }, {})

    indexedProjects[project.revision] = project

    this.projects = Object.keys(indexedProjects)
      .map(revision => indexedProjects[revision])
  }

  @Mutation
  [MUTATION_RESET_PROJECTS] (): void {
    this.projects = []
  }

  get allProjects (): Project[] {
    return this.projects
  }

  get [GETTER_IS_HISTORY_EMPTY] (): boolean {
    return this.projects.length === 0
  }

  get indexedProjects () : {[key: string]: Project} {
    return this.projects
      .reduce((acc: { [x: string]: any }, p: Project) => {
        acc[p.revision] = p
        return acc
      }, {})
  }

  get [GETTER_PROJECT_BY_REVISION_GETTER] () : (projectRevision: string) => Project|undefined {
    return (projectRevision: string) => {
      return this.context.getters.indexedProjects[projectRevision]
    }
  }

  errors: Error[] = []

  @Mutation
  [MUTATION_PUSH_ERROR] ({ error }: {error: Error}) {
    if (error) {
      this.errors.push(error)
    }
  }

  get lastError (): Error|null {
    if (this.errors.length === 0) {
      return null
    }

    return this.errors[this.errors.length - 1]
  }

  get getFetchRequestInit (): (method: HttpMethod, body: BodyInit|null) => RequestInit {
    return (method: HttpMethod, body: BodyInit|null = null) => {
      const requestInit: RequestInit = {
        method,
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        headers: {
          'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer'
      }

      if (body !== null) {
        requestInit.body = body
      }

      return requestInit
    }
  }

  /** @throw ProjectNotFound */
  get [GETTER_PROJECT_BY_REVISION] (): (projectRevision: string) => Project {
    return (projectRevision: string) => {
      const project = this[GETTER_PROJECT_BY_REVISION_GETTER](projectRevision)
      if (typeof project === 'undefined') {
        throw new ProjectNotFound(`Could not find project having revision ${projectRevision}`)
      }

      return project
    }
  }

  get [GETTER_ACTIVE_PROJECT] (): Project|null {
    if (!this.context.rootGetters['editor/isProjectIdValid']()) {
      return null
    }

    try {
      return this[GETTER_PROJECT_BY_REVISION](this.context.rootGetters[`editor/${GETTER_PROJECT_REVISION}`])
    } catch (e) {
      return null
    }
  }

  get routingParams (): {baseUrl: string, routes: Routes} {
    return {
      baseUrl: Config.getBaseURL(),
      routes: Config.getRoutes()
    }
  }

  @Action
  [ACTION_EMPTY_HISTORY] (): void {
    this.context.commit(MUTATION_RESET_PROJECTS)
    this.context.dispatch(ACTION_RESET_VERIFICATION_RUNTIME)
  }

  @Mutation
  [MUTATION_REMOVE_REVISION] ({ revision }: { revision: number }): void {
    this.projects = [...this.projects.filter(p => p.revision !== revision)]
  }

  @Action
  [ACTION_RESET_PROJECTS] (): void {
    const allProjects = this.context.getters.allProjects

    if (
      typeof allProjects === 'undefined' ||
        !allProjects
    ) {
      return
    }

    Object.keys([...allProjects])
      .map((id: any) => {
        const project = allProjects[id]

        return {
          ...project
        }
      })
      .forEach((p: Project) => {
        this.context.dispatch(
          `step/llvm-bitcode-generation/${ACTION_RESET_LLVM_BITCODE_GENERATION}`,
          p,
          { root: true }
        )
        this.context.dispatch(
          `step/symbolic-execution/${ACTION_RESET_SYMBOLIC_EXECUTION}`,
          p,
          { root: true }
        )
      })
  }

  @Action
  [ACTION_RESET_VERIFICATION_RUNTIME] (): void {
    this.context.commit(
      'step/upload-source/enableSourceUpload',
      {},
      { root: true }
    )
    this.context.commit(
      `step/upload-source/${MUTATION_SHOW_EDITOR}`,
      {},
      { root: true }
    )
    this.context.commit(
      `step/llvm-bitcode-generation/${hideLlvmBitcodeGenerationReport}`,
      {},
      { root: true }
    )
    this.context.commit(
      `step/symbolic-execution/${hideSymbolicExecutionReport}`,
      {},
      { root: true }
    )
    this.context.commit(
      'editor/setProjectId',
      { projectId: '' },
      { root: true }
    )
    this.context.commit(
      `verification-steps/${MUTATION_SET_VERIFICATION_STEP}`,
      VerificationStepMod.uploadSourceStep,
      { root: true }
    )
    this.context.commit(
      `verification-steps/${MUTATION_LOCK_RESET_BUTTON}`,
      {},
      { root: true }
    )
    this.context.dispatch(ACTION_RESET_PROJECTS)
  }

  @Action
  [ACTION_REVERT_TO_REVISION] ({ revision }: {revision: string}): void {
    const project: Project|null = this[GETTER_PROJECT_BY_REVISION](revision)

    if (project === null) {
      Vue.notify({
        title: 'Warning',
        text: 'Impossible to revert to this revision.',
        type: 'warn'
      })
      return
    }

    this.context.commit(
      `editor/${MUTATION_SET_BASE64_ENCODED_SOURCE}`,
      atob(project.source),
      { root: true }
    )
    this.context.commit(
      `editor/${MUTATION_SET_PROJECT_NAME}`,
      project.name,
      { root: true }
    )
    this.context.commit(
      `editor/${MUTATION_SET_PROJECT_ID}`,
      { projectId: project.id, revision: project.revision },
      { root: true }
    )

    EventBus.$emit(VerificationEvents.resetVerificationRuntime)
  }
}

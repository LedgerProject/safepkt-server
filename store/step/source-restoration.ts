import { Action, Module, VuexModule } from 'vuex-module-decorators'
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
import { MUTATION_HIDE_EDITOR } from '~/store/step/upload-source'
import { stableStringify } from '~/modules/json'
import { ProjectNotFound } from '~/mixins/project'

@Module({
  name: 'restore-source',
  stateFactory: true,
  namespaced: true
})
class RestoreSourceStore extends VuexModule {
  @Action
  async restoreSource (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.startSourceRestoration.url}`
      .replace('{{ projectId }}', project.id)
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
        text: `The source was successfully restored under project id ${json.project_id}.`,
        type: 'success'
      })

      this.context.commit(
        'editor/setProjectId',
        { projectId: project.id },
        { root: true }
      )

      const projectState: Project = {
        ...project,
        sourceRestorationStepStarted: true
      }

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
            `step/upload-source/${MUTATION_HIDE_EDITOR}`,
            VerificationStep.symbolicExecutionStep,
            { root: true }
        )
        this.context.commit(
            `verification-steps/${MUTATION_SET_VERIFICATION_STEP}`,
            VerificationStep.llvmBitcodeGenerationStep,
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
        typeof json.messages === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const projectState: Project = {
        ...project,
        sourceRestorationStepReport: {
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
}

export default RestoreSourceStore

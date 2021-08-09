import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Vue from 'vue'
import { Project } from '~/types/project'
import { HttpMethod } from '~/config'
import { VerificationStep, VerificationStepProgress } from '~/modules/verification-steps'
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
import { MUTATION_SET_VERIFICATION_STEP } from '~/store/verification-steps'

const ACTION_RESET_LLVM_BITCODE_GENERATION = 'resetLlvmBitcodeGeneration'
const GETTER_IS_REPORT_VISIBLE = 'isReportVisible'
const MUTATION_HIDE_REPORT = 'hideReport'
const MUTATION_SHOW_REPORT = 'showReport'

export {
  ACTION_RESET_LLVM_BITCODE_GENERATION,
  GETTER_IS_REPORT_VISIBLE,
  MUTATION_HIDE_REPORT,
  MUTATION_SHOW_REPORT
}

@Module({
  name: 'llvm-bitcode-generation',
  stateFactory: true,
  namespaced: true
})
class LlvmBitcodeGenerationStore extends VuexModule {
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

  public get canRunLlvmBitcodeGenerationStep (): () => boolean {
    return () => {
      if (!this.context.rootGetters['editor/isProjectIdValid']()) {
        return false
      }

      try {
        const project: Project|null = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`]
        if (project === null) {
          return false
        }

        return this.canGenerateLlvmBitcodeForProject({ project })
      } catch (e) {
        if (!(e instanceof ProjectNotFound)) {
          EventBus.$emit(VerificationEvents.failedVerificationStep, { error: e })
        }

        return false
      }
    }
  }

  get canGenerateLlvmBitcodeForProject (): ({ project }: {project: Project}) => boolean {
    return ({ project }: {project: Project}) => {
      const canDo = !project.llvmBitcodeGenerationStepStarted && !project.llvmBitcodeGenerationStepDone
      if (typeof canDo === 'undefined') {
        return false
      }

      return canDo
    }
  }

  @Action
  async generateLlvmBitcode (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.startLLVMBitcodeGeneration.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.startLLVMBitcodeGeneration.method

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
          text: `Sorry, the LLVM bitcode generation has failed for project having id ${project.id}.`,
          type: 'warn'
        })

        return
      }

      Vue.notify({
        title: 'Success',
        text: [
          `LLVM bitcode generation has been successfully triggered for project having id ${project.id}:`,
          json.message
        ].join('\n'),
        type: 'success'
      })

      const projectState: Project = {
        ...project,
        llvmBitcodeGenerationStepStarted: true
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
          text: 'Sorry, something went wrong when trying to generate LLVM bitcode.',
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
  async pollLlvmBitcodeGenerationProgress (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getLLVMBitcodeGenerationProgress.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getLLVMBitcodeGenerationProgress.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.message === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const llvmBitcodeGenerationStepDone = json.raw_status === VerificationStepProgress.completed

      const projectState: Project = {
        ...project,
        llvmBitcodeGenerationStepProgress: json,
        llvmBitcodeGenerationStepDone
      }

      await this.context.dispatch('pollLlvmBitcodeGenerationReport', project)
      projectState.llvmBitcodeGenerationStepReport = this.context.rootGetters[`verification-runtime/${GETTER_ACTIVE_PROJECT}`].llvmBitcodeGenerationStepReport

      if (llvmBitcodeGenerationStepDone) {
        projectState.llvmBitcodeGenerationStepStarted = false
        this.context.commit(
            `step/upload-source/${MUTATION_HIDE_EDITOR}`,
            VerificationStep.symbolicExecutionStep,
            { root: true }
        )
        this.context.commit(
          `verification-steps/${MUTATION_SET_VERIFICATION_STEP}`,
          VerificationStep.symbolicExecutionStep,
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
          text: 'Sorry, something went wrong when trying to poll the LLVM bitcode generation progress.',
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
  async pollLlvmBitcodeGenerationReport (project: Project) {
    const { baseUrl, routes } = this.context.rootGetters['verification-runtime/routingParams']

    const url = `${baseUrl}${routes.getLLVMBitcodeGenerationReport.url}`
      .replace('{{ projectId }}', project.id)
    const method: HttpMethod = routes.getLLVMBitcodeGenerationReport.method

    try {
      const response = await fetch(url, this.context.rootGetters['verification-runtime/getFetchRequestInit'](method, null))
      const json = await response.json()

      if (
        typeof json.messages === 'undefined' ||
          typeof json.error !== 'undefined'
      ) {
        return
      }

      const lineFeeds = '\n'.repeat(2)
      const projectState: Project = {
        ...project,
        llvmBitcodeGenerationStepReport: {
          ...json,
          messages: `${json.messages}${lineFeeds}`
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
          text: 'Sorry, something went wrong when trying to poll the LLVM bitcode generation report.',
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
  [ACTION_RESET_LLVM_BITCODE_GENERATION] (project: Project): void {
    const projectState: Project = {
      ...project
    }

    project.llvmBitcodeGenerationStepStarted = false
    project.llvmBitcodeGenerationStepProgress = {}
    project.llvmBitcodeGenerationStepReport = {}
    project.llvmBitcodeGenerationStepDone = false

    this.context.commit(
        `verification-runtime/${MUTATION_ADD_PROJECT}`,
        projectState,
        { root: true }
    )
  }
}

export default LlvmBitcodeGenerationStore

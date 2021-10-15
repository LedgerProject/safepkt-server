<template>
  <div class="app">
    <AppHeader />
    <div class="app__body">
      <VerificationSteps
        :enable-upload-source-button="canRunVerificationStep(steps.uploadSourceStep)"
        :enable-generate-llvm-bitcode-button="canRunVerificationStep(steps.llvmBitcodeGenerationStep)"
        :enable-run-symbolic-execution-button="canRunVerificationStep(steps.symbolicExecutionStep)"
        :enable-reset-verification-runtime-button="canResetVerificationRuntime"
      />
      <Editor />
      <Report
        :title="reportTitle('programVerification')"
        :title-icon="titleIcon('programVerification')"
        :content="verificationStepReport('programVerification')"
        :is-report-visible="isVerificationStepReportVisible('programVerification')"
        :toggle-report-visibility="verificationStepReportVisibilityToggler('programVerification')"
      />
      <Report
        v-show="false"
        :title="reportTitle('llvmBitcodeGeneration')"
        :title-icon="titleIcon('llvmBitcodeGeneration')"
        :content="verificationStepReport('llvmBitcodeGeneration')"
        :is-report-visible="isVerificationStepReportVisible('llvmBitcodeGeneration')"
        :toggle-report-visibility="verificationStepReportVisibilityToggler('llvmBitcodeGeneration')"
      />
      <Report
        v-show="false"
        :title="reportTitle('symbolicExecution')"
        :title-icon="titleIcon('symbolicExecution')"
        :content="verificationStepReport('symbolicExecution')"
        :is-report-visible="isVerificationStepReportVisible('symbolicExecution')"
        :toggle-report-visibility="verificationStepReportVisibilityToggler('symbolicExecution')"
      >
        <SymbolicExecutionFlags
          v-if="isVerificationStepReportVisible('symbolicExecution')"
        />
      </Report>
      <History v-show="showHistory" />
      <notifications position="bottom right" />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, mixins, namespace } from 'nuxt-property-decorator'
import AppHeader from '~/components/app-header/app-header.vue'
import Editor from '~/components/editor/editor.vue'
import History from '~/components/history/history.vue'
import Report from '~/components/report/report.vue'
import UploadSourceMixin from '~/mixins/step/upload-source'
import SourceRestorationMixin from '~/mixins/step/source-restoration'
import ProgramVerificationMixin from '~/mixins/step/program-verification'
import SymbolicExecutionMixin from '~/mixins/step/symbolic-execution'
import EventBus from '~/modules/event-bus'
import VerificationEvents, { AppEvents } from '~/modules/events'
import { UnexpectedStep, VerificationStep } from '~/modules/verification-steps'
import SymbolicExecutionFlags from '~/components/symbolic-execution-flags/symbolic-execution-flags.vue'
import VerificationSteps from '~/components/verification-steps/verification-steps.vue'
import { VerificationStep as VerificationStepType } from '~/types/verification-steps'
import MetaMixin from '~/mixins/meta'

const VerificationRuntime = namespace('verification-runtime')

@Component({
  components: {
    AppHeader,
    Editor,
    History,
    Report,
    SymbolicExecutionFlags,
    VerificationSteps
  }
})
export default class Homepage extends mixins(
  MetaMixin,
  ProgramVerificationMixin,
  UploadSourceMixin,
  SourceRestorationMixin,
  SymbolicExecutionMixin
) {
  showHistory: boolean = true
  steps: VerificationStep = new VerificationStep()

  @VerificationRuntime.Action
  emptyHistory!: () => void

  beforeDestroy () {
    EventBus.$off(AppEvents.clearHistoryRequested)
    EventBus.$off(VerificationEvents.resetVerificationRuntime)
    EventBus.$off(VerificationEvents.failedVerificationStep)

    if (this.pollingLlvmBitcodeGenerationProgress) {
      clearInterval(this.pollingLlvmBitcodeGenerationProgress)
    }
    if (this.pollingSymbolicExecutionProgress) {
      clearInterval(this.pollingSymbolicExecutionProgress)
    }
    if (this.pollingProgramVerificationProgress) {
      clearInterval(this.pollingProgramVerificationProgress)
    }
    if (this.pollingSourceRestorationProgress) {
      clearInterval(this.pollingSourceRestorationProgress)
    }
  }

  created () {
    this.steps = new VerificationStep()

    EventBus.$off(AppEvents.clearHistoryRequested)
    EventBus.$off(VerificationEvents.failedVerificationStep)
    EventBus.$off(VerificationEvents.resetVerificationRuntime)

    EventBus.$on(AppEvents.clearHistoryRequested, this.clearHistory)
    EventBus.$on(VerificationEvents.failedVerificationStep, this.reportError)
    EventBus.$on(VerificationEvents.resetVerificationRuntime, this.reset)

    if (this.tryToRestorePreviouslyUploadedSource()) {
      EventBus.$emit(AppEvents.showEditorRequested)

      return
    }

    EventBus.$emit(VerificationEvents.resetVerificationRuntime)
  }

  clearHistory () {
    EventBus.$emit(VerificationEvents.resetVerificationRuntime)
    this.emptyHistory()
  }

  goToHomepage () {
    this.$router.push({ name: 'homepage' })
  }

  tryToRestorePreviouslyUploadedSource () {
    return this.$route.name === 'source-restoration'
  }

  reset () {
    if (this.tryToRestorePreviouslyUploadedSource()) {
      console.log('TODO: try to restore previously uploaded source.')
      // Restore previously uploaded source in editor
    } else {
      this.goToHomepage()
      this.resetVerificationRuntime()
    }

    this.startPollingSymbolicExecutionProgress()
    this.startPollingLlvmBitcodeGenerationProgress()
    this.startPollingProgramVerificationProgress()
    this.startPollingSourceRestorationProgress()
  }

  get titleIcon (): (step: VerificationStepType) => string {
    return (step: VerificationStepType) => {
      switch (true) {
        case step === VerificationStep.programVerificationStep:
          if (this.isVerificationStepReportVisible(step)) {
            return 'Hide program verification report'
          }

          return 'Show program verification report'

        case step === VerificationStep.llvmBitcodeGenerationStep:
          if (this.isVerificationStepReportVisible(step)) {
            return 'Hide LLVM bitcode generation report'
          }

          return 'Show LLVM bitcode generation report'

        case step === VerificationStep.symbolicExecutionStep:
          if (this.isVerificationStepReportVisible(step)) {
            return 'Hide symbolic execution report'
          }

          return 'Show symbolic execution report'

        case step === VerificationStep.sourceRestorationStep:
          if (this.isVerificationStepReportVisible(step)) {
            return 'Hide source restoration report'
          }

          return 'Show source restoration report'

        default:

          throw new UnexpectedStep()
      }
    }
  }
}
</script>

<style lang="scss">
@import "./index.scss";
</style>

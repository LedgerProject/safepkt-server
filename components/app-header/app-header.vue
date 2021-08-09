<template>
  <div class="app-header">
    <div class="app-header__row">
      <h1 class="app-header__title">
        Safe <Logo :source="logo" />
      </h1>
      <p class="app-header__short-description">
        Static analysis of rust-based programs
      </p>
      <VerificationSteps
        :enable-upload-source-button="canRunVerificationStep(steps.uploadSourceStep)"
        :enable-generate-llvm-bitcode-button="canRunVerificationStep(steps.llvmBitcodeGenerationStep)"
        :enable-run-symbolic-execution-button="canRunVerificationStep(steps.symbolicExecutionStep)"
        :enable-reset-verification-runtime-button="canResetVerificationRuntime"
        show-shortcuts
      />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, mixins } from 'nuxt-property-decorator'

import Logo from '~/components/logo/logo.vue'
import VerificationSteps from '~/components/verification-steps/verification-steps.vue'
import logo from '~/assets/pkt-logo.svg'
import VerificationStepsMixin from '~/mixins/verification-steps'
import { VerificationStep } from '~/modules/verification-steps'

@Component({
  components: { Logo, VerificationSteps }
})
export default class AppHeader extends mixins(VerificationStepsMixin) {
  logo: string = logo;

  steps: VerificationStep = new VerificationStep()

  created () {
    this.steps = new VerificationStep()
  }
}
</script>

<style lang="scss">
@import "./app-header.scss";
</style>

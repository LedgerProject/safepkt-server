<template>
  <div
    v-if="canRunVerificationStep(steps.symbolicExecutionStep) || isSymbolicExecutionRunning"
    class="symbolic-execution-flags symbolic-execution-flags__row-group"
  >
    <div class="symbolic-execution-flags__row">
      <label
        class="symbolic-execution-flags__step-label"
        for="additional-flags"
      >
        <span class="symbolic-execution-flags__label">Optional symbolic execution flags:</span>
        <input
          id="additional-flags"
          class="symbolic-execution-flags__flags"
          :disabled="!canRunVerificationStep(steps.symbolicExecutionStep)"
          :value="flags"
          maxlength="200"
          type="text"
          autocomplete="off"
          placeholder="example: --help"
          @input="amendAdditionalFlags"
        >
      </label>
    </div>
    <div class="symbolic-execution-flags__row">
      <label for="command-preview">
        <span class="symbolic-execution-flags__label">Symbolic execution command preview:</span>
        <div
          id="command-preview"
          class="symbolic-execution-flags__preview"
          v-text="symbolicExecutionCommandPreview"
        />
      </label>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, mixins } from 'nuxt-property-decorator'
import { VerificationStep } from '~/modules/verification-steps'
import SymbolicExecutionMixin from '~/mixins/step/symbolic-execution'

@Component
class SymbolicExecutionFlags extends mixins(SymbolicExecutionMixin) {
  steps: VerificationStep = new VerificationStep()

  amendAdditionalFlags ({ target }: {target: {value: string}}) {
    this.setAdditionalFlags(target.value)
  }

  get symbolicExecutionCommandPreview (): string {
    return this.commandPreview(this.projectId)
  }
}

export default SymbolicExecutionFlags
</script>

<style lang="scss">
@import "./symbolic-execution-flags.scss";
</style>

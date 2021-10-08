<template>
  <div :class="componentClasses">
    <!-- source upload and llvm bitcode generation
         have been combined -->
    <div class="verification-steps__row verification-steps__steps">
      <label class="verification-steps__step-label">
        <button
          v-if="false"
          :class="uploadSourceButtonClasses()"
          :disabled="!enableUploadSourceButton"
          @click="tryToUploadSource"
        >
          <span v-if="!showShortcuts">Generate LLVM Bitcode [IR]</span>
          <span v-else>IR</span>
        </button>
        <button
          :class="uploadSourceButtonClasses()"
          :disabled="!enableUploadSourceButton"
          @click="tryToUploadSource"
        >
          <!--          :class="symbolicExecutionButtonClasses()"-->
          <!--          :disabled="!enableRunSymbolicExecutionButton"-->
          <!--          @click="tryToRunSymbolicExecution"-->
          <span v-if="!showShortcuts">Verify program [V]</span>
          <span v-else>V</span>
        </button>
        <button
          :class="resetVerificationButtonClasses()"
          :disabled="!enableResetVerificationRuntimeButton"
          @click="resetRuntime"
        >
          <span v-if="!showShortcuts">⚠️ Reset runtime [R]</span>
          <span v-else>R</span>
        </button>
      </label>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, mixins, Prop } from 'nuxt-property-decorator'
import UploadSourceMixin from '~/mixins/step/upload-source'
import SymbolicExecutionMixin from '~/mixins/step/symbolic-execution'
import VerificationEvents from '~/modules/events'
import EventBus from '~/modules/event-bus'

@Component
export default class VerificationSteps extends mixins(
  UploadSourceMixin,
  SymbolicExecutionMixin
) {
  @Prop({
    type: Boolean,
    required: true
  })
  enableUploadSourceButton!: boolean;

  @Prop({
    type: Boolean,
    required: true
  })
  enableGenerateLlvmBitcodeButton!: boolean;

  @Prop({
    type: Boolean,
    required: true
  })
  enableRunSymbolicExecutionButton!: boolean;

  @Prop({
    type: Boolean,
    required: true
  })
  enableResetVerificationRuntimeButton!: boolean;

  @Prop({
    type: Boolean,
    required: false
  })
  showShortcuts!: boolean;

  generateLlvmBitcodeButtonClasses () {
    return this.getDefaultDisabledButtonClass()
  }

  uploadSourceButtonClasses () {
    if (!this.enableUploadSourceButton) {
      return this.getDefaultDisabledButtonClass()
    }
  }

  symbolicExecutionButtonClasses () {
    if (!this.enableRunSymbolicExecutionButton) {
      return this.getDefaultDisabledButtonClass()
    }
  }

  resetVerificationButtonClasses () {
    if (!this.enableResetVerificationRuntimeButton) {
      return this.getDefaultDisabledButtonClass()
    }
  }

  resetRuntime (): void {
    EventBus.$emit(VerificationEvents.resetVerificationRuntime)
  }

  get componentClasses (): {[key: string]: boolean} {
    return {
      'verification-steps': true,
      'verification-steps--with-shortcuts': this.showShortcuts
    }
  }

  getDefaultDisabledButtonClass () {
    return {
      'verification-steps__disabled': true
    }
  }
}
</script>

<style lang="scss">
@import "./verification-steps.scss";
</style>

<template>
  <div class="editor">
    <!-- See https://nuxtjs.org/docs/2.x/features/nuxt-components#the-client-only-component -->
    <client-only placeholder="Loading...">
      <h2 class="editor__title">
        {{ reportTitle(steps.uploadSourceStep) }}
        <font-awesome-icon
          class="editor__icon"
          :icon="showEditorIcon"
          :title="showEditorIconTitle"
          @click="toggleEditorVisibility"
        />
      </h2>
      <div
        v-if="isEditorVisible"
        class="editor__row"
      >
        <label
          class="editor__step-label"
          for="project-name"
        >
          <span class="editor__label">Pick a name for your project to be verified:</span>
          <input
            id="project-name"
            :value="projectName"
            class="editor__project-name"
            maxlength="30"
            type="text"
            @input="amendProjectName"
          >
        </label>
      </div>
      <prism-editor
        v-if="isEditorVisible"
        :value="source"
        class="editor__inner-editor"
        :highlight="highlighter"
        line-numbers
        @input="setSource"
      />
    </client-only>
  </div>
</template>

<script lang="ts">
import { Component, Watch, mixins } from 'nuxt-property-decorator'

import { PrismEditor } from 'vue-prism-editor'
import 'vue-prism-editor/dist/prismeditor.min.css' // import the styles somewhere

import { highlight, languages } from 'prismjs/components/prism-core'
import 'prismjs/components/prism-clike'
import 'prismjs/components/prism-rust'
import 'prismjs/themes/prism-okaidia.css'
import { VerificationStep } from '~/modules/verification-steps'
import VerificationSteps from '~/components/verification-steps/verification-steps.vue'
import UploadSourceMixin from '~/mixins/step/upload-source'
import SymbolicExecutionMixin from '~/mixins/step/symbolic-execution'
import {
  MUTATION_SET_BASE64_ENCODED_SOURCE,
  MUTATION_SET_PROJECT_NAME
} from '~/store/editor'
import EventBus from '~/modules/event-bus'
import { AppEvents } from '~/modules/events'

@Component({
  components: { PrismEditor, VerificationSteps }
})
export default class Editor extends mixins(
  UploadSourceMixin,
  SymbolicExecutionMixin
) {
  showEditorIcon: string = 'eye-slash';
  showEditorIconTitle: string = 'Hide editor';

  steps: VerificationStep = new VerificationStep()

  beforeDestroy () {
    EventBus.$off(AppEvents.showEditorRequested)
  }

  created () {
    this.steps = new VerificationStep()

    EventBus.$off(AppEvents.showEditorRequested)
    EventBus.$on(AppEvents.showEditorRequested, this.showEditor)
  }

  amendProjectName ({ target }: {target: {value: string}}) {
    this[MUTATION_SET_PROJECT_NAME](target.value)
  }

  setSource (input: string) {
    this[MUTATION_SET_BASE64_ENCODED_SOURCE](input)
  }

  @Watch('isEditorVisible', { deep: true, immediate: true })
  onVisibilityUpdate (newVisibility: boolean) {
    if (newVisibility) {
      this.showEditorIcon = 'eye-slash'
      this.showEditorIconTitle = 'Hide editor'

      return
    }

    this.showEditorIcon = 'eye'
    this.showEditorIconTitle = 'Show editor'
  }

  highlighter (source: string) {
    return highlight(source, languages.rust)
  }

  toggleEditorVisibility () {
    if (this.isEditorVisible) {
      this.hideEditor()

      return
    }

    this.showEditor()
  }

  get source (): string {
    return atob(this.base64EncodedSource)
  }
}
</script>

<style lang="scss">
@import './editor.scss';
</style>

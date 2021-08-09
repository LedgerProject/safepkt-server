<template>
  <div :class="getReportClasses()">
    <h2 class="report__title">
      {{ title }}
      <font-awesome-icon
        class="report__icon"
        :icon="icon"
        :title="titleIcon"
        @click="toggleVisibility"
      />
    </h2>
    <slot />
    <textarea
      v-if="isReportVisible"
      ref="content"
      v-model="content"
      class="report__content"
    />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Watch, mixins } from 'nuxt-property-decorator'
import SymbolicExecutionMixin from '~/mixins/step/symbolic-execution'

@Component
export default class Report extends mixins(SymbolicExecutionMixin) {
  $refs!: {
    content: HTMLElement
  }

  icon: string = 'eye-slash';

  @Prop({
    type: String,
    required: true
  })
  title!: string

  @Prop({
    type: String,
    required: true
  })
  titleIcon!: string

  @Prop({
    type: String,
    default: ''
  })
  content!: string

  @Prop({
    type: Boolean,
    required: true
  })
  isReportVisible!: boolean

  @Prop({
    type: Function,
    required: true
  })
  toggleReportVisibility!: () => void

  @Watch('content', { deep: true, immediate: true })
  onContentUpdated () {
    const content = this.$refs.content
    if (typeof content === 'undefined') {
      return
    }

    content.scrollTop = content.scrollHeight + 2 * 8
  }

  @Watch('isReportVisible', { deep: true, immediate: true })
  onVisibilityUpdate (newVisibility: boolean) {
    if (newVisibility) {
      this.icon = 'eye-slash'

      return
    }

    this.icon = 'eye'
  }

  getReportClasses () {
    return { report: true }
  }

  toggleVisibility (): void {
    this.toggleReportVisibility()
  }
}
</script>

<style lang="scss">
@import './report.scss';
</style>

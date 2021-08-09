<template>
  <div class="logo">
    <img
      alt="PKT logo"
      class="logo__logo"
      :height="side"
      :src="source"
      :width="side"
    >
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'nuxt-property-decorator'

@Component
class Logo extends Vue {
  @Prop({
    type: String,
    required: true
  })
  readonly source!: string;

  @Prop({
    type: Number,
    default: 75
  })
  readonly side!: number;
}

export default Logo
</script>

<style lang="scss">
@import './logo.scss';
</style>

<template>
  <div v-if="!isHistoryEmpty" class="history">
    <h2 class="history__title">
      <font-awesome-icon icon="hourglass" />
      Revisions
    </h2>
    <label for="trash">
      <button
        id="trash"
        class="history__button"
        @click="clearHistory"
      >
        Clear history
      </button>
    </label>
    <ul class="history__list">
      <li
        v-for="project in sortedProjects"
        :key="formatIndex(project)"
        :class="revisionClasses(project)"
      >
        <div
          class="history__action"
          :title="formatActionTitle('Revert to revision {}', project)"
          @click="revertTo({revision: project.revision})"
        >
          <font-awesome-icon
            class="history__icon"
            icon="history"
          />
          <div class="history__contract">
            <div v-text="formatContractName(project)" />
            <span class="history__contract-revision" v-text="formatContractRevision(project)" />
          </div>
        </div>
        <div
          class="history__action history__action--remove"
          @click="remove({revision: project.revision})"
        >
          <font-awesome-icon
            class="history__trash-icon"
            icon="trash"
            :title="formatActionTitle('Remove revision {}', project)"
          />
        </div>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { Component, Vue, namespace } from 'nuxt-property-decorator'
import { Project } from '~/types/project'
import { AppEvents } from '~/modules/events'
import EventBus from '~/modules/event-bus'
import {
  ACTION_REVERT_TO_REVISION,
  MUTATION_REMOVE_REVISION,
  GETTER_ACTIVE_PROJECT
} from '~/store/verification-runtime'

const VerificationRuntime = namespace('verification-runtime')

@Component
export default class History extends Vue {
  @VerificationRuntime.Action
  [ACTION_REVERT_TO_REVISION]!: ({ revision }: { revision: number }) => void

  @VerificationRuntime.Getter
  allProjects!: Project[]

  @VerificationRuntime.Getter
  isHistoryEmpty!: boolean

  @VerificationRuntime.Getter
  [GETTER_ACTIVE_PROJECT]!: Project|null

  @VerificationRuntime.Mutation
  [MUTATION_REMOVE_REVISION]!: ({ revision }: { revision: number }) => void

  clearHistory () {
    EventBus.$emit(AppEvents.clearHistoryRequested)
  }

  get revisionClasses (): (project: Project) => {[key: string]: boolean} {
    return (project: Project) => {
      const activeProject: Project|null = this[GETTER_ACTIVE_PROJECT]

      let isActive = false
      if (activeProject !== null) {
        isActive = project.revision === activeProject.revision
      }

      return {
        'history__item--active': isActive,
        history__item: true
      }
    }
  }

  get sortedProjects (): Project[] {
    return [...this.allProjects].sort((left, right) => {
      if (left.revision === right.revision) {
        return 0
      }

      if (left.revision < right.revision) {
        return 1
      } else {
        return -1
      }
    })
  }

  formatContractName (contract: Project): string {
    return `${contract.name}`
  }

  formatContractRevision (contract: Project): string {
    const revisionDate = new Date(contract.revision)
    const formattedTime = this.$dateFns.format(revisionDate, 'kk:mm {} EEEE MMMM yyyy')
      .replace('{}', 'on')

    return `(revised at ${formattedTime})`
  }

  formatIndex (contract: Project): string {
    return `${contract.id}_${contract.revision}`
  }

  formatActionTitle (tpl: string, contract: Project) {
    return tpl.replace('{}', `"${contract.revision}"`)
  }

  remove ({ revision }: { revision: number }) {
    this.removeRevision({ revision })
  }

  revertTo ({ revision }: { revision: number }) {
    this[ACTION_REVERT_TO_REVISION]({ revision })
  }
}
</script>

<style lang="scss">
@import './history.scss';
</style>

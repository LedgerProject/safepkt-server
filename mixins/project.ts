import { Component, Vue, namespace } from 'nuxt-property-decorator'

// - provides with logger
// - configures API host, scheme, port for current environment
import SharedState from '../modules/shared-state'
import {
  ACTION_ENCODE_SOURCE,
  GETTER_PROJECT_ID,
  GETTER_PROJECT_REVISION,
  MUTATION_SET_PROJECT_NAME
} from '~/store/editor'

const Editor = namespace('editor')

class ProjectNotFound extends Error {}

export { ProjectNotFound }

@Component
class ProjectMixin extends Vue {
  logger = new SharedState.Logger()

  @Editor.Getter
  [GETTER_PROJECT_ID]!: string

  @Editor.Getter
  [GETTER_PROJECT_REVISION]!: string

  @Editor.Getter
  projectName!: string

  @Editor.Getter
  base64EncodedSource!: string

  @Editor.Action
  [ACTION_ENCODE_SOURCE]!: (source: string) => void

  @Editor.Mutation
  setProjectId!: ({ projectId }: {projectId: string}) => void

  @Editor.Mutation
  [MUTATION_SET_PROJECT_NAME]!: (projectName: string) => void

  @Editor.Getter
  isProjectIdValid!: () => boolean
}

export default ProjectMixin

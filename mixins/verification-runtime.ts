import { Component, namespace } from 'nuxt-property-decorator'
import ProjectMixin from '~/mixins/project'
import { Project } from '~/types/project'
import {
  GETTER_ACTIVE_PROJECT,
  GETTER_PROJECT_BY_REVISION,
  ACTION_RESET_VERIFICATION_RUNTIME
} from '~/store/verification-runtime'

const VerificationRuntimeStore = namespace('verification-runtime')

@Component
class VerificationRuntimeMixin extends ProjectMixin {
  @VerificationRuntimeStore.Getter
  public [GETTER_ACTIVE_PROJECT]!: Project;

  @VerificationRuntimeStore.Action
  public [ACTION_RESET_VERIFICATION_RUNTIME]!: () => void

  @VerificationRuntimeStore.Getter
  public [GETTER_PROJECT_BY_REVISION]!: (projectRevision: string) => Project;
}

export default VerificationRuntimeMixin

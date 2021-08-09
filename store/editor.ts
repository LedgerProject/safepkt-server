import { Module, Mutation, VuexModule } from 'vuex-module-decorators'

const GETTER_PROJECT_ID = 'projectId'
const GETTER_PROJECT_REVISION = 'projectRevision'
const MUTATION_SET_BASE64_ENCODED_SOURCE = 'setBase64EncodedSource'
const MUTATION_SET_PROJECT_ID = 'setProjectId'
const MUTATION_SET_PROJECT_NAME = 'setProjectName'

export {
  GETTER_PROJECT_ID,
  GETTER_PROJECT_REVISION,
  MUTATION_SET_BASE64_ENCODED_SOURCE,
  MUTATION_SET_PROJECT_ID,
  MUTATION_SET_PROJECT_NAME
}

@Module({
  name: 'editor',
  stateFactory: true,
  namespaced: true
})
export default class EditorStore extends VuexModule {
  editor: {
    projectId: string,
    projectRevision: number,
    projectName: string,
    base64EncodedSource: string
  } = {
    projectId: '',
    projectRevision: 0,
    projectName: 'Multiplication',
    base64EncodedSource: btoa(`use verification_annotations::prelude::*;

fn main() {
    let a = u32::abstract_value();
    let b = u32::abstract_value();
    verifier::assume(1 <= a && a <= 1000);
    verifier::assume(1 <= b && b <= 1000);
    if verifier::is_replay() {
        eprintln!("Test values: a = {}, b = {}", a, b);
    }
    let r = a * b;
    verifier::assert!(1 <= r && r <= 1000000);
}
`)
  }

  @Mutation
  [MUTATION_SET_BASE64_ENCODED_SOURCE] (source: string): void {
    this.editor = { ...this.editor, base64EncodedSource: btoa(source) }
  }

  @Mutation
  [MUTATION_SET_PROJECT_ID] (
    {
      projectId,
      revision
    }: {
      projectId: string,
      revision: number
    }
  ): void {
    if (!projectId || projectId.trim().length === 0) {
      return
    }

    let projectRevision = (new Date()).getTime()
    if (revision > 0) {
      projectRevision = revision
    }

    this.editor = {
      ...this.editor,
      projectId,
      projectRevision
    }
  }

  @Mutation
  [MUTATION_SET_PROJECT_NAME] (projectName: string): void {
    this.editor = { ...this.editor, projectName }
  }

  get [GETTER_PROJECT_ID] (): string {
    return this.editor.projectId
  }

  get [GETTER_PROJECT_REVISION] (): number {
    return this.editor.projectRevision
  }

  get projectName (): string {
    return this.editor.projectName
  }

  get base64EncodedSource (): string {
    return this.editor.base64EncodedSource
  }

  get isProjectIdValid () : () => boolean {
    return (): boolean => {
      return this.editor.projectId.length > 0
    }
  }
}

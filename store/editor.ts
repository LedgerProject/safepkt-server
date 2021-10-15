import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import Contract from './contract'

const ACTION_ENCODE_SOURCE = 'encodeSource'
const GETTER_PROJECT_ID = 'projectId'
const GETTER_PROJECT_REVISION = 'projectRevision'
const GETTER_SOURCE_ENCODER = 'sourceEncoder'
const MUTATION_SET_ENCODED_SOURCE = 'setEncodedSource'
const MUTATION_SET_PROJECT_ID = 'setProjectId'
const MUTATION_SET_PROJECT_NAME = 'setProjectName'

export {
  ACTION_ENCODE_SOURCE,
  GETTER_PROJECT_ID,
  GETTER_PROJECT_REVISION,
  GETTER_SOURCE_ENCODER,
  MUTATION_SET_ENCODED_SOURCE,
  MUTATION_SET_PROJECT_ID,
  MUTATION_SET_PROJECT_NAME
}

class InvalidModuleName extends Error {}

const encodeSource = (source: string): string => {
  if (source.includes('"project_name"')) {
    return btoa(`${source}`)
  }

  let moduleName: string = ''

  source.replace(
    /mod\s+(?!tests)([\S]+)\s+\{.*/g,
    (_: string, ...args: any[]): string => {
      moduleName = args[0]

      return args[2]
    }
  )

  if (moduleName.length === 0) {
    throw new InvalidModuleName('Could not find module name.')
  }

  const lineFeed = '\n'

  return btoa(`${source}${lineFeed}${lineFeed}// {"project_name": ${moduleName}}${lineFeed}`)
}

const source = Contract.source

@Module({
  name: 'editor',
  stateFactory: true,
  namespaced: true
})
class EditorStore extends VuexModule {
  editor: {
    projectId: string,
    projectRevision: number,
    projectName: string,
    base64EncodedSource: string
  } = {
    projectId: '',
    projectRevision: 0,
    projectName: 'Plain Multisig Wallet',
    base64EncodedSource: encodeSource(source)
  }

  get [GETTER_SOURCE_ENCODER] (): (source: string) => string {
    return (source: string): string => {
      try {
        return encodeSource(source)
      } catch (e) {
        if (e instanceof InvalidModuleName && source.trim().length === 0) {
          return source
        }

        throw e
      }
    }
  }

  @Action
  [ACTION_ENCODE_SOURCE] (source: string): void {
    const encodedSource = this.context.rootGetters[`editor/${GETTER_SOURCE_ENCODER}`](source)
    this.context.commit(
        `editor/${MUTATION_SET_ENCODED_SOURCE}`,
        encodedSource,
        { root: true }
    )
  }

  @Mutation
  [MUTATION_SET_ENCODED_SOURCE] (encodedSource: string): void {
    this.editor = { ...this.editor, base64EncodedSource: encodedSource }
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

export default EditorStore

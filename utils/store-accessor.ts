import { Store } from 'vuex'
import { getModule } from 'vuex-module-decorators'
import editor from '~/store/editor'
import report from '~/store/report'
import verificationRuntime from '~/store/verification-runtime'
import verificationSteps from '~/store/verification-steps'
import uploadSource from '~/store/step/upload-source'
import llvmBitcodeGeneration from '~/store/step/llvm-bitcode-generation'
import symbolicExecution from '~/store/step/symbolic-execution'

let editorStore: editor
let reportStore: report
let verificationRuntimeStore: verificationRuntime
let verificationStepsStore: verificationSteps
let llvmBitcodeGenerationStore: llvmBitcodeGeneration
let uploadSourceStore: uploadSource
let symbolicExecutionStore: symbolicExecution

function initialiseStores (store: Store<any>): void {
  editorStore = getModule(editor, store)
  reportStore = getModule(report, store)
  uploadSourceStore = getModule(uploadSource, store)
  llvmBitcodeGenerationStore = getModule(llvmBitcodeGeneration, store)
  symbolicExecutionStore = getModule(symbolicExecution, store)
  verificationRuntimeStore = getModule(verificationRuntime, store)
  verificationStepsStore = getModule(verificationSteps, store)
}

export {
  initialiseStores,
  editorStore,
  reportStore,
  uploadSourceStore,
  llvmBitcodeGenerationStore,
  symbolicExecutionStore,
  verificationRuntimeStore,
  verificationStepsStore
}

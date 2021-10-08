
type Project = {
    id: string,
    revision: number,
    source: string,
    name: string,
    llvmBitcodeGenerationStepStarted: boolean,
    llvmBitcodeGenerationStepProgress: any,
    llvmBitcodeGenerationStepReport: any,
    llvmBitcodeGenerationStepDone: boolean,
    symbolicExecutionStepStarted: boolean,
    symbolicExecutionStepProgress: any,
    symbolicExecutionStepReport: any,
    symbolicExecutionStepDone: boolean,
    sourceRestorationStepProgress: any,
    sourceRestorationStepReport: any,
    sourceRestorationStepDone: boolean,
}

export { Project };

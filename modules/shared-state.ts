import Config from '../config'

type EnvParam = {
    developmentMode: boolean,
    productionMode: boolean
}

type EnvProvider = {
    getEnvironmentParameters: () => EnvParam
}

const developmentMode = process.env.NODE_ENV !== 'production'
const productionMode = !developmentMode

const environmentParameters: EnvParam = {
  developmentMode,
  productionMode: !developmentMode
}

const getEnvironmentParameters = (): EnvParam => environmentParameters

const environmentProvider: EnvProvider = { getEnvironmentParameters }

const api = Config.getApi(environmentProvider)

const isProductionModeActive = (): boolean => getEnvironmentParameters().productionMode

type extra = {[key:string]: string}

type ErrorContext = {
    error: string,
    file: string,
    extra: extra
}

const logLevel = {
  isSilent: false,
  onError: (_: ErrorContext): void => {
  }
}

class Logger {
  info (message: string, file: string, extra: {[key:string]: string}): void {
    if (logLevel.isSilent) {
      return
    }

    if (productionMode) {
      return
    }

    // eslint-disable-next-line no-console
    console.log(message, file, extra)
  }

  error (error: string, file: string, extra: {[key:string]: string}): string {
    logLevel.onError({ error, file, extra })

    if (productionMode || logLevel.isSilent) {
      return error
    }

    // eslint-disable-next-line no-console
    console.error(error, file, extra)

    return error
  }
}

export { EnvParam, EnvProvider }

export default {
  api,
  getEnvironmentParameters,
  isProductionModeActive,
  Logger,
  logLevel
}

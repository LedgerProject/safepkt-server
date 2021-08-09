
const stableStringify = (subject: any, space: string|number = '') => {
  const allKeys: string[] = []
  const seen: {[key: string]: any} = {}
  JSON.stringify(subject, (key: string, value) => {
    if (!(key in seen)) {
      allKeys.push(key)
      seen[key] = null
    }
    return value
  })
  allKeys.sort()

  return JSON.stringify(subject, allKeys, space)
}

export { stableStringify }

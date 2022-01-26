const variables = ['GITHUB_AUTH_TOKEN'] as const

export function environment(): Record<typeof variables[number], string> {
  const env: Record<string, string> = {}

  for (const variable of variables) {
    const value = process.env[variable]
    if (typeof value === 'string') {
      env[variable] = value
    } else {
      throw Error(`Could not find environment variable: ${variable}`)
    }
  }

  return env
}

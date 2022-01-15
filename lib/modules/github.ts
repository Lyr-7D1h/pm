import { Octokit } from 'octokit'

export class Github {
  private octokit: Octokit

  /** https://github.com/settings/tokens */
  constructor(authToken: string) {
    this.octokit = new Octokit({ auth: authToken })
  }

  async listProjects() {
    const repos = await this.octokit.rest.repos.listForAuthenticatedUser()

    console.log(repos.data)
  }
}

import type { NextPage } from 'next'
import { Page } from '../components/Page'
import { environment } from '../lib/environment'
import { Github } from '../lib/modules/github'

// const { GITHUB_AUTH_TOKEN } = environment()

// const gh = new Github(GITHUB_AUTH_TOKEN)

// gh.listProjects()

const Home: NextPage = () => {
  return <Page></Page>
}

export default Home

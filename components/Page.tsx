import { Nav } from './Nav'

export const Page: React.FC = ({ children }) => {
  return (
    <>
      <Nav />
      <main>{children}</main>
    </>
  )
}

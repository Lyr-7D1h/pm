import Link from 'next/link'

export const Nav = () => {
  return (
    <nav className="navbar navbar-expand-md navbar-dark bg-dark mb-4">
      <div className="container-fluid">
        <Link href="/">
          <a className="navbar-brand" href="#">
            Project Manager
          </a>
        </Link>
        {/* <div className="collapse navbar-collapse">
          <ul className="navbar-nav me-auto mb-2 mb-md-0">
            <li className="nav-item">
              <Link href="/modules">
                <a className="nav-link active" aria-current="page">
                  Modules
                </a>
              </Link>
            </li>
          </ul>
        </div> */}
      </div>
    </nav>
  )
}

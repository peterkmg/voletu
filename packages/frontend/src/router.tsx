import { createBrowserRouter } from 'react-router-dom'

import { LoginPage } from '~/routes/LoginPage'
import { RootRoute } from '~/routes/RootRoute'
import { SetupPage } from '~/routes/SetupPage'
import { SuperadminPage } from '~/routes/SuperadminPage'

export const router = createBrowserRouter([
  {
    path: '/',
    element: <RootRoute />,
  },
  {
    path: '/setup',
    element: <SetupPage />,
  },
  {
    path: '/superadmin',
    element: <SuperadminPage />,
  },
  {
    path: '/login',
    element: <LoginPage />,
  },
])

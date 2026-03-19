import { createFileRoute, redirect } from '@tanstack/react-router'
import { AuthenticatedLayout } from '~/components/layout/authenticated-layout'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: ({ location }) => {
    const { auth } = useAuthStore.getState()
    const { startupState } = useStartupStore.getState()

    if (startupState?.needsSetup) {
      throw redirect({ to: '/setup' })
    }

    if (!auth.accessToken) {
      throw redirect({
        to: '/sign-in',
        search: { redirect: location.href },
      })
    }
  },
  component: AuthenticatedLayout,
})

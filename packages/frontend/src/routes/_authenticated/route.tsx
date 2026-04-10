import { createFileRoute, redirect } from '@tanstack/react-router'
import { AuthenticatedLayout } from '~/components/layout/authenticated-layout'
import { useRuntimeStore } from '~/platform/runtime/runtime-store'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: async ({ location }) => {
    const { status, user } = useAuthStore.getState()
    const { startupState } = useStartupStore.getState()
    const { healthStatus } = useRuntimeStore.getState()

    if (startupState?.needsSetup) {
      throw redirect({ to: '/setup' })
    }

    if (status === 'unauthenticated') {
      throw redirect({
        to: '/sign-in',
        search: { redirect: location.href },
      })
    }

    if (
      healthStatus === 'hydrated'
      && !useNodeStore.getState().status.isInitialized
      && user?.role === 'ADMIN'
    ) {
      throw redirect({ to: '/init' })
    }
  },
  component: AuthenticatedLayout,
})

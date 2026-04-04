import { createFileRoute, redirect } from '@tanstack/react-router'
import { getBaseUrl } from '~/api/client'
import { AuthenticatedLayout } from '~/components/layout/authenticated-layout'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: async ({ location }) => {
    const { status, user } = useAuthStore.getState()
    const { startupState } = useStartupStore.getState()

    if (startupState?.needsSetup) {
      throw redirect({ to: '/setup' })
    }

    if (status === 'unauthenticated') {
      throw redirect({
        to: '/sign-in',
        search: { redirect: location.href },
      })
    }

    // Fetch health to know init state before rendering child routes.
    // This prevents pages from firing API requests that will all 403.
    // /init is outside _authenticated, so no infinite redirect loop risk.
    if (!useNodeStore.getState().status.isInitialized) {
      let healthConfirmed = false
      try {
        const controller = new AbortController()
        const timeoutId = setTimeout(() => controller.abort(), 3000)
        const res = await fetch(`${getBaseUrl()}/health`, { signal: controller.signal })
        clearTimeout(timeoutId)
        const data = await res.json() as { success: boolean, data: { isInitialized: boolean, nodeType: string, nodeName: string } }
        if (data?.success) {
          useNodeStore.getState().setStatus({
            isInitialized: data.data.isInitialized,
            nodeType: data.data.nodeType as import('~/stores/node-store').NodeType,
            nodeName: data.data.nodeName,
          })
          healthConfirmed = true
        }
      }
      catch { /* timeout or network failure — let normal flow continue */ }

      // Only redirect if health explicitly confirmed node is not initialized.
      // Don't redirect on network failure (API might just be restarting).
      if (healthConfirmed && !useNodeStore.getState().status.isInitialized && user?.role === 'ADMIN') {
        throw redirect({ to: '/init' })
      }
    }
  },
  component: AuthenticatedLayout,
})

import { createFileRoute, redirect } from '@tanstack/react-router'
import { Setup } from '~/features/setup'
import { useStartupStore } from '~/stores/startup-store'

export const Route = createFileRoute('/(auth)/setup')({
  beforeLoad: () => {
    const { startupState } = useStartupStore.getState()

    if (!startupState?.needsSetup) {
      throw redirect({ to: '/sign-in' })
    }
  },
  component: SetupPage,
})

function SetupPage() {
  return (
    <div className="flex min-h-svh items-center justify-center p-4">
      <Setup />
    </div>
  )
}

import { createFileRoute, redirect } from '@tanstack/react-router'
import { useStartupStore } from '~/stores/startup-store'
import { Setup } from '~/views/setup'

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
    <div className="flex flex-1 items-center justify-center p-4">
      <Setup />
    </div>
  )
}

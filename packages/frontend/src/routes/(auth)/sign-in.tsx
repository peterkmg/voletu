import { createFileRoute, redirect } from '@tanstack/react-router'
import { z } from 'zod'
import { useAuthStore } from '~/stores/auth-store'
import { useStartupStore } from '~/stores/startup-store'
import { SignInForm } from '~/views/auth/forms/sign-in-form'

const searchSchema = z.object({
  redirect: z.string().optional(),
})

export const Route = createFileRoute('/(auth)/sign-in')({
  validateSearch: searchSchema,
  beforeLoad: () => {
    const { accessToken } = useAuthStore.getState()
    const { startupState } = useStartupStore.getState()

    if (startupState?.needsSetup) {
      throw redirect({ to: '/setup' })
    }

    if (accessToken) {
      throw redirect({ to: '/' })
    }
  },
  component: SignInPage,
})

function SignInPage() {
  const { redirect: redirectTo } = Route.useSearch()

  return (
    <div className="flex flex-1 items-center justify-center p-4">
      <SignInForm redirect={redirectTo} />
    </div>
  )
}

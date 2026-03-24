import { createFileRoute, redirect } from '@tanstack/react-router'
import { InitializePage } from '~/features/system/init'
import { useAuthStore } from '~/stores/auth-store'

export const Route = createFileRoute('/_authenticated/system/init/')({
  beforeLoad: () => {
    const user = useAuthStore.getState().auth.user
    if (user?.role !== 'ADMIN') {
      throw redirect({ to: '/' })
    }
  },
  component: InitializePage,
})

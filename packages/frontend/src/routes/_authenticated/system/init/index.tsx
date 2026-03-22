import { createFileRoute, redirect } from '@tanstack/react-router'
import { useAuthStore } from '~/stores/auth-store'
import { InitializePage } from '~/features/system/init'

export const Route = createFileRoute('/_authenticated/system/init/')({
  beforeLoad: () => {
    const user = useAuthStore.getState().auth.user
    if (user?.role !== 'ADMIN') {
      throw redirect({ to: '/' })
    }
  },
  component: InitializePage,
})

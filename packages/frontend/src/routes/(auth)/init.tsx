import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { InitializePage } from '~/features/system/init'
import { useAuthStore } from '~/stores/auth-store'
import { useNodeStore } from '~/stores/node-store'

export const Route = createFileRoute('/(auth)/init')({
  beforeLoad: () => {
    const { auth } = useAuthStore.getState()
    if (!auth.accessToken) {
      throw redirect({ to: '/sign-in' })
    }
    if (auth.user?.role !== 'ADMIN') {
      throw redirect({ to: '/' })
    }
  },
  component: InitPage,
})

function InitPage() {
  const navigate = useNavigate()
  const isInitialized = useNodeStore(s => s.status.isInitialized)

  useEffect(() => {
    if (isInitialized) {
      void navigate({ to: '/' })
    }
  }, [isInitialized, navigate])

  return (
    <div className="flex flex-1 items-center justify-center overflow-auto p-4">
      <InitializePage />
    </div>
  )
}

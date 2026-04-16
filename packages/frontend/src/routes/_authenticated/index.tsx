import { createFileRoute } from '@tanstack/react-router'
import { DashboardView } from '~/views/dashboard'

export const Route = createFileRoute('/_authenticated/')({
  component: DashboardView,
})

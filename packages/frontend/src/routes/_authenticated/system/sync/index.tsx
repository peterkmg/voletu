import { createFileRoute } from '@tanstack/react-router'
import { SyncDashboard } from '~/features/system/sync'

export const Route = createFileRoute('/_authenticated/system/sync/')({
  component: SyncDashboard,
})

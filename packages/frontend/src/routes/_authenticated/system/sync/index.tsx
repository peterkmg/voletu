import { createFileRoute } from '@tanstack/react-router'
import { defineViewRoute } from '~/router/define-view-route'
import { SyncDashboard } from '~/views/system/sync'

export const Route = defineViewRoute(createFileRoute, '/_authenticated/system/sync/')({
  component: SyncDashboard,
})

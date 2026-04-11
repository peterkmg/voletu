import { createFileRoute } from '@tanstack/react-router'
import { defineViewRoute } from '~/router/define-view-route'
import Settings from '~/views/system/settings'

export const Route = defineViewRoute(createFileRoute, '/_authenticated/settings/')({
  component: Settings,
})

import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { BunkeringDetail } from '~/views/outgoing/bunkering'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/outgoing/bunkering/$id')({
  component: BunkeringDetail,
})

import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { ExternalAcceptanceDetail } from '~/views/incoming/external-acceptance'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/incoming/external/$id')({
  component: ExternalAcceptanceDetail,
})

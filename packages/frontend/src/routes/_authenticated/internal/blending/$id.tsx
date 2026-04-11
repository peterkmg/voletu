import { createFileRoute } from '@tanstack/react-router'
import { defineDetailRoute } from '~/router/define-detail-route'
import { BlendingDetail } from '~/views/internal/blending'

export const Route = defineDetailRoute(createFileRoute, '/_authenticated/internal/blending/$id')({
  component: BlendingDetail,
})

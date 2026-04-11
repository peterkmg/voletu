import { createFileRoute } from '@tanstack/react-router'
import { BlendingDetail } from '~/views/internal/blending'

export const Route = createFileRoute('/_authenticated/internal/blending/$id')({
  component: BlendingDetail,
})

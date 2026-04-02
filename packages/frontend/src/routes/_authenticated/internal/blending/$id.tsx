import { createFileRoute } from '@tanstack/react-router'
import { BlendingDetail } from '~/features/internal/blending'

export const Route = createFileRoute('/_authenticated/internal/blending/$id')({
  component: BlendingDetail,
})

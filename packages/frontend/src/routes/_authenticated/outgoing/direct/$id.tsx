import { createFileRoute } from '@tanstack/react-router'
import { DirectDispatchDetail } from '~/views/outgoing/direct-dispatch'

export const Route = createFileRoute('/_authenticated/outgoing/direct/$id')({
  component: DirectDispatchDetail,
})

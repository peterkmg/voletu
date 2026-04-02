import { createFileRoute } from '@tanstack/react-router'
import { BunkeringDetail } from '~/features/outgoing/bunkering'

export const Route = createFileRoute('/_authenticated/outgoing/bunkering/$id')({
  component: BunkeringDetail,
})

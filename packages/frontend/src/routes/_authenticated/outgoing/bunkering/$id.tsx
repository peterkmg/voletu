import { createFileRoute } from '@tanstack/react-router'
import { BunkeringDetail } from '~/views/outgoing/bunkering'

export const Route = createFileRoute('/_authenticated/outgoing/bunkering/$id')({
  component: BunkeringDetail,
})

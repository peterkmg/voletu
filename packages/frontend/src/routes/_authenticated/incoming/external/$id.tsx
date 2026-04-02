import { createFileRoute } from '@tanstack/react-router'
import { ExternalAcceptanceDetail } from '~/features/incoming/external-acceptance'

export const Route = createFileRoute('/_authenticated/incoming/external/$id')({
  component: ExternalAcceptanceDetail,
})

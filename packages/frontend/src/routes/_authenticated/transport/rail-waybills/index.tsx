import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { RailWaybills } from '~/features/transport/rail-waybills'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute(
  '/_authenticated/transport/rail-waybills/',
)({
  validateSearch: searchSchema,
  component: RailWaybills,
})

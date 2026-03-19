import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { PhysicalTransfers } from '~/features/documents/physical-transfer'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/documents/physical-transfer/')({
  validateSearch: searchSchema,
  component: PhysicalTransfers,
})

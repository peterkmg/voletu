import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { OwnershipTransfers } from '~/features/documents/ownership-transfer'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/documents/ownership-transfer/')({
  validateSearch: searchSchema,
  component: OwnershipTransfers,
})

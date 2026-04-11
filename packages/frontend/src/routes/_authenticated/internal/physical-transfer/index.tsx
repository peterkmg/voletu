import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { PhysicalTransferPage } from '~/views/internal/physical-transfer'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/internal/physical-transfer/')({
  validateSearch: searchSchema,
  component: PhysicalTransferPage,
})

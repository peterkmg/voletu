import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { RailReceiptPage } from '~/views/incoming/rail-receipt'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/incoming/rail/')({
  validateSearch: searchSchema,
  component: RailReceiptPage,
})

import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { ReconciliationPage } from '~/features/internal/reconciliation'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/internal/reconciliation/')({
  validateSearch: searchSchema,
  component: ReconciliationPage,
})

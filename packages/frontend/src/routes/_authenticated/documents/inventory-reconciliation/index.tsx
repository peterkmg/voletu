import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { InventoryReconciliation } from '~/features/documents/inventory-reconciliation'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute(
  '/_authenticated/documents/inventory-reconciliation/',
)({
  validateSearch: searchSchema,
  component: InventoryReconciliation,
})

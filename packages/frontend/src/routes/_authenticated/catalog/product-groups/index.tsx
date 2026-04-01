import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { ProductGroups } from '~/features/catalog/product-groups.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/product-groups/')({
  validateSearch: searchSchema,
  component: ProductGroups,
})

import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Products } from '~/features/catalog/products.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/products/')({
  validateSearch: searchSchema,
  component: Products,
})

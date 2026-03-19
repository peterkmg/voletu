import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Warehouses } from '~/features/catalog/warehouses'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/warehouses/')({
  validateSearch: searchSchema,
  component: Warehouses,
})

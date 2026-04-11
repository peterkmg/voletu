import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Companies } from '~/views/catalog/companies.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/companies/')({
  validateSearch: searchSchema,
  component: Companies,
})

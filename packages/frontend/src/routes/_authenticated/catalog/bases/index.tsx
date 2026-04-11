import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Bases } from '~/views/catalog/bases.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/bases/')({
  validateSearch: searchSchema,
  component: Bases,
})

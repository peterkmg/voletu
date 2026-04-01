import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Storages } from '~/features/catalog/storages.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/storages/')({
  validateSearch: searchSchema,
  component: Storages,
})

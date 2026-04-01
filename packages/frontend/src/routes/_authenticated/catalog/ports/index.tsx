import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Ports } from '~/features/catalog/ports.tsx'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/catalog/ports/')({
  validateSearch: searchSchema,
  component: Ports,
})

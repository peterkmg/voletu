import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { DirectDispatchPage } from '~/views/outgoing/direct-dispatch'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/outgoing/direct/')({
  validateSearch: searchSchema,
  component: DirectDispatchPage,
})

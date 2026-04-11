import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { BunkeringPage } from '~/views/outgoing/bunkering'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/outgoing/bunkering/')({
  validateSearch: searchSchema,
  component: BunkeringPage,
})

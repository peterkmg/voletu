import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { AcceptanceDocuments } from '~/features/documents/acceptance'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/documents/acceptance/')({
  validateSearch: searchSchema,
  component: AcceptanceDocuments,
})

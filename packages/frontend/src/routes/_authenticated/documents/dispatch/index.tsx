import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { DispatchDocuments } from '~/features/documents/dispatch'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/documents/dispatch/')({
  validateSearch: searchSchema,
  component: DispatchDocuments,
})

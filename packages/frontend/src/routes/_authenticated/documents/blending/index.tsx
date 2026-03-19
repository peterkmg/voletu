import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { BlendingDocuments } from '~/features/documents/blending'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/documents/blending/')({
  validateSearch: searchSchema,
  component: BlendingDocuments,
})

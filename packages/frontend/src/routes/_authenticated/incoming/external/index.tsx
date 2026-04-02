import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { ExternalAcceptancePage } from '~/features/incoming/external-acceptance'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/incoming/external/')({
  validateSearch: searchSchema,
  component: ExternalAcceptancePage,
})

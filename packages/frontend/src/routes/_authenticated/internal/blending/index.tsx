import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { BlendingPage } from '~/features/internal/blending'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/internal/blending/')({
  validateSearch: searchSchema,
  component: BlendingPage,
})

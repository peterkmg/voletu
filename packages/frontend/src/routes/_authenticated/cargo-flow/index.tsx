import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { CargoFlowPage } from '~/views/cargo-flow'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/cargo-flow/')({
  validateSearch: searchSchema,
  component: CargoFlowPage,
})

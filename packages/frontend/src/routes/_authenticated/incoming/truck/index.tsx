import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { TruckReceiptPage } from '~/views/incoming/truck-receipt'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
  create: z.boolean().optional(),
})

export const Route = createFileRoute('/_authenticated/incoming/truck/')({
  validateSearch: searchSchema,
  component: TruckReceiptPage,
})

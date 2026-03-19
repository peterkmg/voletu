import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Ledger } from '~/features/system/ledger'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/ledger/')({
  validateSearch: searchSchema,
  component: Ledger,
})

import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { Users } from '~/features/system/users'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/system/users/')({
  validateSearch: searchSchema,
  component: Users,
})

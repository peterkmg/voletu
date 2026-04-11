import { z } from 'zod'

export const paginatedListSearchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const createEnabledListSearchSchema = paginatedListSearchSchema.extend({
  create: z.boolean().optional(),
})

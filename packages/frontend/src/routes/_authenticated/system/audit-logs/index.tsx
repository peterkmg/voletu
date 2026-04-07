import { createFileRoute } from '@tanstack/react-router'
import { z } from 'zod'
import { AuditLogsPage } from '~/features/system/audit-logs'

const searchSchema = z.object({
  page: z.number().optional(),
  pageSize: z.number().optional(),
  filter: z.string().optional(),
})

export const Route = createFileRoute('/_authenticated/system/audit-logs/')({
  validateSearch: searchSchema,
  component: AuditLogsPage,
})

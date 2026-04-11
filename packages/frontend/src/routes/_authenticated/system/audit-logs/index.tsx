import { createFileRoute } from '@tanstack/react-router'
import { defineListRoute } from '~/router/define-list-route'
import { paginatedListSearchSchema } from '~/router/search-schemas'
import { AuditLogsPage } from '~/views/system/audit-logs'

export const Route = defineListRoute(createFileRoute, '/_authenticated/system/audit-logs/')({
  validateSearch: paginatedListSearchSchema,
  component: AuditLogsPage,
})

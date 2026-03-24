import type { UserResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'
import { createGlobalFilter, EntityTable } from '~/components/data-table'
import { getUserColumns } from './users-columns'

const route = getRouteApi('/_authenticated/system/users/')
const globalFilterFn = createGlobalFilter<UserResponse>('username', 'fullname')

interface UsersTableProps {
  data: UserResponse[]
}

export function UsersTable({ data }: UsersTableProps) {
  return (
    <EntityTable
      tableId="users"
      data={data}
      getColumns={getUserColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['system', 'common']}
    />
  )
}

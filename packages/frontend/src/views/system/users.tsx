import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { UserResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { PasswordField, SelectField, TextField } from '~/components/forms/form-fields'
import { Badge } from '~/components/ui/badge'
import { Form } from '~/components/ui/form'
import { systemUserCreate, systemUserDelete, systemUserUpdate } from '~/generated/client'
import { systemUserListQueryKey, useSystemUserList } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { defineCrudViews } from '~/lib/define-crud-views'

// --- Columns ---

function getUserColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<UserResponse> }>,
): ColumnDef<UserResponse>[] {
  return [
    textColumn<UserResponse>('username', t('system:users.columns.username'), { sizing: 'capped', maxSize: 180 }),
    textColumn<UserResponse>('fullname', t('system:users.columns.fullname'), { primary: false }),
    {
      accessorKey: 'role',
      header: ({ column }) => (
        <DataTableColumnHeader
          column={column}
          title={t('system:users.columns.role')}
        />
      ),
      minSize: 80,
      maxSize: 150,
      meta: { sizingCategory: 'capped' as const },
      cell: ({ row }) => (
        <Badge variant="outline" className="text-xs">
          {row.getValue('role')}
        </Badge>
      ),
    },
    { ...dateColumn<UserResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<UserResponse>(RowActions, 2),
  ]
}

// --- Table ---

const route = getRouteApi('/_authenticated/system/users/')
const globalFilterFn = createGlobalFilter<UserResponse>('username', 'fullname')

function UsersTable({
  data,
  actions,
  RowActions,
}: {
  data: UserResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<UserResponse> }>
}) {
  return (
    <EntityTable
      tableId="users"
      data={data}
      getColumns={t => getUserColumns(t, RowActions)}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['system', 'common']}
      actions={actions}
    />
  )
}

function useUsersTitle() {
  return useTranslation(['system']).t('system:users.title')
}

// --- Mutate Dialog ---

const USER_ROLE_KEYS = ['ADMIN', 'SENIOR_SUPERVISOR', 'SUPERVISOR', 'OPERATOR'] as const
type UserRoleKey = typeof USER_ROLE_KEYS[number]

const ROLE_LABEL_KEY: Record<UserRoleKey, string> = {
  ADMIN: 'system:users.roles.admin',
  SENIOR_SUPERVISOR: 'system:users.roles.seniorSupervisor',
  SUPERVISOR: 'system:users.roles.supervisor',
  OPERATOR: 'system:users.roles.operator',
}

const userFormSchema = z.object({
  username: z.string().min(1),
  password: z.string().optional().or(z.literal('')),
  fullname: z.string().nullable().optional(),
  roleName: z.enum(USER_ROLE_KEYS),
})

type UserFormValues = z.infer<typeof userFormSchema>

interface UserMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: UserResponse | null
  onCreated?: (id: string) => void
}

interface UserMutationPayload {
  username: string
  password: string | null
  fullname: string | null
  roleName: string
}

function createUserAdapter(
  payload: UserMutationPayload,
  opts?: { headers?: Record<string, string> },
) {
  return systemUserCreate(
    {
      username: payload.username,
      password: payload.password ?? '',
      fullname: payload.fullname,
      roleName: payload.roleName,
    },
    opts,
  )
}

function updateUserAdapter(id: string, payload: UserMutationPayload) {
  return systemUserUpdate(id, payload)
}

export function UserMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: UserMutateDialogProps) {
  const { t } = useTranslation(['system', 'common', 'forms', 'entities'])

  const roleOptions = USER_ROLE_KEYS.map(value => ({
    value,
    label: t(ROLE_LABEL_KEY[value]),
  }))

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: userFormSchema,
    defaultValues: {
      username: '',
      password: '',
      fullname: '',
      roleName: 'OPERATOR',
    },
    mapRowToForm: row => ({
      username: row.username,
      password: '',
      fullname: row.fullname ?? '',
      roleName: (USER_ROLE_KEYS as readonly string[]).includes(row.role)
        ? (row.role as UserFormValues['roleName'])
        : 'OPERATOR',
    }),
    transformPayload: values => ({
      username: values.username,
      password: values.password ? values.password : null,
      fullname: values.fullname || null,
      roleName: values.roleName,
    }),
    createFn: createUserAdapter,
    updateFn: updateUserAdapter,
    queryKey: systemUserListQueryKey(),
    entityLabel: t('system:users.singular'),
    formId: 'user-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('system:users.edit', { defaultValue: 'Edit User' }) : t('system:users.create')}
      formId="user-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="user-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<UserFormValues>
            name="username"
            label={t('system:users.form.username')}
          />
          <PasswordField<UserFormValues>
            name="password"
            label={t('system:users.form.password')}
            description={isUpdate ? t('system:users.form.passwordEditHint', { defaultValue: 'Leave blank to keep current password' }) : undefined}
          />
          <TextField<UserFormValues>
            name="fullname"
            label={t('system:users.form.fullname')}
            nullable
          />
          <SelectField<UserFormValues>
            name="roleName"
            label={t('system:users.form.roleId')}
            options={roleOptions}
            placeholder={t('forms:select.placeholder', { defaultValue: 'Select…' })}
          />
        </form>
      </Form>
    </FormDialog>
  )
}

const usersViewDefinition = defineCrudViews<UserResponse>({
  displayName: 'Users',
  useTitle: useUsersTitle,
  useQuery: useSystemUserList,
  Table: UsersTable,
  MutateDialog: UserMutateDialog,
  deleteDialog: {
    hardDeleteFn: systemUserDelete,
    queryKey: systemUserListQueryKey,
    entityLabel: 'system:users.singular',
    i18nNamespaces: ['common', 'system'],
  },
})

export function Users() {
  return <usersViewDefinition.View />
}

import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { UserResponse } from '~/generated/types'
import { zodResolver } from '@hookform/resolvers/zod'
import { getRouteApi } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { queryClient } from '~/api/query-client'
import { actionsColumn, createGlobalFilter, DataTableColumnHeader, dateColumn, EntityTable, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { PasswordInput } from '~/components/forms/password-input'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from '~/components/ui/sheet'
import { systemUserCreate, systemUserDelete } from '~/generated/client'
import { systemUserListQueryKey, useSystemUserList } from '~/generated/hooks/SystemUserHooks/useSystemUserList'
import { useIdempotencyKey } from '~/hooks/use-idempotency-key'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'
import { extractErrorMessage } from '~/lib/error'

// --- Provider ---

type UsersDialogType = 'create' | 'delete'

const { Provider: UsersProvider, useEntity: useUsers }
  = createEntityProvider<UserResponse, UsersDialogType>('Users')

// --- Row actions ---

const DataTableRowActions = createRowActions<UserResponse>({ useEntity: useUsers, deleteOnly: true })

// --- Columns ---

function getUserColumns(t: TFunction): ColumnDef<UserResponse>[] {
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
    actionsColumn<UserResponse>(DataTableRowActions, 1),
  ]
}

// --- Table ---

const route = getRouteApi('/_authenticated/system/users/')
const globalFilterFn = createGlobalFilter<UserResponse>('username', 'fullname')

function UsersTable({ data, actions }: { data: UserResponse[], actions?: React.ReactNode }) {
  return (
    <EntityTable
      tableId="users"
      data={data}
      getColumns={getUserColumns}
      routeApi={route}
      globalFilterFn={globalFilterFn}
      i18nNamespaces={['system', 'common']}
      actions={actions}
    />
  )
}

// --- Create drawer ---

const userFormSchema = z.object({
  username: z.string().min(1, 'Username is required'),
  password: z.string().min(1, 'Password is required'),
  fullname: z.string().nullable().optional(),
  roleName: z.string().min(1, 'Role is required'),
})

type UserFormValues = z.infer<typeof userFormSchema>

interface UserCreateDrawerProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

function UserCreateDrawer({
  open,
  onOpenChange,
}: UserCreateDrawerProps) {
  const { t } = useTranslation(['system', 'common'])
  const idempotencyKey = useIdempotencyKey()

  const form = useForm<UserFormValues>({
    resolver: zodResolver(userFormSchema),
    defaultValues: {
      username: '',
      password: '',
      fullname: '',
      roleName: '',
    },
  })
  const { reset } = form

  useEffect(() => {
    if (!open) {
      reset({
        username: '',
        password: '',
        fullname: '',
        roleName: '',
      })
    }
  }, [open, reset])

  const onSubmit = async (values: UserFormValues) => {
    try {
      await systemUserCreate({
        username: values.username,
        password: values.password,
        fullname: values.fullname || null,
        roleName: values.roleName,
      }, { headers: { 'Idempotency-Key': idempotencyKey } })
      toast.success(
        t('common:toast.createSuccess', {
          entity: t('system:users.singular'),
        }),
      )
      await queryClient.invalidateQueries({ queryKey: systemUserListQueryKey() })
      onOpenChange(false)
      form.reset()
    }
    catch (err) {
      toast.error(
        extractErrorMessage(err, t('common:toast.error')),
      )
    }
  }

  return (
    <Sheet
      open={open}
      onOpenChange={(v) => {
        onOpenChange(v)
        form.reset()
      }}
    >
      <SheetContent className="flex flex-col">
        <SheetHeader className="text-start">
          <SheetTitle>{t('system:users.create')}</SheetTitle>
          <SheetDescription>{t('system:users.create')}</SheetDescription>
        </SheetHeader>
        <Form {...form}>
          <form
            id="user-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="flex-1 space-y-5 overflow-y-auto px-4"
          >
            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:users.form.username')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:users.form.password')}</FormLabel>
                  <FormControl>
                    <PasswordInput {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="fullname"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:users.form.fullname')}</FormLabel>
                  <FormControl>
                    <Input {...field} value={field.value ?? ''} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="roleName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('system:users.form.roleId')}</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <SheetFooter className="gap-2">
          <SheetClose asChild>
            <Button variant="outline">{t('common:actions.close')}</Button>
          </SheetClose>
          <Button form="user-form" type="submit">
            {t('common:actions.save')}
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  )
}

// --- Delete dialog ---

const UserDeleteDialog = createDeleteDialog({
  useEntity: useUsers,
  hardDeleteFn: systemUserDelete,
  queryKey: systemUserListQueryKey,
  entityLabel: 'system:users.singular',
  i18nNamespaces: ['common', 'system'],
})

// --- Entity dialogs ---

const UsersDialogs = createEntityDialogs({
  useEntity: useUsers,
  MutateDialog: UserCreateDrawer,
  DeleteDialog: UserDeleteDialog,
  supportsUpdate: false,
})

// --- Primary buttons ---

const UsersPrimaryButtons = createPrimaryButtons({ useEntity: useUsers })

// --- Page component ---

export function Users() {
  const { t } = useTranslation(['system'])
  const queryResult = useSystemUserList()

  return (
    <EntityPage
      provider={UsersProvider}
      title={t('system:users.title')}
      queryResult={queryResult}
      primaryButtons={UsersPrimaryButtons}
      table={UsersTable}
      dialogs={UsersDialogs}
    />
  )
}

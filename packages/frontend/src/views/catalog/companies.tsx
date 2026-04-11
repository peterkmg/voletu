import type { ColumnDef, Row } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CompanyResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, StatusBadge, textColumn } from '~/components/data-table'
import { FormDialog } from '~/components/forms/form-dialog'
import { CheckboxField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogCompanyCreate, catalogCompanyHardDelete, catalogCompanySoftDelete, catalogCompanyUpdate } from '~/generated/client'
import { catalogCompanyListQueryKey, useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { companyRoleColors } from '~/lib/badge-colors'
import { defineCrudViews } from '~/lib/define-crud-views'

// --- Columns ---

function getCompanyColumns(
  t: TFunction,
  RowActions: React.ComponentType<{ row: Row<CompanyResponse> }>,
): ColumnDef<CompanyResponse>[] {
  return [
    textColumn<CompanyResponse>('commonName', t('catalog:company.columns.commonName')),
    textColumn<CompanyResponse>('legalName', t('catalog:company.columns.legalName'), { primary: false }),
    {
      id: 'roles',
      header: t('common:table.status'),
      minSize: 90,
      maxSize: 180,
      meta: { sizingCategory: 'capped' as const },
      cell: ({ row }) => {
        const flags = [
          { key: 'isContractor', label: t('catalog:company.columns.isContractor') },
          { key: 'isExporter', label: t('catalog:company.columns.isExporter') },
          { key: 'isManufacturer', label: t('catalog:company.columns.isManufacturer') },
          { key: 'isSender', label: t('catalog:company.columns.isSender') },
        ] as const

        const active = flags.filter(
          f => row.original[f.key as keyof typeof row.original],
        )

        return (
          <div className="flex flex-wrap gap-1">
            {active.map(f => (
              <StatusBadge key={f.key} value={f.key} label={f.label} colorMap={companyRoleColors} className="text-xs" />
            ))}
          </div>
        )
      },
    },
    { ...dateColumn<CompanyResponse>('createdAt', t('common:table.createdAt')), enableHiding: true, meta: { label: t('common:table.createdAt'), sizingCategory: 'capped', requiresRole: 'senior_supervisor' } },
    actionsColumn<CompanyResponse>(RowActions, 2),
  ]
}

// --- Table ---

const companiesRoute = getRouteApi('/_authenticated/catalog/companies/')
const companiesGlobalFilterFn = createGlobalFilter<CompanyResponse>('commonName', 'legalName')

interface CompaniesTableProps {
  data: CompanyResponse[]
  actions?: React.ReactNode
  RowActions: React.ComponentType<{ row: Row<CompanyResponse> }>
}

function CompaniesTable({ data, actions, RowActions }: CompaniesTableProps) {
  return (
    <EntityTable
      tableId="companies"
      data={data}
      getColumns={t => getCompanyColumns(t, RowActions)}
      routeApi={companiesRoute}
      globalFilterFn={companiesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
      actions={actions}
    />
  )
}

function useCompaniesTitle() {
  return useTranslation(['catalog']).t('catalog:company.title')
}

// --- Mutate Dialog ---

const companyFormSchema = z.object({
  commonName: z.string().min(1, 'Common name is required'),
  legalName: z.string().nullable().optional(),
  isContractor: z.boolean(),
  isExporter: z.boolean(),
  isManufacturer: z.boolean(),
  isSender: z.boolean(),
})

type CompanyFormValues = z.infer<typeof companyFormSchema>

interface CompanyMutateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow?: CompanyResponse | null
  onCreated?: (id: string) => void
}

export function CompanyMutateDialog({
  open,
  onOpenChange,
  currentRow,
  onCreated,
}: CompanyMutateDialogProps) {
  const { t } = useTranslation(['catalog', 'common'])

  const { form, isUpdate, handleSubmit, handleOpenChange } = useMutateDialog({
    open,
    onOpenChange,
    currentRow,
    schema: companyFormSchema,
    defaultValues: {
      commonName: '',
      legalName: '',
      isContractor: false,
      isExporter: false,
      isManufacturer: false,
      isSender: false,
    },
    mapRowToForm: row => ({
      commonName: row.commonName,
      legalName: row.legalName ?? '',
      isContractor: row.isContractor,
      isExporter: row.isExporter,
      isManufacturer: row.isManufacturer,
      isSender: row.isSender,
    }),
    transformPayload: values => ({
      ...values,
      legalName: values.legalName || null,
    }),
    createFn: catalogCompanyCreate,
    updateFn: catalogCompanyUpdate,
    queryKey: catalogCompanyListQueryKey(),
    entityLabel: t('catalog:company.singular'),
    formId: 'company-form',
    onCreated,
  })

  return (
    <FormDialog
      open={open}
      onOpenChange={handleOpenChange}
      title={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      description={isUpdate ? t('catalog:company.edit') : t('catalog:company.create')}
      formId="company-form"
      isSubmitting={form.formState.isSubmitting}
    >
      <Form {...form}>
        <form
          id="company-form"
          onSubmit={handleSubmit}
          className="space-y-5"
        >
          <TextField<CompanyFormValues> name="commonName" label={t('catalog:company.form.commonName')} />
          <TextField<CompanyFormValues> name="legalName" label={t('catalog:company.form.legalName')} nullable />
          <div className="space-y-3">
            <CheckboxField<CompanyFormValues> name="isContractor" label={t('catalog:company.form.isContractor')} />
            <CheckboxField<CompanyFormValues> name="isExporter" label={t('catalog:company.form.isExporter')} />
            <CheckboxField<CompanyFormValues> name="isManufacturer" label={t('catalog:company.form.isManufacturer')} />
            <CheckboxField<CompanyFormValues> name="isSender" label={t('catalog:company.form.isSender')} />
          </div>
        </form>
      </Form>
    </FormDialog>
  )
}

const companiesViewDefinition = defineCrudViews<CompanyResponse>({
  displayName: 'Companies',
  useTitle: useCompaniesTitle,
  useQuery: useCatalogCompanyList,
  Table: CompaniesTable,
  MutateDialog: CompanyMutateDialog,
  deleteDialog: {
    hardDeleteFn: catalogCompanyHardDelete,
    softDeleteFn: catalogCompanySoftDelete,
    queryKey: catalogCompanyListQueryKey,
    entityLabel: 'catalog:company.singular',
    i18nNamespaces: ['common', 'catalog'],
  },
})

export function Companies() {
  return <companiesViewDefinition.View />
}

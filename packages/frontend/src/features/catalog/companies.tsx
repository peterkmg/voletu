import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { CompanyResponse } from '~/generated/types'
import { getRouteApi } from '@tanstack/react-router'

import { useTranslation } from 'react-i18next'
import { z } from 'zod'
import { actionsColumn, createGlobalFilter, dateColumn, EntityTable, StatusBadge, textColumn } from '~/components/data-table'
import { EntityPage } from '~/components/entity-page'
import { FormDialog } from '~/components/forms/form-dialog'
import { CheckboxField, TextField } from '~/components/forms/form-fields'
import { Form } from '~/components/ui/form'
import { catalogCompanyCreate, catalogCompanyHardDelete, catalogCompanySoftDelete, catalogCompanyUpdate } from '~/generated/client'
import { catalogCompanyListQueryKey, useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useMutateDialog } from '~/hooks/use-mutate-dialog'
import { companyRoleColors } from '~/lib/badge-colors'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { createEntityProvider } from '~/lib/create-entity-provider'
import { createPrimaryButtons } from '~/lib/create-primary-buttons'
import { createRowActions } from '~/lib/create-row-actions'

// --- Provider ---

type CompaniesDialogType = 'create' | 'update' | 'delete' | 'hard-delete'

const { Provider: CompaniesProvider, useEntity: useCompanies }
  = createEntityProvider<CompanyResponse, CompaniesDialogType>('Companies')

// --- Row Actions ---

const DataTableRowActions = createRowActions<CompanyResponse>({ useEntity: useCompanies })

// --- Columns ---

function getCompanyColumns(t: TFunction): ColumnDef<CompanyResponse>[] {
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
    dateColumn<CompanyResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<CompanyResponse>(DataTableRowActions, 2),
  ]
}

// --- Table ---

const companiesRoute = getRouteApi('/_authenticated/catalog/companies/')
const companiesGlobalFilterFn = createGlobalFilter<CompanyResponse>('commonName', 'legalName')

interface CompaniesTableProps {
  data: CompanyResponse[]
}

function CompaniesTable({ data }: CompaniesTableProps) {
  return (
    <EntityTable
      tableId="companies"
      data={data}
      getColumns={getCompanyColumns}
      routeApi={companiesRoute}
      globalFilterFn={companiesGlobalFilterFn}
      i18nNamespaces={['catalog', 'common']}
    />
  )
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

// --- Delete Dialog ---

const CompanyDeleteDialog = createDeleteDialog({
  useEntity: useCompanies,
  hardDeleteFn: catalogCompanyHardDelete,
  softDeleteFn: catalogCompanySoftDelete,
  queryKey: catalogCompanyListQueryKey,
  entityLabel: 'catalog:company.singular',
  i18nNamespaces: ['common', 'catalog'],
})

// --- Entity Dialogs ---

const CompaniesDialogs = createEntityDialogs({
  useEntity: useCompanies,
  MutateDialog: CompanyMutateDialog,
  DeleteDialog: CompanyDeleteDialog,
})

// --- Primary Buttons ---

const CompaniesPrimaryButtons = createPrimaryButtons({ useEntity: useCompanies })

// --- Page ---

export function Companies() {
  const { t } = useTranslation(['catalog'])
  const queryResult = useCatalogCompanyList()

  return (
    <EntityPage
      provider={CompaniesProvider}
      title={t('catalog:company.title')}
      queryResult={queryResult}
      primaryButtons={CompaniesPrimaryButtons}
      table={CompaniesTable}
      dialogs={CompaniesDialogs}
    />
  )
}

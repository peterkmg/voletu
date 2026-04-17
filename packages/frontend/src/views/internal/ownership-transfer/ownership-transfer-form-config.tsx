/**
 * Ownership-transfer composite form configuration.
 *
 * i18n keys this file depends on (all in the `ownership-transfer` namespace):
 *   ownership-transfer.dialog.title.create
 *   ownership-transfer.dialog.title.edit
 *   ownership-transfer.field.date
 *   ownership-transfer.field.contractorId
 *   ownership-transfer.field.storage
 *   ownership-transfer.field.product
 *   ownership-transfer.field.fromContractor
 *   ownership-transfer.field.toContractor
 *   ownership-transfer.field.amount
 *   ownership-transfer.section.items
 *   ownership-transfer.toast.created
 *   ownership-transfer.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createOwnershipTransferRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateOwnershipTransferCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/ownershipTransferItemCompositeRequestSchema.ts
 *   - Types:
 *       packages/frontend/src/generated/types/CreateOwnershipTransferRequest.ts
 *       packages/frontend/src/generated/types/UpdateOwnershipTransferCompositeRequest.ts
 *       packages/frontend/src/generated/types/OwnershipTransferItemCompositeRequest.ts
 *
 * Both create and update validate min(1) on items: a transfer with zero
 * lines is not useful and the dialog enforces at least one row.
 *
 * Per-row pickers are CONTRACTOR-scoped (`fromContractorId`, `toContractorId`)
 * because ownership transfers move stock between owners while leaving the
 * physical storage location unchanged. The header has no `documentNumber`
 * field (server generates the document identity).
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreateOwnershipTransferRequest } from '~/generated/types/CreateOwnershipTransferRequest'
import type { OwnershipTransferItemCompositeRequest } from '~/generated/types/OwnershipTransferItemCompositeRequest'
import type { UpdateOwnershipTransferCompositeRequest } from '~/generated/types/UpdateOwnershipTransferCompositeRequest'
import { useFormContext } from 'react-hook-form'
import { z } from 'zod/v4'
import {
  ContractorCell,
  formatAmount,
  ProductCell,
  StorageCell,
} from '~/components/composite-form'
import { EntityPickerField } from '~/components/entity-picker'
import {
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { createOwnershipTransferRequestSchema } from '~/generated/zod/createOwnershipTransferRequestSchema'
import { ownershipTransferItemCompositeRequestSchema } from '~/generated/zod/ownershipTransferItemCompositeRequestSchema'
import { updateOwnershipTransferCompositeRequestSchema } from '~/generated/zod/updateOwnershipTransferCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) — the generated composite schema
 * is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able, so we
 * compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const ownershipTransferItemsArraySchema = z
  .array(ownershipTransferItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating an ownership transfer. Layered min(1) is
 * surfaced before the server rejects the payload.
 */
export const ownershipTransferCreateSchema = createOwnershipTransferRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = ownershipTransferItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<OwnershipTransferCreate>

/**
 * Composite schema for updating an ownership transfer. Mirrors the create
 * schema at the field level; the wire type allows optional per-item `id`,
 * which the dialog round-trips for existing rows.
 */
export const ownershipTransferUpdateSchema = updateOwnershipTransferCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = ownershipTransferItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<OwnershipTransferUpdate>

export type OwnershipTransferCreate = CreateOwnershipTransferRequest
export type OwnershipTransferUpdate = UpdateOwnershipTransferCompositeRequest
export type OwnershipTransferItem = OwnershipTransferItemCompositeRequest

// --- Inline picker / input components (small wrappers) ---

interface FieldComponentProps<TForm extends FieldValues> {
  name: Path<TForm>
  placeholder?: string
  disabled?: boolean
}

function ContractorPicker<TForm extends FieldValues>({
  name,
  placeholder,
}: FieldComponentProps<TForm>) {
  const queryResult = useCatalogContractorList()
  return (
    <EntityPickerField<TForm>
      name={name}
      label=""
      placeholder={placeholder}
      queryResult={queryResult}
    />
  )
}

function ProductPicker<TForm extends FieldValues>({
  name,
  placeholder,
}: FieldComponentProps<TForm>) {
  const queryResult = useCatalogProductList() as unknown as UseQueryResult<{
    data?: Array<Record<string, unknown>>
  }>
  return (
    <EntityPickerField<TForm>
      name={name}
      label=""
      placeholder={placeholder}
      queryResult={queryResult}
    />
  )
}

function StoragePicker<TForm extends FieldValues>({
  name,
  placeholder,
}: FieldComponentProps<TForm>) {
  const queryResult = useCatalogStorageList() as unknown as UseQueryResult<{
    data?: Array<Record<string, unknown>>
  }>
  return (
    <EntityPickerField<TForm>
      name={name}
      label=""
      placeholder={placeholder}
      queryResult={queryResult}
    />
  )
}

function DateTimeInput<TForm extends FieldValues>({
  name,
  disabled,
}: FieldComponentProps<TForm>) {
  const { control } = useFormContext<TForm>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <Input
              type="datetime-local"
              disabled={disabled}
              {...field}
              value={(field.value as string | undefined) ?? ''}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// `amount` is a Decimal-as-string on the wire. We keep the value as a
// string in form state and use a numeric-looking input for UX.
function DecimalAmountInput<TForm extends FieldValues>({
  name,
  placeholder,
  disabled,
}: FieldComponentProps<TForm>) {
  const { control } = useFormContext<TForm>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <Input
              type="text"
              inputMode="decimal"
              placeholder={placeholder}
              disabled={disabled}
              {...field}
              value={(field.value as string | undefined) ?? ''}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// --- Header field spec ---
//
// Ownership transfer has no `documentNumber` (server-managed identity) and
// no per-document contractor (per-row from/to contractors instead).

export const ownershipTransferHeaderSpec: HeaderFieldSpec<OwnershipTransferCreate>[] = [
  {
    name: 'date' as Path<OwnershipTransferCreate>,
    labelKey: 'ownership-transfer:field.date',
    component: DateTimeInput,
    required: true,
  },
]

// --- Items column spec (read-only summary) ---

export const ownershipTransferItemColumns: ColumnSpec<OwnershipTransferItem>[] = [
  {
    key: 'storageId',
    labelKey: 'ownership-transfer:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'productId',
    labelKey: 'ownership-transfer:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'fromContractorId',
    labelKey: 'ownership-transfer:field.fromContractor',
    render: value => <ContractorCell id={value as string} />,
  },
  {
    key: 'toContractorId',
    labelKey: 'ownership-transfer:field.toContractor',
    render: value => <ContractorCell id={value as string} />,
  },
  {
    key: 'amount',
    labelKey: 'ownership-transfer:field.amount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

// --- Row drawer field spec ---
//
// `colSpan: 1` for from/to so they render side-by-side on `md+`.
export const ownershipTransferItemFields: RowFieldSpec<OwnershipTransferItem>[] = [
  {
    name: 'storageId',
    labelKey: 'ownership-transfer:field.storage',
    component: StoragePicker,
    required: true,
  },
  {
    name: 'productId',
    labelKey: 'ownership-transfer:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'fromContractorId',
    labelKey: 'ownership-transfer:field.fromContractor',
    component: ContractorPicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'toContractorId',
    labelKey: 'ownership-transfer:field.toContractor',
    component: ContractorPicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'amount',
    labelKey: 'ownership-transfer:field.amount',
    component: DecimalAmountInput,
    required: true,
  },
]

// --- Empty defaults ---

export const emptyOwnershipTransferItem: OwnershipTransferItem = {
  storageId: '',
  productId: '',
  fromContractorId: '',
  toContractorId: '',
  amount: '',
}

export const emptyOwnershipTransferCreate: OwnershipTransferCreate = {
  date: '',
  items: [],
}

// Re-export the row schema so the dialog can pass it to DocItemRowDrawer.
export const ownershipTransferItemSchema = ownershipTransferItemCompositeRequestSchema

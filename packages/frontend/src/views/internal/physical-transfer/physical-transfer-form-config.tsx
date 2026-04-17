/**
 * Physical-storage-transfer composite form configuration.
 *
 * i18n keys this file depends on (all in the `physical-transfer` namespace):
 *   physical-transfer.dialog.title.create
 *   physical-transfer.dialog.title.edit
 *   physical-transfer.field.documentNumber
 *   physical-transfer.field.date
 *   physical-transfer.field.contractorId
 *   physical-transfer.field.startCargoOps
 *   physical-transfer.field.endCargoOps
 *   physical-transfer.field.product
 *   physical-transfer.field.fromStorage
 *   physical-transfer.field.toStorage
 *   physical-transfer.field.amount
 *   physical-transfer.section.items
 *   physical-transfer.toast.created
 *   physical-transfer.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createPhysicalTransferRequestSchema.ts
 *       packages/frontend/src/generated/zod/updatePhysicalTransferCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/physicalTransferItemCompositeRequestSchema.ts
 *   - Types:
 *       packages/frontend/src/generated/types/CreatePhysicalTransferRequest.ts
 *       packages/frontend/src/generated/types/UpdatePhysicalTransferCompositeRequest.ts
 *       packages/frontend/src/generated/types/PhysicalTransferItemCompositeRequest.ts
 *
 * Both create and update validate min(1) on items: a transfer with zero
 * lines is not useful and the dialog enforces at least one row.
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreatePhysicalTransferRequest } from '~/generated/types/CreatePhysicalTransferRequest'
import type { PhysicalTransferItemCompositeRequest } from '~/generated/types/PhysicalTransferItemCompositeRequest'
import type { UpdatePhysicalTransferCompositeRequest } from '~/generated/types/UpdatePhysicalTransferCompositeRequest'
import { useFormContext } from 'react-hook-form'
import { z } from 'zod/v4'
import {
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
import { createPhysicalTransferRequestSchema } from '~/generated/zod/createPhysicalTransferRequestSchema'
import { physicalTransferItemCompositeRequestSchema } from '~/generated/zod/physicalTransferItemCompositeRequestSchema'
import { updatePhysicalTransferCompositeRequestSchema } from '~/generated/zod/updatePhysicalTransferCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) — the generated composite schema
 * is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able, so we
 * compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const physicalTransferItemsArraySchema = z
  .array(physicalTransferItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating a physical transfer. Layered min(1) is
 * surfaced before the server rejects the payload.
 */
export const physicalTransferCreateSchema = createPhysicalTransferRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = physicalTransferItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<PhysicalTransferCreate>

/**
 * Composite schema for updating a physical transfer. Mirrors the create
 * schema at the field level; the wire type allows optional per-item `id`,
 * which the dialog does not yet round-trip (every row treated as insert).
 */
export const physicalTransferUpdateSchema = updatePhysicalTransferCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = physicalTransferItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<PhysicalTransferUpdate>

export type PhysicalTransferCreate = CreatePhysicalTransferRequest
export type PhysicalTransferUpdate = UpdatePhysicalTransferCompositeRequest
export type PhysicalTransferItem = PhysicalTransferItemCompositeRequest

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

function PlainTextInput<TForm extends FieldValues>({
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

export const physicalTransferHeaderSpec: HeaderFieldSpec<PhysicalTransferCreate>[] = [
  {
    name: 'documentNumber' as Path<PhysicalTransferCreate>,
    labelKey: 'physical-transfer:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<PhysicalTransferCreate>,
    labelKey: 'physical-transfer:field.date',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'contractorId' as Path<PhysicalTransferCreate>,
    labelKey: 'physical-transfer:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'startCargoOps' as Path<PhysicalTransferCreate>,
    labelKey: 'physical-transfer:field.startCargoOps',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'endCargoOps' as Path<PhysicalTransferCreate>,
    labelKey: 'physical-transfer:field.endCargoOps',
    component: DateTimeInput,
    required: true,
  },
]

// --- Items column spec (read-only summary) ---

export const physicalTransferItemColumns: ColumnSpec<PhysicalTransferItem>[] = [
  {
    key: 'productId',
    labelKey: 'physical-transfer:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'fromStorageId',
    labelKey: 'physical-transfer:field.fromStorage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'toStorageId',
    labelKey: 'physical-transfer:field.toStorage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'amount',
    labelKey: 'physical-transfer:field.amount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

// --- Row drawer field spec ---
//
// `colSpan: 1` for from/to so they render side-by-side on `md+`.
export const physicalTransferItemFields: RowFieldSpec<PhysicalTransferItem>[] = [
  {
    name: 'productId',
    labelKey: 'physical-transfer:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'fromStorageId',
    labelKey: 'physical-transfer:field.fromStorage',
    component: StoragePicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'toStorageId',
    labelKey: 'physical-transfer:field.toStorage',
    component: StoragePicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'amount',
    labelKey: 'physical-transfer:field.amount',
    component: DecimalAmountInput,
    required: true,
  },
]

// --- Empty defaults ---

export const emptyPhysicalTransferItem: PhysicalTransferItem = {
  productId: '',
  fromStorageId: '',
  toStorageId: '',
  amount: '',
}

export const emptyPhysicalTransferCreate: PhysicalTransferCreate = {
  documentNumber: '',
  date: '',
  contractorId: '',
  startCargoOps: '',
  endCargoOps: '',
  items: [],
}

// Re-export the row schema so the dialog can pass it to DocItemRowDrawer.
export const physicalTransferItemSchema = physicalTransferItemCompositeRequestSchema

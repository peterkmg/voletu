/**
 * Inventory-reconciliation composite form configuration.
 *
 * i18n keys this file depends on (all in the `reconciliation` namespace):
 *   reconciliation.dialog.title.create
 *   reconciliation.dialog.title.edit
 *   reconciliation.field.documentNumber
 *   reconciliation.field.date
 *   reconciliation.field.contractorId
 *   reconciliation.field.warehouseId
 *   reconciliation.field.storage
 *   reconciliation.field.product
 *   reconciliation.field.adjustmentType
 *   reconciliation.field.amount
 *   reconciliation.field.reason
 *   reconciliation.section.adjustments
 *   reconciliation.toast.created
 *   reconciliation.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createInventoryReconciliationCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateInventoryReconciliationCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/inventoryAdjustmentCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/adjustmentTypeSchema.ts
 *           -> adjustmentTypeSchema = z.enum(['SURPLUS', 'LOSS'])
 *   - Types:
 *       packages/frontend/src/generated/types/CreateInventoryReconciliationCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateInventoryReconciliationCompositeRequest.ts
 *       packages/frontend/src/generated/types/InventoryAdjustmentCompositeRequest.ts
 *       packages/frontend/src/generated/types/AdjustmentType.ts
 *
 * Both create and update validate min(1) on `adjustments`: a reconciliation
 * with no adjustments is not useful and the dialog enforces at least one row.
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { AdjustmentType } from '~/generated/types/AdjustmentType'
import type { CreateInventoryReconciliationCompositeRequest } from '~/generated/types/CreateInventoryReconciliationCompositeRequest'
import type { InventoryAdjustmentCompositeRequest } from '~/generated/types/InventoryAdjustmentCompositeRequest'
import type { UpdateInventoryReconciliationCompositeRequest } from '~/generated/types/UpdateInventoryReconciliationCompositeRequest'
import { useFormContext } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { adjustmentTypeEnum } from '~/generated/types/AdjustmentType'
import { createInventoryReconciliationCompositeRequestSchema } from '~/generated/zod/createInventoryReconciliationCompositeRequestSchema'
import { inventoryAdjustmentCompositeRequestSchema } from '~/generated/zod/inventoryAdjustmentCompositeRequestSchema'
import { updateInventoryReconciliationCompositeRequestSchema } from '~/generated/zod/updateInventoryReconciliationCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined adjustments schema with min(1) — the generated composite
 * schema is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able,
 * so composition happens at the row + array level instead. The generated
 * row schema is reused unchanged so any future Kubb refresh propagates to
 * validation.
 */
const reconciliationAdjustmentsArraySchema = z
  .array(inventoryAdjustmentCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating a reconciliation. Layered min(1) is surfaced
 * before the server rejects the payload.
 */
export const reconciliationCreateSchema = createInventoryReconciliationCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const adjustments = (val as { adjustments?: unknown[] }).adjustments
    const result = reconciliationAdjustmentsArraySchema.safeParse(adjustments ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['adjustments', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<ReconciliationCreate>

/**
 * Composite schema for updating a reconciliation. Mirrors the create schema
 * at the field level; the wire type allows optional per-row `id`, which the
 * dialog does not yet round-trip (every row treated as insert).
 */
export const reconciliationUpdateSchema = updateInventoryReconciliationCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const adjustments = (val as { adjustments?: unknown[] }).adjustments
    const result = reconciliationAdjustmentsArraySchema.safeParse(adjustments ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['adjustments', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<ReconciliationUpdate>

export type ReconciliationCreate = CreateInventoryReconciliationCompositeRequest
export type ReconciliationUpdate = UpdateInventoryReconciliationCompositeRequest
export type ReconciliationAdjustment = InventoryAdjustmentCompositeRequest

// --- Inline picker / input components (small wrappers) ---
//
// Each picker / input is rendered inside a `<FormItem>` cell created by
// DocHeaderSection / DocItemRowDrawer, so the wrappers pass an empty label
// (the surrounding FormItem already shows the visible label) and bind
// their own data hook.

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

function WarehousePicker<TForm extends FieldValues>({
  name,
  placeholder,
}: FieldComponentProps<TForm>) {
  const queryResult = useCatalogWarehouseList() as unknown as UseQueryResult<{
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

function StorageBasePicker<TForm extends FieldValues>({
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

function AdjustmentTypeSelect<TForm extends FieldValues>({
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
          <Select
            onValueChange={field.onChange}
            value={field.value as string | undefined}
            disabled={disabled}
          >
            <FormControl>
              <SelectTrigger>
                <SelectValue placeholder={placeholder} />
              </SelectTrigger>
            </FormControl>
            <SelectContent>
              {Object.values(adjustmentTypeEnum).map(value => (
                <SelectItem key={value} value={value}>
                  {value}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <FormMessage />
        </FormItem>
      )}
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

export const reconciliationHeaderSpec: HeaderFieldSpec<ReconciliationCreate>[] = [
  {
    name: 'documentNumber' as Path<ReconciliationCreate>,
    labelKey: 'reconciliation:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<ReconciliationCreate>,
    labelKey: 'reconciliation:field.date',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'contractorId' as Path<ReconciliationCreate>,
    labelKey: 'reconciliation:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'warehouseId' as Path<ReconciliationCreate>,
    labelKey: 'reconciliation:field.warehouseId',
    component: WarehousePicker,
    required: true,
  },
]

// --- Adjustments column spec (read-only summary) ---

/**
 * Renders the adjustment-type enum (`SURPLUS` / `LOSS`) via i18n. The enum
 * value doubles as the i18n leaf key under `reconciliation:adjustmentType.*`.
 * Falls back to the raw enum value if the translation is missing.
 */
function AdjustmentTypeCell({ value }: { value: unknown }) {
  const { t } = useTranslation('reconciliation')
  if (value === null || value === undefined || value === '')
    return null
  const key = `adjustmentType.${String(value)}`
  const translated = t(key)
  // i18next returns the key itself when missing — fall back to the raw value.
  return <span>{translated === key ? String(value) : translated}</span>
}

export const reconciliationAdjustmentColumns: ColumnSpec<ReconciliationAdjustment>[] = [
  {
    key: 'storageId',
    labelKey: 'reconciliation:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'productId',
    labelKey: 'reconciliation:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'adjustmentType',
    labelKey: 'reconciliation:field.adjustmentType',
    render: value => <AdjustmentTypeCell value={value} />,
  },
  {
    key: 'amount',
    labelKey: 'reconciliation:field.amount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
  {
    key: 'reason',
    labelKey: 'reconciliation:field.reason',
  },
]

// --- Row drawer field spec ---

export const reconciliationAdjustmentFields: RowFieldSpec<ReconciliationAdjustment>[] = [
  {
    name: 'storageId',
    labelKey: 'reconciliation:field.storage',
    component: StorageBasePicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'productId',
    labelKey: 'reconciliation:field.product',
    component: ProductPicker,
    required: true,
    colSpan: 1,
  },
  {
    name: 'adjustmentType',
    labelKey: 'reconciliation:field.adjustmentType',
    component: AdjustmentTypeSelect,
    required: true,
    colSpan: 1,
  },
  {
    name: 'amount',
    labelKey: 'reconciliation:field.amount',
    component: DecimalAmountInput,
    required: true,
    colSpan: 1,
  },
  {
    name: 'reason',
    labelKey: 'reconciliation:field.reason',
    component: PlainTextInput,
  },
]

// --- Empty defaults ---

const DEFAULT_ADJUSTMENT_TYPE: AdjustmentType = adjustmentTypeEnum.SURPLUS

export const emptyReconciliationAdjustment: ReconciliationAdjustment = {
  storageId: '',
  productId: '',
  adjustmentType: DEFAULT_ADJUSTMENT_TYPE,
  amount: '',
  reason: '',
}

export const emptyReconciliationCreate: ReconciliationCreate = {
  documentNumber: '',
  date: '',
  contractorId: '',
  warehouseId: '',
  adjustments: [],
}

// Re-export the row schema so the dialog can pass it to DocItemRowDrawer.
export const reconciliationAdjustmentSchema = inventoryAdjustmentCompositeRequestSchema

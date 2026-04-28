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

import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldComponentProps,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { AdjustmentType } from '~/generated/types/AdjustmentType'
import type { CreateInventoryReconciliationCompositeRequest } from '~/generated/types/CreateInventoryReconciliationCompositeRequest'
import type { InventoryAdjustmentCompositeRequest } from '~/generated/types/InventoryAdjustmentCompositeRequest'
import type { UpdateInventoryReconciliationCompositeRequest } from '~/generated/types/UpdateInventoryReconciliationCompositeRequest'
import { useTranslation } from 'react-i18next'
import { z } from 'zod/v4'
import {
  ContractorPicker,
  DateTimeInput,
  DecimalAmountInput,
  formatAmount,
  PlainTextInput,
  ProductCell,
  ProductPicker,
  StorageCell,
  StoragePicker,
  WarehousePicker,
} from '~/components/composite-form'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
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

// `AdjustmentTypeSelect` is form-specific and stays here; everything else
// (text / date / decimal / picker inputs) lives in `composite-form/field-cells`.
function AdjustmentTypeSelect<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Select
      onValueChange={field.onChange}
      value={field.value as string | undefined}
      disabled={disabled}
    >
      <SelectTrigger>
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        {Object.values(adjustmentTypeEnum).map(value => (
          <SelectItem key={value} value={value}>
            {value}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
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
    component: StoragePicker,
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

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

const reconciliationAdjustmentsArraySchema = z
  .array(inventoryAdjustmentCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

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

function AdjustmentTypeCell({ value }: { value: unknown }) {
  const { t } = useTranslation('reconciliation')
  if (value === null || value === undefined || value === '')
    return null
  const key = `adjustmentType.${String(value)}`
  const translated = t(key)

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

export const reconciliationAdjustmentSchema = inventoryAdjustmentCompositeRequestSchema

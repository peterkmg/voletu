import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { TruckWaybillCompositeRequest } from '~/generated/types/TruckWaybillCompositeRequest'
import type { TruckWaybillItemCompositeRequest } from '~/generated/types/TruckWaybillItemCompositeRequest'
import type { UpdateTruckWaybillCompositeRequest } from '~/generated/types/UpdateTruckWaybillCompositeRequest'
import { z } from 'zod/v4'
import {
  BasePicker,
  ContractorPicker,
  DateInput,
  DecimalAmountInput,
  formatAmount,
  PlainTextInput,
  ProductCell,
  ProductPicker,
} from '~/components/composite-form'
import { truckWaybillCompositeRequestSchema } from '~/generated/zod/truckWaybillCompositeRequestSchema'
import { truckWaybillItemCompositeRequestSchema } from '~/generated/zod/truckWaybillItemCompositeRequestSchema'
import { updateTruckWaybillCompositeRequestSchema } from '~/generated/zod/updateTruckWaybillCompositeRequestSchema'

const truckReceiptItemsArraySchema = z
  .array(truckWaybillItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

export const truckReceiptCreateSchema = truckWaybillCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] | null }).items
    const result = truckReceiptItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<TruckReceiptCreate>

export const truckReceiptUpdateSchema = updateTruckWaybillCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = truckReceiptItemsArraySchema.safeParse(items ?? [])
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<TruckReceiptUpdate>

export type TruckReceiptCreate = Omit<TruckWaybillCompositeRequest, 'items' | 'weightDocs'> & {
  items: TruckWaybillItemCompositeRequest[]
}

export type TruckReceiptUpdate = UpdateTruckWaybillCompositeRequest
export type TruckReceiptItem = TruckWaybillItemCompositeRequest

export const truckReceiptHeaderSpec: HeaderFieldSpec<TruckReceiptCreate>[] = [
  {
    name: 'documentNumber' as Path<TruckReceiptCreate>,
    labelKey: 'truck-receipt:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<TruckReceiptCreate>,
    labelKey: 'truck-receipt:field.date',
    component: DateInput,
    required: true,
  },
  {
    name: 'senderId' as Path<TruckReceiptCreate>,
    labelKey: 'truck-receipt:field.senderId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'baseId' as Path<TruckReceiptCreate>,
    labelKey: 'truck-receipt:field.baseId',
    component: BasePicker,
    required: true,
  },
]

export const truckReceiptItemColumns: ColumnSpec<TruckReceiptItem>[] = [
  {
    key: 'productId',
    labelKey: 'truck-receipt:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'declaredAmount',
    labelKey: 'truck-receipt:field.declaredAmount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const truckReceiptItemFields: RowFieldSpec<TruckReceiptItem>[] = [
  {
    name: 'productId',
    labelKey: 'truck-receipt:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'declaredAmount',
    labelKey: 'truck-receipt:field.declaredAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

export const emptyTruckReceiptItem: TruckReceiptItem = {
  productId: '',
  declaredAmount: '',
}

export const emptyTruckReceiptCreate: TruckReceiptCreate = {
  documentNumber: '',
  date: '',
  senderId: '',
  baseId: '',
  items: [],
}

export const truckReceiptItemSchema = truckWaybillItemCompositeRequestSchema

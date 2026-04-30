import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreatePhysicalTransferRequest } from '~/generated/types/CreatePhysicalTransferRequest'
import type { PhysicalTransferItemCompositeRequest } from '~/generated/types/PhysicalTransferItemCompositeRequest'
import type { UpdatePhysicalTransferCompositeRequest } from '~/generated/types/UpdatePhysicalTransferCompositeRequest'
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
} from '~/components/composite-form'
import { createPhysicalTransferRequestSchema } from '~/generated/zod/createPhysicalTransferRequestSchema'
import { physicalTransferItemCompositeRequestSchema } from '~/generated/zod/physicalTransferItemCompositeRequestSchema'
import { updatePhysicalTransferCompositeRequestSchema } from '~/generated/zod/updatePhysicalTransferCompositeRequestSchema'

const physicalTransferItemsArraySchema = z
  .array(physicalTransferItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

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

export const physicalTransferItemSchema = physicalTransferItemCompositeRequestSchema

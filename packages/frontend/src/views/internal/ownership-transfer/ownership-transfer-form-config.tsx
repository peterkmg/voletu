import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreateOwnershipTransferRequest } from '~/generated/types/CreateOwnershipTransferRequest'
import type { OwnershipTransferItemCompositeRequest } from '~/generated/types/OwnershipTransferItemCompositeRequest'
import type { UpdateOwnershipTransferCompositeRequest } from '~/generated/types/UpdateOwnershipTransferCompositeRequest'
import { z } from 'zod/v4'
import {
  ContractorCell,
  ContractorPicker,
  DateTimeInput,
  DecimalAmountInput,
  formatAmount,
  ProductCell,
  ProductPicker,
  StorageCell,
  StoragePicker,
} from '~/components/composite-form'
import { createOwnershipTransferRequestSchema } from '~/generated/zod/createOwnershipTransferRequestSchema'
import { ownershipTransferItemCompositeRequestSchema } from '~/generated/zod/ownershipTransferItemCompositeRequestSchema'
import { updateOwnershipTransferCompositeRequestSchema } from '~/generated/zod/updateOwnershipTransferCompositeRequestSchema'

const ownershipTransferItemsArraySchema = z
  .array(ownershipTransferItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

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

export const ownershipTransferHeaderSpec: HeaderFieldSpec<OwnershipTransferCreate>[] = [
  {
    name: 'date' as Path<OwnershipTransferCreate>,
    labelKey: 'ownership-transfer:field.date',
    component: DateTimeInput,
    required: true,
  },
]

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

export const ownershipTransferItemSchema = ownershipTransferItemCompositeRequestSchema

import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreateDispatchCompositeRequest } from '~/generated/types/CreateDispatchCompositeRequest'
import type { DispatchItemCompositeRequest } from '~/generated/types/DispatchItemCompositeRequest'
import type { DispatchMeasurementCompositeRequest } from '~/generated/types/DispatchMeasurementCompositeRequest'
import type { UpdateDispatchCompositeRequest } from '~/generated/types/UpdateDispatchCompositeRequest'
import { z } from 'zod/v4'
import {
  BasePicker,
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
import { createDispatchCompositeRequestSchema } from '~/generated/zod/createDispatchCompositeRequestSchema'
import { dispatchItemCompositeRequestSchema } from '~/generated/zod/dispatchItemCompositeRequestSchema'
import { dispatchMeasurementCompositeRequestSchema } from '~/generated/zod/dispatchMeasurementCompositeRequestSchema'
import { updateDispatchCompositeRequestSchema } from '~/generated/zod/updateDispatchCompositeRequestSchema'

const dispatchItemsArraySchema = z
  .array(dispatchItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

const dispatchMeasurementsArraySchema = z
  .array(dispatchMeasurementCompositeRequestSchema)
  .optional()

export const directDispatchCreateSchema = createDispatchCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const itemsResult = dispatchItemsArraySchema.safeParse(items ?? [])
    if (!itemsResult.success) {
      for (const issue of itemsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
    const measurements = (val as { storageMeasurements?: unknown[] | null }).storageMeasurements
    if (measurements != null) {
      const mResult = dispatchMeasurementsArraySchema.safeParse(measurements)
      if (!mResult.success) {
        for (const issue of mResult.error.issues) {
          ctx.addIssue({ ...issue, path: ['storageMeasurements', ...(issue.path ?? [])] })
        }
      }
    }
  },
) as unknown as z.ZodType<DirectDispatchCreate>

export const directDispatchUpdateSchema = updateDispatchCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const itemsResult = dispatchItemsArraySchema.safeParse(items ?? [])
    if (!itemsResult.success) {
      for (const issue of itemsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
    const measurements = (val as { storageMeasurements?: unknown[] | null }).storageMeasurements
    if (measurements != null) {
      const mResult = dispatchMeasurementsArraySchema.safeParse(measurements)
      if (!mResult.success) {
        for (const issue of mResult.error.issues) {
          ctx.addIssue({ ...issue, path: ['storageMeasurements', ...(issue.path ?? [])] })
        }
      }
    }
  },
) as unknown as z.ZodType<DirectDispatchUpdate>

export type DirectDispatchCreate = CreateDispatchCompositeRequest
export type DirectDispatchUpdate = UpdateDispatchCompositeRequest
export type DirectDispatchItem = DispatchItemCompositeRequest
export type DirectDispatchMeasurement = DispatchMeasurementCompositeRequest

export const directDispatchHeaderSpec: HeaderFieldSpec<DirectDispatchCreate>[] = [
  {
    name: 'documentNumber' as Path<DirectDispatchCreate>,
    labelKey: 'direct-dispatch:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<DirectDispatchCreate>,
    labelKey: 'direct-dispatch:field.date',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'contractorId' as Path<DirectDispatchCreate>,
    labelKey: 'direct-dispatch:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'destinationBaseId' as Path<DirectDispatchCreate>,
    labelKey: 'direct-dispatch:field.destinationBaseId',
    component: BasePicker,
  },
]

export const directDispatchItemColumns: ColumnSpec<DirectDispatchItem>[] = [
  {
    key: 'productId',
    labelKey: 'direct-dispatch:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'storageId',
    labelKey: 'direct-dispatch:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'dispatchedAmount',
    labelKey: 'direct-dispatch:field.dispatchedAmount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const directDispatchItemFields: RowFieldSpec<DirectDispatchItem>[] = [
  {
    name: 'productId',
    labelKey: 'direct-dispatch:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'storageId',
    labelKey: 'direct-dispatch:field.storage',
    component: StoragePicker,
    required: true,
  },
  {
    name: 'dispatchedAmount',
    labelKey: 'direct-dispatch:field.dispatchedAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

export const directDispatchMeasurementColumns: ColumnSpec<DirectDispatchMeasurement>[] = [
  {
    key: 'storageId',
    labelKey: 'direct-dispatch:field.measurementStorage',
    render: value => <StorageCell id={value as string} />,
  },
  { key: 'beforeHeight', labelKey: 'direct-dispatch:field.beforeHeight', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'beforeVolume', labelKey: 'direct-dispatch:field.beforeVolume', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'beforeDensity', labelKey: 'direct-dispatch:field.beforeDensity', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'beforeMass', labelKey: 'direct-dispatch:field.beforeMass', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'afterHeight', labelKey: 'direct-dispatch:field.afterHeight', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'afterVolume', labelKey: 'direct-dispatch:field.afterVolume', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'afterDensity', labelKey: 'direct-dispatch:field.afterDensity', alignClass: 'text-end', render: value => formatAmount(value) },
  { key: 'afterMass', labelKey: 'direct-dispatch:field.afterMass', alignClass: 'text-end', render: value => formatAmount(value) },
]

export const directDispatchMeasurementFields: RowFieldSpec<DirectDispatchMeasurement>[] = [
  {
    name: 'storageId',
    labelKey: 'direct-dispatch:field.measurementStorage',
    component: StoragePicker,
    required: true,
    colSpan: 2,
  },
  { name: 'beforeHeight', labelKey: 'direct-dispatch:field.beforeHeight', component: DecimalAmountInput, colSpan: 1 },
  { name: 'beforeVolume', labelKey: 'direct-dispatch:field.beforeVolume', component: DecimalAmountInput, colSpan: 1 },
  { name: 'beforeDensity', labelKey: 'direct-dispatch:field.beforeDensity', component: DecimalAmountInput, colSpan: 1 },
  { name: 'beforeMass', labelKey: 'direct-dispatch:field.beforeMass', component: DecimalAmountInput, required: true, colSpan: 1 },
  { name: 'afterHeight', labelKey: 'direct-dispatch:field.afterHeight', component: DecimalAmountInput, colSpan: 1 },
  { name: 'afterVolume', labelKey: 'direct-dispatch:field.afterVolume', component: DecimalAmountInput, colSpan: 1 },
  { name: 'afterDensity', labelKey: 'direct-dispatch:field.afterDensity', component: DecimalAmountInput, colSpan: 1 },
  { name: 'afterMass', labelKey: 'direct-dispatch:field.afterMass', component: DecimalAmountInput, required: true, colSpan: 1 },
]

export const emptyDirectDispatchItem: DirectDispatchItem = {
  productId: '',
  storageId: '',
  dispatchedAmount: '',
}

export const emptyDirectDispatchMeasurement: DirectDispatchMeasurement = {
  storageId: '',
  beforeHeight: null,
  beforeVolume: null,
  beforeDensity: null,
  beforeMass: '',
  afterHeight: null,
  afterVolume: null,
  afterDensity: null,
  afterMass: '',
}

export const emptyDirectDispatchCreate: DirectDispatchCreate = {
  documentNumber: '',
  date: '',
  dispatchPurpose: 'EXTERNAL',
  dispatchMethod: 'VESSEL_TERMINAL',
  contractorId: '',
  destinationBaseId: null,
  receiverEntity: null,
  startCargoOps: null,
  endCargoOps: null,
  bunkerType: null,
  exporterId: null,
  portId: null,
  items: [],
  storageMeasurements: null,
}

export const directDispatchItemSchema = dispatchItemCompositeRequestSchema
export const directDispatchMeasurementSchema = dispatchMeasurementCompositeRequestSchema

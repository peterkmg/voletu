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

export const truckDispatchCreateSchema = createDispatchCompositeRequestSchema.superRefine(
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
) as unknown as z.ZodType<TruckDispatchCreate>

export const truckDispatchUpdateSchema = updateDispatchCompositeRequestSchema.superRefine(
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
) as unknown as z.ZodType<TruckDispatchUpdate>

export type TruckDispatchCreate = CreateDispatchCompositeRequest
export type TruckDispatchUpdate = UpdateDispatchCompositeRequest
export type TruckDispatchItem = DispatchItemCompositeRequest
export type TruckDispatchMeasurement = DispatchMeasurementCompositeRequest

export const truckDispatchHeaderSpec: HeaderFieldSpec<TruckDispatchCreate>[] = [
  {
    name: 'documentNumber' as Path<TruckDispatchCreate>,
    labelKey: 'truck-dispatch:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<TruckDispatchCreate>,
    labelKey: 'truck-dispatch:field.date',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'contractorId' as Path<TruckDispatchCreate>,
    labelKey: 'truck-dispatch:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'destinationBaseId' as Path<TruckDispatchCreate>,
    labelKey: 'truck-dispatch:field.destinationBaseId',
    component: BasePicker,
  },
]

export const truckDispatchItemColumns: ColumnSpec<TruckDispatchItem>[] = [
  {
    key: 'productId',
    labelKey: 'truck-dispatch:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'storageId',
    labelKey: 'truck-dispatch:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'dispatchedAmount',
    labelKey: 'truck-dispatch:field.dispatchedAmount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const truckDispatchItemFields: RowFieldSpec<TruckDispatchItem>[] = [
  {
    name: 'productId',
    labelKey: 'truck-dispatch:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'storageId',
    labelKey: 'truck-dispatch:field.storage',
    component: StoragePicker,
    required: true,
  },
  {
    name: 'dispatchedAmount',
    labelKey: 'truck-dispatch:field.dispatchedAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

export const truckDispatchMeasurementColumns: ColumnSpec<TruckDispatchMeasurement>[] = [
  {
    key: 'storageId',
    labelKey: 'truck-dispatch:field.measurementStorage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'beforeHeight',
    labelKey: 'truck-dispatch:field.beforeHeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'beforeVolume',
    labelKey: 'truck-dispatch:field.beforeVolume',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'beforeDensity',
    labelKey: 'truck-dispatch:field.beforeDensity',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'beforeMass',
    labelKey: 'truck-dispatch:field.beforeMass',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'afterHeight',
    labelKey: 'truck-dispatch:field.afterHeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'afterVolume',
    labelKey: 'truck-dispatch:field.afterVolume',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'afterDensity',
    labelKey: 'truck-dispatch:field.afterDensity',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'afterMass',
    labelKey: 'truck-dispatch:field.afterMass',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
]

export const truckDispatchMeasurementFields: RowFieldSpec<TruckDispatchMeasurement>[] = [
  {
    name: 'storageId',
    labelKey: 'truck-dispatch:field.measurementStorage',
    component: StoragePicker,
    required: true,
    colSpan: 2,
  },
  {
    name: 'beforeHeight',
    labelKey: 'truck-dispatch:field.beforeHeight',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'beforeVolume',
    labelKey: 'truck-dispatch:field.beforeVolume',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'beforeDensity',
    labelKey: 'truck-dispatch:field.beforeDensity',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'beforeMass',
    labelKey: 'truck-dispatch:field.beforeMass',
    component: DecimalAmountInput,
    required: true,
    colSpan: 1,
  },
  {
    name: 'afterHeight',
    labelKey: 'truck-dispatch:field.afterHeight',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'afterVolume',
    labelKey: 'truck-dispatch:field.afterVolume',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'afterDensity',
    labelKey: 'truck-dispatch:field.afterDensity',
    component: DecimalAmountInput,
    colSpan: 1,
  },
  {
    name: 'afterMass',
    labelKey: 'truck-dispatch:field.afterMass',
    component: DecimalAmountInput,
    required: true,
    colSpan: 1,
  },
]

export const emptyTruckDispatchItem: TruckDispatchItem = {
  productId: '',
  storageId: '',
  dispatchedAmount: '',
}

export const emptyTruckDispatchMeasurement: TruckDispatchMeasurement = {
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

export const emptyTruckDispatchCreate: TruckDispatchCreate = {
  documentNumber: '',
  date: '',
  dispatchPurpose: 'EXTERNAL',
  dispatchMethod: 'TRUCK',
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

export const truckDispatchItemSchema = dispatchItemCompositeRequestSchema
export const truckDispatchMeasurementSchema = dispatchMeasurementCompositeRequestSchema

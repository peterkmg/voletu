/**
 * Truck-dispatch composite form configuration.
 *
 * The truck-dispatch document is a `dispatch_document` with `dispatchMethod`
 * hard-locked to `TRUCK` and `dispatchPurpose` hard-locked to `EXTERNAL`.
 * Both discriminators are injected at submit time by the dialog and never
 * surface as user-editable header fields.
 *
 * i18n keys this file depends on (all in the `truck-dispatch` namespace):
 *   truck-dispatch.dialog.title.create
 *   truck-dispatch.dialog.title.edit
 *   truck-dispatch.field.documentNumber
 *   truck-dispatch.field.date
 *   truck-dispatch.field.contractorId
 *   truck-dispatch.field.destinationBaseId
 *   truck-dispatch.field.product
 *   truck-dispatch.field.storage
 *   truck-dispatch.field.dispatchedAmount
 *   truck-dispatch.field.measurementStorage
 *   truck-dispatch.field.{before,after}{Height,Volume,Density,Mass}
 *   truck-dispatch.section.items
 *   truck-dispatch.section.storageMeasurements
 *   truck-dispatch.toast.created
 *   truck-dispatch.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createDispatchCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateDispatchCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/dispatchItemCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/dispatchMeasurementCompositeRequestSchema.ts
 *   - Types:
 *       packages/frontend/src/generated/types/CreateDispatchCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateDispatchCompositeRequest.ts
 *       packages/frontend/src/generated/types/DispatchItemCompositeRequest.ts
 *       packages/frontend/src/generated/types/DispatchMeasurementCompositeRequest.ts
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { CreateDispatchCompositeRequest } from '~/generated/types/CreateDispatchCompositeRequest'
import type { DispatchItemCompositeRequest } from '~/generated/types/DispatchItemCompositeRequest'
import type { DispatchMeasurementCompositeRequest } from '~/generated/types/DispatchMeasurementCompositeRequest'
import type { UpdateDispatchCompositeRequest } from '~/generated/types/UpdateDispatchCompositeRequest'
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
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { createDispatchCompositeRequestSchema } from '~/generated/zod/createDispatchCompositeRequestSchema'
import { dispatchItemCompositeRequestSchema } from '~/generated/zod/dispatchItemCompositeRequestSchema'
import { dispatchMeasurementCompositeRequestSchema } from '~/generated/zod/dispatchMeasurementCompositeRequestSchema'
import { updateDispatchCompositeRequestSchema } from '~/generated/zod/updateDispatchCompositeRequestSchema'
import { toDateTimeLocalValue } from '~/lib/datetime-local'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) — the generated composite schema is
 * a `z.lazy(...).and(z.object(...))` intersection that is not `.extend()`-able,
 * so we compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const dispatchItemsArraySchema = z
  .array(dispatchItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Storage measurements are optional. When present we still validate each row.
 */
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

// --- Inline picker / input components ---

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

function BasePicker<TForm extends FieldValues>({
  name,
  placeholder,
}: FieldComponentProps<TForm>) {
  const queryResult = useCatalogBaseList() as unknown as UseQueryResult<{
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
              value={toDateTimeLocalValue(field.value as string | null | undefined)}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// `dispatchedAmount` and the various Decimal measurement fields are wire-typed
// as `string`. We keep the form value as a string and use a numeric input.
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

// --- Items column spec (read-only summary) ---

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

// --- Items row drawer field spec ---

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

// --- Measurements column spec (read-only summary) ---

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

// --- Measurements row drawer field spec ---

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

// --- Empty defaults ---

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

/**
 * Empty `CreateDispatchCompositeRequest` defaults for the truck variant.
 *
 * The dispatch_method / dispatch_purpose discriminators are hard-coded here
 * (TRUCK / EXTERNAL) and never user-editable; they are also re-applied at
 * submit time as a defensive measure inside the dialog.
 */
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

// Re-export the row schemas so the dialog can pass them to DocItemRowDrawer.
export const truckDispatchItemSchema = dispatchItemCompositeRequestSchema
export const truckDispatchMeasurementSchema = dispatchMeasurementCompositeRequestSchema

/**
 * Direct-dispatch composite form configuration.
 *
 * The direct-dispatch document is a `dispatch_document` with `dispatchMethod`
 * hard-locked to `VESSEL_TERMINAL` (the wire variant for the direct vessel /
 * terminal transfer flow) and `dispatchPurpose` hard-locked to `EXTERNAL`.
 * Both discriminators are injected at submit time by the dialog and never
 * surface as user-editable header fields.
 *
 * i18n keys this file depends on (all in the `direct-dispatch` namespace):
 *   direct-dispatch.dialog.title.create
 *   direct-dispatch.dialog.title.edit
 *   direct-dispatch.field.documentNumber
 *   direct-dispatch.field.date
 *   direct-dispatch.field.contractorId
 *   direct-dispatch.field.destinationBaseId
 *   direct-dispatch.field.product
 *   direct-dispatch.field.storage
 *   direct-dispatch.field.dispatchedAmount
 *   direct-dispatch.field.measurementStorage
 *   direct-dispatch.field.{before,after}{Height,Volume,Density,Mass}
 *   direct-dispatch.section.items
 *   direct-dispatch.section.storageMeasurements
 *   direct-dispatch.toast.created
 *   direct-dispatch.toast.updated
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

// --- Items column / row drawer specs ---

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

// --- Measurements column / row drawer specs ---

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

// --- Empty defaults ---

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

/**
 * Empty `CreateDispatchCompositeRequest` defaults for the direct variant.
 *
 * The dispatch_method / dispatch_purpose discriminators are hard-coded here
 * (VESSEL_TERMINAL / EXTERNAL) and never user-editable; they are also re-applied
 * at submit time as a defensive measure inside the dialog.
 */
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

// Re-export the row schemas so the dialog can pass them to DocItemRowDrawer.
export const directDispatchItemSchema = dispatchItemCompositeRequestSchema
export const directDispatchMeasurementSchema = dispatchMeasurementCompositeRequestSchema

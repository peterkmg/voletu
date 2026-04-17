/**
 * Truck waybill (truck receipt basis) composite form configuration.
 *
 * i18n keys this file depends on (all in the `truck-receipt` namespace):
 *   truck-receipt.dialog.title.create
 *   truck-receipt.dialog.title.edit
 *   truck-receipt.field.documentNumber
 *   truck-receipt.field.date
 *   truck-receipt.field.senderId
 *   truck-receipt.field.baseId
 *   truck-receipt.field.product
 *   truck-receipt.field.declaredAmount
 *   truck-receipt.section.items
 *   truck-receipt.toast.created
 *   truck-receipt.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/truckWaybillCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateTruckWaybillCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/truckWaybillItemCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateTruckWaybillItemCompositeRequestSchema.ts
 *   - Types:
 *       packages/frontend/src/generated/types/TruckWaybillCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateTruckWaybillCompositeRequest.ts
 *       packages/frontend/src/generated/types/TruckWaybillItemCompositeRequest.ts
 *
 * The backend allows `items` to be `Option<Vec<...>>` on create and
 * `Vec<...>` on update. We require min(1) on both create + update to keep
 * the UI flow consistent (a truck receipt with zero items is not useful
 * in practice and the dialog enforces at least one row).
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { TruckWaybillCompositeRequest } from '~/generated/types/TruckWaybillCompositeRequest'
import type { TruckWaybillItemCompositeRequest } from '~/generated/types/TruckWaybillItemCompositeRequest'
import type { UpdateTruckWaybillCompositeRequest } from '~/generated/types/UpdateTruckWaybillCompositeRequest'
import { useFormContext } from 'react-hook-form'
import { z } from 'zod/v4'
import { formatAmount, ProductCell } from '~/components/composite-form'
import { EntityPickerField } from '~/components/entity-picker'
import { FormControl, FormField, FormItem, FormMessage } from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { truckWaybillCompositeRequestSchema } from '~/generated/zod/truckWaybillCompositeRequestSchema'
import { truckWaybillItemCompositeRequestSchema } from '~/generated/zod/truckWaybillItemCompositeRequestSchema'
import { updateTruckWaybillCompositeRequestSchema } from '~/generated/zod/updateTruckWaybillCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) - the generated composite schema
 * is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able, so we
 * compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const truckReceiptItemsArraySchema = z
  .array(truckWaybillItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating a truck receipt.
 *
 * The generated `truckWaybillCompositeRequestSchema` allows `items` to be
 * nullish on the wire, which matches the backend's `Option<Vec<...>>`. The
 * UI is stricter: we require at least one item via a layered superRefine.
 */
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

/**
 * Composite schema for updating a truck receipt.
 *
 * The generated `updateTruckWaybillCompositeRequestSchema` already validates
 * the full update request (header partial + required items list); we layer
 * the same min(1) guardrail to surface the friendly i18n message before the
 * server rejects the payload.
 */
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

/**
 * The wire shape on create includes optional `items`/`weightDocs`. The dialog
 * always submits a full items list (never null), so we narrow `items` to a
 * required array for the form-state type. `weightDocs` is omitted entirely
 * from the dialog (a separate flow attaches them).
 */
export type TruckReceiptCreate = Omit<TruckWaybillCompositeRequest, 'items' | 'weightDocs'> & {
  items: TruckWaybillItemCompositeRequest[]
}

export type TruckReceiptUpdate = UpdateTruckWaybillCompositeRequest
export type TruckReceiptItem = TruckWaybillItemCompositeRequest

// --- Inline picker / input components (small wrappers) ---

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

function DateInput<TForm extends FieldValues>({
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
              type="date"
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

// `declaredAmount` is a Decimal-as-string on the wire. We keep the value
// as a string in form state and use a numeric-looking input for UX.
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

// --- Items column spec (read-only summary) ---

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

// --- Row drawer field spec ---

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

// --- Empty defaults ---

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

// Re-export the row schema so the dialog can pass it to DocItemRowDrawer.
export const truckReceiptItemSchema = truckWaybillItemCompositeRequestSchema

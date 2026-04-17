/**
 * Blending composite form configuration.
 *
 * i18n keys this file depends on (all in the `blending` namespace):
 *   blending.dialog.title.create
 *   blending.dialog.title.edit
 *   blending.field.documentNumber
 *   blending.field.date
 *   blending.field.contractorId
 *   blending.field.targetProductId
 *   blending.field.sourceProduct
 *   blending.field.storage
 *   blending.field.amountUsed
 *   blending.field.producedAmount
 *   blending.section.components
 *   blending.section.results
 *   blending.toast.created
 *   blending.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createBlendingCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/updateBlendingCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/blendingComponentCompositeRequestSchema.ts
 *       packages/frontend/src/generated/zod/blendingResultCompositeRequestSchema.ts
 *   - Types:
 *       packages/frontend/src/generated/types/CreateBlendingCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateBlendingCompositeRequest.ts
 *       packages/frontend/src/generated/types/BlendingComponentCompositeRequest.ts
 *       packages/frontend/src/generated/types/BlendingResultCompositeRequest.ts
 *
 * Both create and update validate min(1) on `components` AND `results`: a
 * blending document with no inputs or no outputs is not useful and the dialog
 * enforces at least one row in each collection.
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { BlendingComponentCompositeRequest } from '~/generated/types/BlendingComponentCompositeRequest'
import type { BlendingResultCompositeRequest } from '~/generated/types/BlendingResultCompositeRequest'
import type { CreateBlendingCompositeRequest } from '~/generated/types/CreateBlendingCompositeRequest'
import type { UpdateBlendingCompositeRequest } from '~/generated/types/UpdateBlendingCompositeRequest'
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
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { blendingComponentCompositeRequestSchema } from '~/generated/zod/blendingComponentCompositeRequestSchema'
import { blendingResultCompositeRequestSchema } from '~/generated/zod/blendingResultCompositeRequestSchema'
import { createBlendingCompositeRequestSchema } from '~/generated/zod/createBlendingCompositeRequestSchema'
import { updateBlendingCompositeRequestSchema } from '~/generated/zod/updateBlendingCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined collection schemas with min(1) — the generated composite schemas
 * are `z.lazy(...).and(z.object(...))` (or plain `z.object`) which are not
 * `.extend()`-able, so we compose at the row + array level instead. The
 * generated row schemas are reused unchanged so any future Kubb refresh
 * propagates to validation.
 */
const blendingComponentsArraySchema = z
  .array(blendingComponentCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

const blendingResultsArraySchema = z
  .array(blendingResultCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating a blending document. Layered min(1) on both
 * `components` and `results` is surfaced before the server rejects the payload.
 */
export const blendingCreateSchema = createBlendingCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const components = (val as { components?: unknown[] }).components
    const componentsResult = blendingComponentsArraySchema.safeParse(components ?? [])
    if (!componentsResult.success) {
      for (const issue of componentsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['components', ...(issue.path ?? [])] })
      }
    }
    const results = (val as { results?: unknown[] }).results
    const resultsResult = blendingResultsArraySchema.safeParse(results ?? [])
    if (!resultsResult.success) {
      for (const issue of resultsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['results', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<BlendingCreate>

/**
 * Composite schema for updating a blending document. Mirrors the create
 * schema at the field level; the wire type allows optional per-row `id`,
 * which the dialog does not yet round-trip (every row treated as insert).
 */
export const blendingUpdateSchema = updateBlendingCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const components = (val as { components?: unknown[] }).components
    const componentsResult = blendingComponentsArraySchema.safeParse(components ?? [])
    if (!componentsResult.success) {
      for (const issue of componentsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['components', ...(issue.path ?? [])] })
      }
    }
    const results = (val as { results?: unknown[] }).results
    const resultsResult = blendingResultsArraySchema.safeParse(results ?? [])
    if (!resultsResult.success) {
      for (const issue of resultsResult.error.issues) {
        ctx.addIssue({ ...issue, path: ['results', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<BlendingUpdate>

export type BlendingCreate = CreateBlendingCompositeRequest
export type BlendingUpdate = UpdateBlendingCompositeRequest
export type BlendingComponent = BlendingComponentCompositeRequest
export type BlendingResult = BlendingResultCompositeRequest

// --- Inline picker / input components (small wrappers) ---
//
// Each picker / input is rendered inside a `<FormItem>` cell created by
// DocHeaderSection / DocItemRowDrawer, so the wrappers pass an empty label
// (the surrounding FormItem already shows the visible label) and bind
// their own data hook.

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

function StorageBasePicker<TForm extends FieldValues>({
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
              value={(field.value as string | undefined) ?? ''}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// `amountUsed` / `producedAmount` are Decimal-as-string on the wire. We keep
// the value as a string in form state and use a numeric-looking input for UX.
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

export const blendingHeaderSpec: HeaderFieldSpec<BlendingCreate>[] = [
  {
    name: 'documentNumber' as Path<BlendingCreate>,
    labelKey: 'blending:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<BlendingCreate>,
    labelKey: 'blending:field.date',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'contractorId' as Path<BlendingCreate>,
    labelKey: 'blending:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'targetProductId' as Path<BlendingCreate>,
    labelKey: 'blending:field.targetProductId',
    component: ProductPicker,
    required: true,
  },
]

// --- Component (input) row column / field specs ---

export const blendingComponentColumns: ColumnSpec<BlendingComponent>[] = [
  {
    key: 'sourceProductId',
    labelKey: 'blending:field.sourceProduct',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'storageId',
    labelKey: 'blending:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'amountUsed',
    labelKey: 'blending:field.amountUsed',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const blendingComponentFields: RowFieldSpec<BlendingComponent>[] = [
  {
    name: 'sourceProductId',
    labelKey: 'blending:field.sourceProduct',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'storageId',
    labelKey: 'blending:field.storage',
    component: StorageBasePicker,
    required: true,
  },
  {
    name: 'amountUsed',
    labelKey: 'blending:field.amountUsed',
    component: DecimalAmountInput,
    required: true,
  },
]

// --- Result (output) row column / field specs ---

export const blendingResultColumns: ColumnSpec<BlendingResult>[] = [
  {
    key: 'storageId',
    labelKey: 'blending:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'producedAmount',
    labelKey: 'blending:field.producedAmount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const blendingResultFields: RowFieldSpec<BlendingResult>[] = [
  {
    name: 'storageId',
    labelKey: 'blending:field.storage',
    component: StorageBasePicker,
    required: true,
  },
  {
    name: 'producedAmount',
    labelKey: 'blending:field.producedAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

// --- Empty defaults ---

export const emptyBlendingComponent: BlendingComponent = {
  sourceProductId: '',
  storageId: '',
  amountUsed: '',
}

export const emptyBlendingResult: BlendingResult = {
  storageId: '',
  producedAmount: '',
}

export const emptyBlendingCreate: BlendingCreate = {
  documentNumber: '',
  date: '',
  contractorId: '',
  targetProductId: '',
  components: [],
  results: [],
}

// Re-export the row schemas so the dialog can pass them to DocItemRowDrawer.
export const blendingComponentSchema = blendingComponentCompositeRequestSchema
export const blendingResultSchema = blendingResultCompositeRequestSchema

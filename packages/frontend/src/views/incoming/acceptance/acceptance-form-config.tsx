/**
 * Acceptance composite form configuration.
 *
 * i18n keys this file depends on (also see forms.* in forms.json):
 *   acceptance.dialog.title.create
 *   acceptance.dialog.title.edit
 *   acceptance.field.documentNumber
 *   acceptance.field.dateAccepted
 *   acceptance.field.arrivalType
 *   acceptance.field.sourceEntity
 *   acceptance.field.contractorId
 *   acceptance.section.items
 *   acceptance.field.product
 *   acceptance.field.storage
 *   acceptance.field.acceptedAmount
 *   acceptance.toast.created
 *   acceptance.toast.updated
 *
 * Generated Kubb artifacts consumed (camelCase field naming throughout):
 *   - Schemas (zod/v4):
 *       packages/frontend/src/generated/zod/createAcceptanceCompositeRequestSchema.ts
 *           -> createAcceptanceCompositeRequestSchema (z.lazy(...).and(z.object(...)) -- NOT .extend()-able)
 *       packages/frontend/src/generated/zod/createAcceptanceRequestSchema.ts
 *           -> createAcceptanceRequestSchema (header object)
 *       packages/frontend/src/generated/zod/acceptanceItemCompositeRequestSchema.ts
 *           -> acceptanceItemCompositeRequestSchema { acceptedAmount: string, productId: uuid, storageId: uuid }
 *       packages/frontend/src/generated/zod/arrivalTypeSchema.ts
 *           -> arrivalTypeSchema = z.enum(['TRUCK','RAIL','EXTERNAL','INITIAL_BALANCE'])
 *   - Types:
 *       packages/frontend/src/generated/types/CreateAcceptanceCompositeRequest.ts
 *       packages/frontend/src/generated/types/AcceptanceItemCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateAcceptanceRequest.ts (header-only update; no composite update yet)
 *
 * IMPORTANT: UpdateAcceptanceCompositeRequestSchema does NOT exist yet.
 * Task 1's backend `UpdateAcceptanceCompositeRequest` DTO has not been
 * picked up by a Kubb regen. Until `pnpm --filter frontend codegen` is
 * re-run after backend changes, edit-mode reuses the create-composite
 * schema. Field rosters happen to match (header + items) so this is
 * acceptable for v1; once the update DTO is generated, swap the
 * `acceptanceUpdateSchema` definition over to the generated one.
 */

import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { AcceptanceItemCompositeRequest } from '~/generated/types/AcceptanceItemCompositeRequest'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { CreateAcceptanceCompositeRequest } from '~/generated/types/CreateAcceptanceCompositeRequest'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { useCatalogCompanyList as useCatalogContractorList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { arrivalTypeEnum } from '~/generated/types/ArrivalType'
import { acceptanceItemCompositeRequestSchema } from '~/generated/zod/acceptanceItemCompositeRequestSchema'
import { createAcceptanceCompositeRequestSchema } from '~/generated/zod/createAcceptanceCompositeRequestSchema'
import { toDateTimeLocalValue } from '~/lib/datetime-local'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) — the generated composite schema
 * is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able, so we
 * compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const acceptanceItemsArraySchema = z
  .array(acceptanceItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Composite schema for creating an acceptance.
 *
 * The generated `createAcceptanceCompositeRequestSchema` already validates
 * the full request; we layer an additional `superRefine` that re-asserts
 * the items.min(1) constraint. We retain the generated schema as the
 * authoritative type contract and cast through `as z.ZodType<...>` to
 * keep the inferred type aligned with the Kubb-generated TS type.
 */
export const acceptanceCreateSchema = createAcceptanceCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = acceptanceItemsArraySchema.safeParse(items)
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
  },
) as unknown as z.ZodType<CreateAcceptanceCompositeRequest>

/**
 * Composite schema for updating an acceptance.
 *
 * TODO(kubb-regen): Once `UpdateAcceptanceCompositeRequest` lands in
 * generated/zod, replace this alias with a wrapper around
 * `updateAcceptanceCompositeRequestSchema`. Until then the create schema
 * is reused; on the wire, the backend update endpoint accepts the same
 * field roster.
 */
export const acceptanceUpdateSchema = acceptanceCreateSchema

export type AcceptanceCreate = CreateAcceptanceCompositeRequest
export type AcceptanceUpdate = CreateAcceptanceCompositeRequest
export type AcceptanceItem = AcceptanceItemCompositeRequest

// --- Inline picker / input components (small wrappers) ---

// `EntityPickerField` requires a `label` and a `queryResult`, while
// `HeaderFieldSpec.component` only passes `{ name, placeholder, disabled }`.
// We render the label inside DocHeaderSection / DocItemRowDrawer already,
// so each wrapper passes an empty label to the field (the surrounding
// FormItem already shows the visible label) and binds its own data hook.

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

function ArrivalTypeSelect<TForm extends FieldValues>({
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
          <Select
            onValueChange={field.onChange}
            value={field.value as string | undefined}
            disabled={disabled}
          >
            <FormControl>
              <SelectTrigger>
                <SelectValue placeholder={placeholder} />
              </SelectTrigger>
            </FormControl>
            <SelectContent>
              {Object.values(arrivalTypeEnum).map(value => (
                <SelectItem key={value} value={value}>
                  {value}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <FormMessage />
        </FormItem>
      )}
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

function NullableTextInput<TForm extends FieldValues>({
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
              value={(field.value as string | null | undefined) ?? ''}
              onChange={e =>
                field.onChange(e.target.value === '' ? null : e.target.value)}
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

// `acceptedAmount` is a Decimal-as-string on the wire. We keep the value
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

export const acceptanceHeaderSpec: HeaderFieldSpec<AcceptanceCreate>[] = [
  {
    name: 'documentNumber' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'dateAccepted' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.dateAccepted',
    component: DateTimeInput,
    required: true,
  },
  {
    name: 'arrivalType' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.arrivalType',
    component: ArrivalTypeSelect,
    required: true,
  },
  {
    name: 'contractorId' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.contractorId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'sourceEntity' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.sourceEntity',
    component: NullableTextInput,
    colSpan: 2,
  },
]

// --- Items column spec (read-only summary) ---

export const acceptanceItemColumns: ColumnSpec<AcceptanceItem>[] = [
  {
    key: 'productId',
    labelKey: 'acceptance:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'storageId',
    labelKey: 'acceptance:field.storage',
    render: value => <StorageCell id={value as string} />,
  },
  {
    key: 'acceptedAmount',
    labelKey: 'acceptance:field.acceptedAmount',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

// --- Row drawer field spec ---

export const acceptanceItemFields: RowFieldSpec<AcceptanceItem>[] = [
  {
    name: 'productId',
    labelKey: 'acceptance:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'storageId',
    labelKey: 'acceptance:field.storage',
    component: StorageBasePicker,
    required: true,
  },
  {
    name: 'acceptedAmount',
    labelKey: 'acceptance:field.acceptedAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

// --- Empty defaults ---

export const emptyAcceptanceItem: AcceptanceItem = {
  productId: '',
  storageId: '',
  acceptedAmount: '',
}

const DEFAULT_ARRIVAL_TYPE: ArrivalType = arrivalTypeEnum.EXTERNAL

export const emptyAcceptanceCreate: AcceptanceCreate = {
  documentNumber: '',
  dateAccepted: '',
  arrivalType: DEFAULT_ARRIVAL_TYPE,
  contractorId: '',
  sourceEntity: null,
  items: [],
}

// Re-export the row schema so the dialog can pass it to DocItemRowDrawer.
export const acceptanceItemSchema = acceptanceItemCompositeRequestSchema

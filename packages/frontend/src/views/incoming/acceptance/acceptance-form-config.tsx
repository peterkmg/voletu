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

import type { FieldValues, Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldComponentProps,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { AcceptanceItemCompositeRequest } from '~/generated/types/AcceptanceItemCompositeRequest'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { CreateAcceptanceCompositeRequest } from '~/generated/types/CreateAcceptanceCompositeRequest'
import { z } from 'zod/v4'
import {
  ContractorPicker,
  DateTimeInput,
  DecimalAmountInput,
  formatAmount,
  NullableTextInput,
  PlainTextInput,
  ProductCell,
  ProductPicker,
  StorageCell,
  StoragePicker,
} from '~/components/composite-form'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { arrivalTypeEnum } from '~/generated/types/ArrivalType'
import { acceptanceItemCompositeRequestSchema } from '~/generated/zod/acceptanceItemCompositeRequestSchema'
import { createAcceptanceCompositeRequestSchema } from '~/generated/zod/createAcceptanceCompositeRequestSchema'

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

// `ArrivalTypeSelect` is form-specific and stays here; everything else (text /
// date / decimal / picker inputs) lives in `composite-form/field-cells`.
function ArrivalTypeSelect<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Select
      onValueChange={field.onChange}
      value={field.value as string | undefined}
      disabled={disabled}
    >
      <SelectTrigger>
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        {Object.values(arrivalTypeEnum).map(value => (
          <SelectItem key={value} value={value}>
            {value}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
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
    component: StoragePicker,
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

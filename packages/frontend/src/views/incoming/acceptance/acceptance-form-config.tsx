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
 *       packages/frontend/src/generated/zod/updateAcceptanceItemCompositeRequestSchema.ts
 *           -> updateAcceptanceItemCompositeRequestSchema { id?: uuid | null, acceptedAmount: string, productId: uuid, storageId: uuid }
 *       packages/frontend/src/generated/zod/updateAcceptanceCompositeRequestSchema.ts
 *           -> updateAcceptanceCompositeRequestSchema (z.lazy(...).and(z.object(...)) -- NOT .extend()-able)
 *       packages/frontend/src/generated/zod/arrivalTypeSchema.ts
 *           -> arrivalTypeSchema = z.enum(['TRUCK','RAIL','EXTERNAL','INITIAL_BALANCE'])
 *   - Types:
 *       packages/frontend/src/generated/types/CreateAcceptanceCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateAcceptanceCompositeRequest.ts
 *       packages/frontend/src/generated/types/AcceptanceItemCompositeRequest.ts
 *       packages/frontend/src/generated/types/UpdateAcceptanceItemCompositeRequest.ts
 */

import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { ArrivalType } from '~/generated/types/ArrivalType'
import type { CreateAcceptanceCompositeRequest } from '~/generated/types/CreateAcceptanceCompositeRequest'
import type { UpdateAcceptanceCompositeRequest } from '~/generated/types/UpdateAcceptanceCompositeRequest'
import type { UpdateAcceptanceItemCompositeRequest } from '~/generated/types/UpdateAcceptanceItemCompositeRequest'
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
import { arrivalTypeEnum } from '~/generated/types/ArrivalType'
import { acceptanceItemCompositeRequestSchema } from '~/generated/zod/acceptanceItemCompositeRequestSchema'
import { createAcceptanceCompositeRequestSchema } from '~/generated/zod/createAcceptanceCompositeRequestSchema'
import { updateAcceptanceCompositeRequestSchema } from '~/generated/zod/updateAcceptanceCompositeRequestSchema'
import { updateAcceptanceItemCompositeRequestSchema } from '~/generated/zod/updateAcceptanceItemCompositeRequestSchema'

// --- Schemas ---

/**
 * Hand-refined items schema with min(1) — the generated composite schema
 * is `z.lazy(...).and(z.object(...))` which is not `.extend()`-able, so we
 * compose at the row + array level instead. The generated row schema is
 * reused unchanged so any future Kubb refresh propagates to validation.
 */
const acceptanceCreateItemsArraySchema = z
  .array(acceptanceItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })
const acceptanceUpdateItemsArraySchema = z
  .array(updateAcceptanceItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

/**
 * Discriminated-union refine for the basis tab.
 *
 * Spec rule 2.4 / 3.1: `arrivalType` selects exactly one basis surface.
 * EXTERNAL allows `sourceEntity` (a free-text label, optional); TRUCK
 * requires `truckWaybillId`; RAIL requires `railWaybillId`. The other
 * two basis fields must be null/absent in each case.
 *
 * Error messages are i18n keys (translations land in Phase 6); the
 * key path is `acceptance:basis.error.*`.
 *
 * Applies identically to create and update schemas. Per spec rule 2.6,
 * arrivalType / FK immutability in edit mode is a UI affordance enforced
 * by the dialog (Task 3.5), not by the schema layer.
 */
function refineBasisDiscriminatedUnion(
  val: unknown,
  ctx: z.RefinementCtx,
) {
  const v = val as {
    arrivalType?: string | null
    truckWaybillId?: string | null
    railWaybillId?: string | null
    sourceEntity?: string | null
  }
  const truckSet = v.truckWaybillId != null
  const railSet = v.railWaybillId != null
  const sourceSet = v.sourceEntity != null

  if (v.arrivalType === 'TRUCK') {
    if (!truckSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['truckWaybillId'],
        message: 'acceptance:basis.error.basisRequired',
      })
    }
    if (railSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['railWaybillId'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
    if (sourceSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['sourceEntity'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
  }
  else if (v.arrivalType === 'RAIL') {
    if (!railSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['railWaybillId'],
        message: 'acceptance:basis.error.basisRequired',
      })
    }
    if (truckSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['truckWaybillId'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
    if (sourceSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['sourceEntity'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
  }
  else if (v.arrivalType === 'EXTERNAL') {
    if (truckSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['truckWaybillId'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
    if (railSet) {
      ctx.addIssue({
        code: 'custom',
        path: ['railWaybillId'],
        message: 'acceptance:basis.error.basisMismatch',
      })
    }
  }
}

/**
 * Composite schema for creating an acceptance.
 *
 * The generated `createAcceptanceCompositeRequestSchema` already validates
 * the full request; we layer an additional `superRefine` that re-asserts
 * the items.min(1) constraint and applies the basis discriminated-union
 * rule. We retain the generated schema as the authoritative type contract
 * and cast through `as z.ZodType<...>` to keep the inferred type aligned
 * with the Kubb-generated TS type.
 */
export const acceptanceCreateSchema = createAcceptanceCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = acceptanceCreateItemsArraySchema.safeParse(items)
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
    refineBasisDiscriminatedUnion(val, ctx)
  },
) as unknown as z.ZodType<CreateAcceptanceCompositeRequest>

/**
 * Composite schema for updating an acceptance.
 *
 * Stateless variant — applies the items + discriminated-union refines but
 * does not enforce arrivalType / basis-FK immutability (spec rule 2.6).
 * Prefer `makeAcceptanceUpdateSchema(original)` whenever the original
 * loaded values are available; this stateless export remains for tests
 * and code paths that do not have access to the original.
 */
export const acceptanceUpdateSchema = updateAcceptanceCompositeRequestSchema.superRefine(
  (val, ctx) => {
    const items = (val as { items?: unknown[] }).items
    const result = acceptanceUpdateItemsArraySchema.safeParse(items)
    if (!result.success) {
      for (const issue of result.error.issues) {
        ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
      }
    }
    refineBasisDiscriminatedUnion(val, ctx)
  },
) as unknown as z.ZodType<UpdateAcceptanceCompositeRequest>

/**
 * Lifecycle-aware update schema factory (spec rule 2.6 — `arrivalType` is
 * immutable post-creation; the basis FK that matched the original
 * `arrivalType` is also immutable). The dialog uses the loaded composite
 * to seed `original`, providing a defense-in-depth guard against a
 * malicious or buggy submission that would change the arrival type.
 */
export interface AcceptanceUpdateOriginal {
  arrivalType: string | null | undefined
  truckWaybillId: string | null | undefined
  railWaybillId: string | null | undefined
  sourceEntity: string | null | undefined
}

export function makeAcceptanceUpdateSchema(
  original: AcceptanceUpdateOriginal,
): z.ZodType<UpdateAcceptanceCompositeRequest> {
  return updateAcceptanceCompositeRequestSchema.superRefine(
    (val, ctx) => {
      const items = (val as { items?: unknown[] }).items
      const result = acceptanceUpdateItemsArraySchema.safeParse(items)
      if (!result.success) {
        for (const issue of result.error.issues) {
          ctx.addIssue({ ...issue, path: ['items', ...(issue.path ?? [])] })
        }
      }
      refineBasisDiscriminatedUnion(val, ctx)

      const v = val as {
        arrivalType?: string | null
        truckWaybillId?: string | null
        railWaybillId?: string | null
      }
      if (v.arrivalType !== original.arrivalType) {
        ctx.addIssue({
          code: 'custom',
          path: ['arrivalType'],
          message: 'acceptance:basis.error.arrivalTypeImmutable',
        })
      }
      if ((v.truckWaybillId ?? null) !== (original.truckWaybillId ?? null)) {
        ctx.addIssue({
          code: 'custom',
          path: ['truckWaybillId'],
          message: 'acceptance:basis.error.basisImmutable',
        })
      }
      if ((v.railWaybillId ?? null) !== (original.railWaybillId ?? null)) {
        ctx.addIssue({
          code: 'custom',
          path: ['railWaybillId'],
          message: 'acceptance:basis.error.basisImmutable',
        })
      }
    },
  ) as unknown as z.ZodType<UpdateAcceptanceCompositeRequest>
}

export type AcceptanceCreate = CreateAcceptanceCompositeRequest
export type AcceptanceUpdate = UpdateAcceptanceCompositeRequest
export type AcceptanceItem = UpdateAcceptanceItemCompositeRequest

// --- Header field spec ---
//
// Only the *common* header fields live here. `arrivalType` (the basis
// discriminator) and the per-tab fields (`sourceEntity` / `truckWaybillId` /
// `railWaybillId`) are owned by `<AcceptanceBasisSection>`, which renders
// the tab strip and the conditional field for the active arrival type.
// This split keeps each surface focused on one concern: common
// metadata here, basis selection there.
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
    name: 'contractorId' as Path<AcceptanceCreate>,
    labelKey: 'acceptance:field.contractorId',
    component: ContractorPicker,
    required: true,
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
  truckWaybillId: null,
  railWaybillId: null,
  items: [],
}

// Re-export the update row schema so edit-mode row saves preserve existing item ids.
// Create-mode rows omit `id`, which the generated update row schema also accepts.
export const acceptanceItemSchema = updateAcceptanceItemCompositeRequestSchema

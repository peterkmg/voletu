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

const acceptanceCreateItemsArraySchema = z
  .array(acceptanceItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })
const acceptanceUpdateItemsArraySchema = z
  .array(updateAcceptanceItemCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

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

export const acceptanceItemSchema = updateAcceptanceItemCompositeRequestSchema

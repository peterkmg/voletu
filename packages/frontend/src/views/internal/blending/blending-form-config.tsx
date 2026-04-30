import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { BlendingComponentCompositeRequest } from '~/generated/types/BlendingComponentCompositeRequest'
import type { BlendingResultCompositeRequest } from '~/generated/types/BlendingResultCompositeRequest'
import type { CreateBlendingCompositeRequest } from '~/generated/types/CreateBlendingCompositeRequest'
import type { UpdateBlendingCompositeRequest } from '~/generated/types/UpdateBlendingCompositeRequest'
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
import { blendingComponentCompositeRequestSchema } from '~/generated/zod/blendingComponentCompositeRequestSchema'
import { blendingResultCompositeRequestSchema } from '~/generated/zod/blendingResultCompositeRequestSchema'
import { createBlendingCompositeRequestSchema } from '~/generated/zod/createBlendingCompositeRequestSchema'
import { updateBlendingCompositeRequestSchema } from '~/generated/zod/updateBlendingCompositeRequestSchema'

const blendingComponentsArraySchema = z
  .array(blendingComponentCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

const blendingResultsArraySchema = z
  .array(blendingResultCompositeRequestSchema)
  .min(1, { message: 'forms.validation.itemsRequired' })

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
    component: StoragePicker,
    required: true,
  },
  {
    name: 'amountUsed',
    labelKey: 'blending:field.amountUsed',
    component: DecimalAmountInput,
    required: true,
  },
]

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
    component: StoragePicker,
    required: true,
  },
  {
    name: 'producedAmount',
    labelKey: 'blending:field.producedAmount',
    component: DecimalAmountInput,
    required: true,
  },
]

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

export const blendingComponentSchema = blendingComponentCompositeRequestSchema
export const blendingResultSchema = blendingResultCompositeRequestSchema

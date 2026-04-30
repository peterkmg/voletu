import type { Path } from 'react-hook-form'
import type {
  ColumnSpec,
  HeaderFieldSpec,
  RowFieldSpec,
} from '~/components/composite-form'
import type { UpdateRailWagonManifestCompositeRequest } from '~/generated/types/UpdateRailWagonManifestCompositeRequest'
import type { UpdateRailWagonMeasurementCompositeRequest } from '~/generated/types/UpdateRailWagonMeasurementCompositeRequest'
import type { UpdateRailWagonWeightCompositeRequest } from '~/generated/types/UpdateRailWagonWeightCompositeRequest'
import type { UpdateRailWaybillCompositeRequest } from '~/generated/types/UpdateRailWaybillCompositeRequest'
import {
  BasePicker,
  ContractorPicker,
  DateInput,
  DecimalInput,
  formatAmount,
  OptionalDecimalInput,
  PlainTextInput,
  ProductCell,
  ProductPicker,
} from '~/components/composite-form'
import { updateRailWagonManifestCompositeRequestSchema } from '~/generated/zod/updateRailWagonManifestCompositeRequestSchema'
import { updateRailWagonMeasurementCompositeRequestSchema } from '~/generated/zod/updateRailWagonMeasurementCompositeRequestSchema'
import { updateRailWagonWeightCompositeRequestSchema } from '~/generated/zod/updateRailWagonWeightCompositeRequestSchema'
import { updateRailWaybillCompositeRequestSchema } from '~/generated/zod/updateRailWaybillCompositeRequestSchema'

export type RailReceiptForm = UpdateRailWaybillCompositeRequest
export type RailReceiptManifest = UpdateRailWagonManifestCompositeRequest
export type RailReceiptMeasurement = UpdateRailWagonMeasurementCompositeRequest
export type RailReceiptWeight = UpdateRailWagonWeightCompositeRequest

export const railReceiptFormSchema = updateRailWaybillCompositeRequestSchema
export const railReceiptManifestSchema = updateRailWagonManifestCompositeRequestSchema
export const railReceiptMeasurementSchema = updateRailWagonMeasurementCompositeRequestSchema
export const railReceiptWeightSchema = updateRailWagonWeightCompositeRequestSchema

export const railReceiptHeaderSpec: HeaderFieldSpec<RailReceiptForm>[] = [
  {
    name: 'documentNumber' as Path<RailReceiptForm>,
    labelKey: 'rail-receipt:field.documentNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'date' as Path<RailReceiptForm>,
    labelKey: 'rail-receipt:field.date',
    component: DateInput,
    required: true,
  },
  {
    name: 'senderId' as Path<RailReceiptForm>,
    labelKey: 'rail-receipt:field.senderId',
    component: ContractorPicker,
    required: true,
  },
  {
    name: 'baseId' as Path<RailReceiptForm>,
    labelKey: 'rail-receipt:field.baseId',
    component: BasePicker,
    required: true,
  },
]

export const railReceiptManifestColumns: ColumnSpec<RailReceiptManifest>[] = [
  { key: 'wagonNumber', labelKey: 'rail-receipt:field.wagonNumber' },
  {
    key: 'productId',
    labelKey: 'rail-receipt:field.product',
    render: value => <ProductCell id={value as string} />,
  },
  {
    key: 'declaredMass',
    labelKey: 'rail-receipt:field.declaredMass',
    alignClass: 'text-end',
    widthClass: 'w-32',
    render: value => formatAmount(value),
  },
]

export const railReceiptManifestFields: RowFieldSpec<RailReceiptManifest>[] = [
  {
    name: 'wagonNumber',
    labelKey: 'rail-receipt:field.wagonNumber',
    component: PlainTextInput,
    required: true,
  },
  {
    name: 'productId',
    labelKey: 'rail-receipt:field.product',
    component: ProductPicker,
    required: true,
  },
  {
    name: 'declaredVolume',
    labelKey: 'rail-receipt:field.declaredVolume',
    component: DecimalInput,
    required: true,
  },
  {
    name: 'declaredDensity',
    labelKey: 'rail-receipt:field.declaredDensity',
    component: DecimalInput,
    required: true,
  },
  {
    name: 'declaredMass',
    labelKey: 'rail-receipt:field.declaredMass',
    component: DecimalInput,
    required: true,
  },
]

export const railReceiptMeasurementColumns: ColumnSpec<RailReceiptMeasurement>[] = [
  {
    key: 'measuredHeight',
    labelKey: 'rail-receipt:field.measuredHeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'labDensity',
    labelKey: 'rail-receipt:field.labDensity',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'calculatedMass',
    labelKey: 'rail-receipt:field.calculatedMass',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
]

export const railReceiptMeasurementFields: RowFieldSpec<RailReceiptMeasurement>[] = [
  {
    name: 'measuredHeight',
    labelKey: 'rail-receipt:field.measuredHeight',
    component: DecimalInput,
    required: true,
  },
  {
    name: 'labDensity',
    labelKey: 'rail-receipt:field.labDensity',
    component: OptionalDecimalInput,
  },
  {
    name: 'calculatedMass',
    labelKey: 'rail-receipt:field.calculatedMass',
    component: DecimalInput,
    required: true,
  },
]

export const railReceiptWeightColumns: ColumnSpec<RailReceiptWeight>[] = [
  {
    key: 'grossWeight',
    labelKey: 'rail-receipt:field.grossWeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'tareWeight',
    labelKey: 'rail-receipt:field.tareWeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
  {
    key: 'netProductWeight',
    labelKey: 'rail-receipt:field.netProductWeight',
    alignClass: 'text-end',
    render: value => formatAmount(value),
  },
]

export const railReceiptWeightFields: RowFieldSpec<RailReceiptWeight>[] = [
  {
    name: 'grossWeight',
    labelKey: 'rail-receipt:field.grossWeight',
    component: DecimalInput,
    required: true,
  },
  {
    name: 'tareWeight',
    labelKey: 'rail-receipt:field.tareWeight',
    component: DecimalInput,
    required: true,
  },
  {
    name: 'netProductWeight',
    labelKey: 'rail-receipt:field.netProductWeight',
    component: DecimalInput,
    required: true,
  },
]

export const emptyRailReceiptMeasurement: RailReceiptMeasurement = {
  id: null,
  measuredHeight: '',
  labDensity: null,
  calculatedMass: '',
}

export const emptyRailReceiptWeight: RailReceiptWeight = {
  id: null,
  grossWeight: '',
  tareWeight: '',
  netProductWeight: '',
}

export const emptyRailReceiptManifest: RailReceiptManifest = {
  id: null,
  wagonNumber: '',
  productId: '',
  declaredVolume: '',
  declaredDensity: '',
  declaredMass: '',
  measurements: [],
  weights: [],
}

export const emptyRailReceiptForm: RailReceiptForm = {
  documentNumber: '',
  date: '',
  senderId: '',
  baseId: '',
  manifests: [],
}

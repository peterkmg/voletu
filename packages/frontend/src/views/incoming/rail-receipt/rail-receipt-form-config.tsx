/**
 * Rail waybill (rail receipt basis) composite form configuration.
 *
 * The rail waybill graph is three levels deep:
 *
 *   RailWaybill
 *   └── manifests: RailWagonManifest[]    (per-wagon manifest, one row per wagon)
 *       ├── measurements: RailWagonMeasurement[]    (one per manifest in the
 *       │                                            current schema; modelled
 *       │                                            as a list to keep room
 *       │                                            for future extension and
 *       │                                            to share <DocItemsTable>)
 *       └── weights: RailWagonWeight[]              (same)
 *
 * The dialog renders the manifests as the outer `<DocItemsTable>` and uses the
 * `rowDrawerExtra` slot to render two compact inner `<DocItemsTable>` instances
 * for the measurements and weights of the manifest currently being edited.
 *
 * i18n keys this file depends on (all in the `rail-receipt` namespace):
 *   rail-receipt.dialog.title.create
 *   rail-receipt.dialog.title.edit
 *   rail-receipt.field.documentNumber
 *   rail-receipt.field.date
 *   rail-receipt.field.senderId
 *   rail-receipt.field.baseId
 *   rail-receipt.field.product
 *   rail-receipt.field.wagonNumber
 *   rail-receipt.field.declaredVolume
 *   rail-receipt.field.declaredDensity
 *   rail-receipt.field.declaredMass
 *   rail-receipt.field.measuredHeight
 *   rail-receipt.field.labDensity
 *   rail-receipt.field.calculatedMass
 *   rail-receipt.field.grossWeight
 *   rail-receipt.field.tareWeight
 *   rail-receipt.field.netProductWeight
 *   rail-receipt.section.manifests
 *   rail-receipt.section.measurements
 *   rail-receipt.section.weights
 *   rail-receipt.toast.created
 *   rail-receipt.toast.updated
 */

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

// --- Form-state types ---
//
// We use the *update* composite shape as the unified form state for both the
// create and edit flows. Reasons:
//   - the update shape carries `id?: string | null` on every row, which is
//     required to round-trip ids during edits (otherwise the backend would
//     interpret each row as an INSERT and delete the originals).
//   - the create wire shape takes the same scalar fields plus a denormalised
//     `wagonNumber` on each child; we synthesise that at submit time from the
//     parent manifest, so the form-state never has to worry about it.
//   - reusing one shape avoids two parallel sets of empty defaults / row specs.

export type RailReceiptForm = UpdateRailWaybillCompositeRequest
export type RailReceiptManifest = UpdateRailWagonManifestCompositeRequest
export type RailReceiptMeasurement = UpdateRailWagonMeasurementCompositeRequest
export type RailReceiptWeight = UpdateRailWagonWeightCompositeRequest

// --- Schemas ---
//
// The generated `updateRailWaybillCompositeRequestSchema` already validates
// the full update payload (header partial + required nested arrays); we reuse
// it directly. For inner row drawers we expose the per-row schemas the
// generator produced, so `<DocItemsTable>` can validate one row at a time.

export const railReceiptFormSchema = updateRailWaybillCompositeRequestSchema
export const railReceiptManifestSchema = updateRailWagonManifestCompositeRequestSchema
export const railReceiptMeasurementSchema = updateRailWagonMeasurementCompositeRequestSchema
export const railReceiptWeightSchema = updateRailWagonWeightCompositeRequestSchema

// Field cells (inputs + entity pickers) come from the shared
// `composite-form/field-cells` module — local wrappers used to nest a second
// `<FormField>`, which produced duplicate validation messages. See
// `HeaderFieldComponentProps` for the contract.

// --- Header field spec (waybill scalars) ---

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

// --- Outer (manifests) column + row spec ---

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

// --- Inner (measurements) column + row spec ---

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

// --- Inner (weights) column + row spec ---

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

// --- Empty defaults ---

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

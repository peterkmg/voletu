import { transportTruckWaybillHardDelete, transportTruckWaybillSoftDelete } from '~/generated/client'
import { transportTruckWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportTruckWaybillList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { TruckWaybillMutateDialog } from './truck-waybill-mutate-dialog'
import { useTruckWaybills } from './truck-waybills-provider'

const TruckWaybillDeleteDialog = createDeleteDialog({
  useEntity: useTruckWaybills,
  hardDeleteFn: transportTruckWaybillHardDelete,
  softDeleteFn: transportTruckWaybillSoftDelete,
  queryKey: transportTruckWaybillListQueryKey,
  entityLabel: 'transport:truck.singular',
  i18nNamespaces: ['common', 'transport'],
})

export const TruckWaybillsDialogs = createEntityDialogs({
  useEntity: useTruckWaybills,
  MutateDialog: TruckWaybillMutateDialog,
  DeleteDialog: TruckWaybillDeleteDialog,
})

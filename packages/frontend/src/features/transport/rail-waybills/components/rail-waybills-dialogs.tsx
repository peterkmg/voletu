import { transportRailWaybillHardDelete, transportRailWaybillSoftDelete } from '~/generated/client'
import { transportRailWaybillListQueryKey } from '~/generated/hooks/DocumentTransportHooks/useTransportRailWaybillList'
import { createDeleteDialog } from '~/lib/create-delete-dialog'
import { createEntityDialogs } from '~/lib/create-entity-dialogs'
import { RailWaybillMutateDialog } from './rail-waybill-mutate-dialog'
import { useRailWaybills } from './rail-waybills-provider'

const RailWaybillDeleteDialog = createDeleteDialog({
  useEntity: useRailWaybills,
  hardDeleteFn: transportRailWaybillHardDelete,
  softDeleteFn: transportRailWaybillSoftDelete,
  queryKey: transportRailWaybillListQueryKey,
  entityLabel: 'transport:rail.singular',
  i18nNamespaces: ['common', 'transport'],
})

export const RailWaybillsDialogs = createEntityDialogs({
  useEntity: useRailWaybills,
  MutateDialog: RailWaybillMutateDialog,
  DeleteDialog: RailWaybillDeleteDialog,
})

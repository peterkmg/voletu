import type { ColumnDef } from '@tanstack/react-table'
import type { TFunction } from 'i18next'
import type { StorageResponse } from '~/generated/types'
import { actionsColumn, dateColumn, numericColumn, resolvedColumn, selectColumn, StatusBadge, textColumn } from '~/components/data-table'
import { entityActiveColors } from '~/lib/badge-colors'
import { createRowActions } from '~/lib/create-row-actions'
import { useStorages } from './storages-provider'

const DataTableRowActions = createRowActions<StorageResponse>({ useEntity: useStorages })

export function getStorageColumns(t: TFunction): ColumnDef<StorageResponse>[] {
  return [
    selectColumn<StorageResponse>(),
    textColumn<StorageResponse>('commonName', t('catalog:storage.columns.commonName')),
    resolvedColumn<StorageResponse>('warehouseId', t('catalog:storage.columns.warehouseId'), 'warehouseIdName'),
    numericColumn<StorageResponse>('capacity', t('catalog:storage.columns.capacity')),
    resolvedColumn<StorageResponse>('productTypeId', t('catalog:storage.columns.productTypeId'), 'productTypeIdName'),
    {
      accessorKey: 'isTypeSpecific',
      header: t('catalog:storage.columns.isTypeSpecific'),
      cell: ({ row }) => {
        const value = row.getValue<boolean>('isTypeSpecific')
        return (
          <StatusBadge
            value={value ? 'active' : 'archived'}
            label={value ? t('common:yes') : t('common:no')}
            colorMap={entityActiveColors}
            className="text-xs"
          />
        )
      },
    },
    dateColumn<StorageResponse>('createdAt', t('common:table.createdAt')),
    actionsColumn<StorageResponse>(DataTableRowActions),
  ]
}

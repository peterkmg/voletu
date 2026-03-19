import type { Row } from '@tanstack/react-table'
import type { PhysicalTransferResponse } from '~/generated/types'
import { MoreHorizontal, Play, Undo2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { usePhysicalTransfer } from './physical-transfer-provider'

interface DataTableRowActionsProps {
  row: Row<PhysicalTransferResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = usePhysicalTransfer()

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <Button
          variant="ghost"
          className="flex h-8 w-8 p-0 data-[state=open]:bg-muted"
        >
          <MoreHorizontal className="size-4" />
          <span className="sr-only">Open menu</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-[160px]">
        {row.original.status === 'DRAFT' && (
          <DropdownMenuItem
            onClick={() => {
              setCurrentRow(row.original)
              setOpen('execute')
            }}
          >
            <Play className="mr-2 size-4" />
            {t('common:actions.execute')}
          </DropdownMenuItem>
        )}
        {row.original.status === 'POSTED' && (
          <DropdownMenuItem
            onClick={() => {
              setCurrentRow(row.original)
              setOpen('revert')
            }}
          >
            <Undo2 className="mr-2 size-4" />
            {t('common:actions.revert')}
          </DropdownMenuItem>
        )}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

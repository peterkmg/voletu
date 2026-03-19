import type { Row } from '@tanstack/react-table'
import type { AcceptanceResponse } from '~/generated/types'
import { Archive, MoreHorizontal, Pencil, PlayCircle, RotateCcw, Trash2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { useAcceptance } from './acceptance-provider'

interface DataTableRowActionsProps {
  row: Row<AcceptanceResponse>
}

export function DataTableRowActions({ row }: DataTableRowActionsProps) {
  const { t } = useTranslation(['common'])
  const { setOpen, setCurrentRow } = useAcceptance()

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
        <DropdownMenuItem
          onClick={() => {
            setCurrentRow(row.original)
            setOpen('update')
          }}
        >
          <Pencil className="mr-2 size-4" />
          {t('common:actions.edit')}
        </DropdownMenuItem>
        {row.original.status === 'DRAFT' && (
          <DropdownMenuItem
            onClick={() => {
              setCurrentRow(row.original)
              setOpen('execute')
            }}
          >
            <PlayCircle className="mr-2 size-4" />
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
            <RotateCcw className="mr-2 size-4" />
            {t('common:actions.revert')}
          </DropdownMenuItem>
        )}
        <DropdownMenuSeparator />
        <DropdownMenuItem
          onClick={() => {
            setCurrentRow(row.original)
            setOpen('delete')
          }}
        >
          <Archive className="mr-2 size-4" />
          {t('common:actions.softDelete')}
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => {
            setCurrentRow(row.original)
            setOpen('hard-delete')
          }}
          className="text-destructive focus:text-destructive"
        >
          <Trash2 className="mr-2 size-4" />
          {t('common:actions.hardDelete')}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

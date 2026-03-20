import type { EntityItem } from './entity-picker-combobox'
import { Check, Plus, Search } from 'lucide-react'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import { Input } from '~/components/ui/input'
import { ScrollArea } from '~/components/ui/scroll-area'
import { cn } from '~/lib/utils'

interface EntityPickerDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  items: EntityItem[]
  value: string | null | undefined
  onSelect: (id: string) => void
  title: string
  allowCreate?: boolean
  onCreateNew?: () => void
}

export function EntityPickerDialog({
  open,
  onOpenChange,
  items,
  value,
  onSelect,
  title,
  allowCreate,
  onCreateNew,
}: EntityPickerDialogProps) {
  const { t } = useTranslation('common')
  const [search, setSearch] = useState('')
  const [selectedId, setSelectedId] = useState<string | null>(value ?? null)

  const filtered = useMemo(() => {
    if (!search)
      return items
    const lower = search.toLowerCase()
    return items.filter(
      item =>
        item.label.toLowerCase().includes(lower)
        || item.secondary?.toLowerCase().includes(lower),
    )
  }, [items, search])

  const handleConfirm = () => {
    if (selectedId) {
      onSelect(selectedId)
      onOpenChange(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[80vh] flex-col sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <div className="relative">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder={`${t('actions.search')}...`}
            value={search}
            onChange={e => setSearch(e.target.value)}
            className="pl-9"
          />
        </div>
        <ScrollArea className="flex-1 -mx-1 max-h-[50vh]">
          <div className="space-y-0.5 px-1">
            {filtered.length === 0 && (
              <p className="py-6 text-center text-sm text-muted-foreground">
                {t('table.noResults')}
              </p>
            )}
            {filtered.map(item => (
              <button
                key={item.id}
                type="button"
                className={cn(
                  'flex w-full items-center gap-3 rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-accent',
                  selectedId === item.id && 'bg-accent',
                )}
                onClick={() => setSelectedId(item.id)}
                onDoubleClick={() => {
                  onSelect(item.id)
                  onOpenChange(false)
                }}
              >
                <Check
                  className={cn(
                    'h-4 w-4 shrink-0',
                    selectedId === item.id ? 'opacity-100' : 'opacity-0',
                  )}
                />
                <div className="flex flex-col min-w-0">
                  <span className="truncate">{item.label}</span>
                  {item.secondary && (
                    <span className="truncate text-xs text-muted-foreground">{item.secondary}</span>
                  )}
                </div>
              </button>
            ))}
          </div>
        </ScrollArea>
        <DialogFooter className="flex-row justify-between sm:justify-between">
          <div>
            {allowCreate && onCreateNew && (
              <Button variant="outline" size="sm" onClick={onCreateNew}>
                <Plus className="mr-1 h-4 w-4" />
                {t('actions.create')}
              </Button>
            )}
          </div>
          <div className="flex gap-2">
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              {t('actions.cancel')}
            </Button>
            <Button onClick={handleConfirm} disabled={!selectedId}>
              {t('actions.confirm')}
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

import type { EntityItem } from './entity-picker-combobox'
import { Check, Plus } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '~/components/ui/command'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
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

/**
 * Browse-all dialog reachable via the combobox's "View all…" item. Uses the
 * exact same cmdk primitives as the inline dropdown so typography, hover
 * state, keyboard navigation, and search behaviour stay identical between
 * the two surfaces.
 */
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
  const { t } = useTranslation('forms')
  const [selectedId, setSelectedId] = useState<string | null>(value ?? null)

  const handleConfirm = () => {
    if (selectedId) {
      onSelect(selectedId)
      onOpenChange(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="flex max-h-[80vh] flex-col gap-4 sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <Command className="overflow-hidden rounded-md border">
          <CommandInput placeholder={t('picker.search')} />
          <CommandList className="max-h-[50vh]">
            <CommandEmpty>{t('picker.noResults')}</CommandEmpty>
            <CommandGroup>
              {items.map(item => (
                <CommandItem
                  key={item.id}
                  value={`${item.label} ${item.secondary ?? ''}`}
                  onSelect={() => setSelectedId(item.id)}
                  data-active={selectedId === item.id || undefined}
                  className="data-[active]:bg-accent data-[active]:text-accent-foreground"
                >
                  <Check
                    className={cn(
                      'mr-2 h-4 w-4 shrink-0',
                      selectedId === item.id ? 'opacity-100' : 'opacity-0',
                    )}
                  />
                  <div className="flex min-w-0 flex-col">
                    <span className="truncate">{item.label}</span>
                    {item.secondary && (
                      <span className="truncate text-xs text-muted-foreground">{item.secondary}</span>
                    )}
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
        <DialogFooter className="sm:justify-between">
          <div>
            {allowCreate && onCreateNew && (
              <Button variant="outline" size="sm" onClick={onCreateNew}>
                <Plus className="mr-1 h-4 w-4" />
                {t('picker.create')}
              </Button>
            )}
          </div>
          <div className="flex gap-2">
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              {t('picker.cancel')}
            </Button>
            <Button onClick={handleConfirm} disabled={!selectedId}>
              {t('picker.confirm')}
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

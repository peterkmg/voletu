import { Check, ChevronsUpDown, Search, X } from 'lucide-react'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '~/components/ui/command'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '~/components/ui/popover'
import { cn } from '~/lib/utils'

export interface EntityItem {
  id: string
  label: string
  secondary?: string
}

interface EntityPickerComboboxProps {
  items: EntityItem[]
  value: string | null | undefined
  onChange: (value: string | null) => void
  placeholder?: string
  nullable?: boolean
  onBrowseAll?: () => void
  disabled?: boolean
  className?: string
}

export function EntityPickerCombobox({
  items,
  value,
  onChange,
  placeholder,
  nullable = false,
  onBrowseAll,
  disabled,
  className,
}: EntityPickerComboboxProps) {
  const { t } = useTranslation('forms')
  const [open, setOpen] = useState(false)

  const selectedItem = useMemo(
    () => items.find(item => item.id === value),
    [items, value],
  )

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          disabled={disabled}
          className={cn(
            'w-full justify-between font-normal',
            !selectedItem && 'text-muted-foreground',
            className,
          )}
        >
          <span className="truncate">
            {selectedItem ? selectedItem.label : (placeholder ?? t('picker.placeholder'))}
          </span>
          <div className="flex items-center gap-1">
            {nullable && selectedItem && (
              <X
                className="h-3.5 w-3.5 shrink-0 opacity-50 hover:opacity-100"
                onClick={(e) => {
                  e.stopPropagation()
                  onChange(null)
                }}
              />
            )}
            <ChevronsUpDown className="h-4 w-4 shrink-0 opacity-50" />
          </div>
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[--radix-popover-trigger-width] p-0" align="start">
        <Command>
          <CommandInput placeholder={t('picker.search')} />
          <CommandList>
            <CommandEmpty>{t('picker.noResults')}</CommandEmpty>
            <CommandGroup>
              {items.map(item => (
                <CommandItem
                  key={item.id}
                  value={item.label}
                  onSelect={() => {
                    onChange(item.id)
                    setOpen(false)
                  }}
                >
                  <Check
                    className={cn(
                      'mr-2 h-4 w-4 shrink-0',
                      value === item.id ? 'opacity-100' : 'opacity-0',
                    )}
                  />
                  <div className="flex flex-col">
                    <span>{item.label}</span>
                    {item.secondary && (
                      <span className="text-xs text-muted-foreground">{item.secondary}</span>
                    )}
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
            {onBrowseAll && (
              <>
                <CommandSeparator />
                <CommandGroup>
                  <CommandItem
                    onSelect={() => {
                      onBrowseAll()
                      setOpen(false)
                    }}
                  >
                    <Search className="mr-2 h-4 w-4" />
                    {t('picker.viewAll')}
                  </CommandItem>
                </CommandGroup>
              </>
            )}
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  )
}

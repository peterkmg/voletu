import type { UseQueryResult } from '@tanstack/react-query'
import type { EntityItem } from './entity-picker-types'
import { useMemo, useState } from 'react'
import { EntityPickerCombobox } from './entity-picker-combobox'
import { EntityPickerDialog } from './entity-picker-dialog'

export interface EntityPickerInputProps {
  value: string | null | undefined
  onChange: (value: string | null) => void
  queryResult: UseQueryResult<{ data?: Array<Record<string, unknown>> }>
  displayField?: string
  secondaryField?: string
  nullable?: boolean
  dialogTitle?: string
  allowCreate?: boolean
  createDialog?: React.ComponentType<{
    open: boolean
    onOpenChange: (open: boolean) => void
    onCreated?: (id: string) => void
  }>
  filter?: (item: Record<string, unknown>) => boolean
  placeholder?: string
  disabled?: boolean
  className?: string
}

export function EntityPickerInput({
  value,
  onChange,
  queryResult,
  displayField = 'commonName',
  secondaryField,
  nullable = false,
  dialogTitle,
  allowCreate = false,
  createDialog: CreateDialog,
  filter,
  placeholder,
  disabled,
  className,
}: EntityPickerInputProps) {
  const [browseOpen, setBrowseOpen] = useState(false)
  const [createOpen, setCreateOpen] = useState(false)

  const items: EntityItem[] = useMemo(() => {
    const data = queryResult.data?.data ?? []
    const filtered = filter ? data.filter(filter) : data
    return filtered.map(item => ({
      id: item.id as string,
      label: (item[displayField] as string) ?? (item.id as string),
      secondary: secondaryField ? (item[secondaryField] as string) : undefined,
    }))
  }, [queryResult.data, displayField, secondaryField, filter])

  return (
    <>
      <EntityPickerCombobox
        items={items}
        value={value}
        onChange={(val) => {
          onChange(nullable ? val : (val ?? null))
        }}
        placeholder={placeholder}
        nullable={nullable}
        onBrowseAll={() => setBrowseOpen(true)}
        disabled={disabled}
        className={className}
      />
      <EntityPickerDialog
        open={browseOpen}
        onOpenChange={setBrowseOpen}
        items={items}
        value={value ?? null}
        onSelect={(id) => {
          onChange(id)
          setBrowseOpen(false)
        }}
        title={dialogTitle ?? placeholder ?? ''}
        allowCreate={allowCreate && !!CreateDialog}
        onCreateNew={() => setCreateOpen(true)}
      />
      {CreateDialog && (
        <CreateDialog
          open={createOpen}
          onOpenChange={setCreateOpen}
          onCreated={(id) => {
            onChange(id)
            setCreateOpen(false)
            setBrowseOpen(false)
          }}
        />
      )}
    </>
  )
}

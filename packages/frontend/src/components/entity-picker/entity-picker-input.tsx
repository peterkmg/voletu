import type { UseQueryResult } from '@tanstack/react-query'
import type { EntityItem } from './entity-picker-combobox'
import { useMemo, useState } from 'react'
import { EntityPickerCombobox } from './entity-picker-combobox'
import { EntityPickerDialog } from './entity-picker-dialog'

export interface EntityPickerInputProps {
  /** Current selected id (or empty string / null for unselected). */
  value: string | null | undefined
  /** Called with the new id on selection, or `null` when cleared. */
  onChange: (value: string | null) => void
  /** Query hook result — must return `{ data: { data: Array<{ id, [displayField]: string }> } }`. */
  queryResult: UseQueryResult<{ data?: Array<Record<string, unknown>> }>
  /** Field shown as the primary label. Defaults to `'commonName'`. */
  displayField?: string
  /** Optional secondary display field. */
  secondaryField?: string
  /** Whether the field accepts the empty selection. */
  nullable?: boolean
  /** Title for the browse-all dialog. */
  dialogTitle?: string
  /** Allow inline creation via `createDialog`. */
  allowCreate?: boolean
  /** Component opened as a nested dialog when the user picks "Create". */
  createDialog?: React.ComponentType<{
    open: boolean
    onOpenChange: (open: boolean) => void
    onCreated?: (id: string) => void
  }>
  /** Optional client-side filter applied to raw rows before mapping. */
  filter?: (item: Record<string, unknown>) => boolean
  /** Placeholder text for the trigger button. */
  placeholder?: string
  /** Disable the picker. */
  disabled?: boolean
  /** Extra class names on the trigger. */
  className?: string
}

/**
 * Controlled entity picker — renders the combobox + browse-all dialog without
 * any RHF integration. Use `<EntityPickerField>` if you want a fully-wrapped
 * RHF field with label and validation message; use this primitive when the
 * surrounding context already provides those (e.g. `<DocHeaderSection>`).
 */
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

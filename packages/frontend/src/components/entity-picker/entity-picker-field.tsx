import type { UseQueryResult } from '@tanstack/react-query'
import type { ControllerRenderProps, FieldValues, Path } from 'react-hook-form'
import type { EntityItem } from './entity-picker-combobox'
import { useCallback, useMemo, useState } from 'react'
import { useFormContext } from 'react-hook-form'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { EntityPickerCombobox } from './entity-picker-combobox'
import { EntityPickerDialog } from './entity-picker-dialog'

interface EntityPickerFieldProps<TForm extends FieldValues> {
  name: Path<TForm>
  label: string
  placeholder?: string
  /** Query hook result - must return { data: { data: Array<{ id: string, [displayField]: string }> } } */
  queryResult: UseQueryResult<{ data?: Array<Record<string, unknown>> }>
  /** Field to display as the main label. Defaults to 'commonName'. */
  displayField?: string
  /** Optional secondary display field */
  secondaryField?: string
  /** Whether the field is nullable (optional FK) */
  nullable?: boolean
  /** Title for the browse-all dialog */
  dialogTitle?: string
  /** Whether to allow creating new entities inline */
  allowCreate?: boolean
  /** Component to render for inline creation (will be opened as a nested dialog) */
  createDialog?: React.ComponentType<{
    open: boolean
    onOpenChange: (open: boolean) => void
    onCreated?: (id: string) => void
  }>
  /**
   * Optional client-side filter predicate applied to the raw data rows
   * returned by `queryResult` before they are mapped into picker items.
   * Use this to narrow the picker contents per-document (e.g. only active
   * companies, only contractors of a given role) without wiring a separate
   * data hook. The predicate runs against the raw row shape returned by
   * the underlying API client.
   *
   * NOTE: server-side filtering still belongs at the call site by passing
   * query parameters into the data hook itself (`useCatalog*List(params)`).
   * This prop is the lightweight escape hatch for the common case of
   * narrowing an already-fetched list.
   */
  filter?: (item: Record<string, unknown>) => boolean
}

export function EntityPickerField<TForm extends FieldValues>({
  name,
  label,
  placeholder,
  queryResult,
  displayField = 'commonName',
  secondaryField,
  nullable = false,
  dialogTitle,
  allowCreate = false,
  createDialog: CreateDialog,
  filter,
}: EntityPickerFieldProps<TForm>) {
  const form = useFormContext<TForm>()
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

  const handleCreated = useCallback(
    (id: string) => {
      form.setValue(name, id as never)
      setCreateOpen(false)
      setBrowseOpen(false)
    },
    [form, name],
  )

  return (
    <>
      <FormField
        control={form.control}
        name={name}
        render={({ field }: { field: ControllerRenderProps<TForm, Path<TForm>> }) => (
          <FormItem>
            <FormLabel>{label}</FormLabel>
            <FormControl>
              <EntityPickerCombobox
                items={items}
                value={field.value as string | null}
                onChange={(val) => {
                  field.onChange(nullable ? (val ?? '') : (val ?? ''))
                }}
                placeholder={placeholder}
                nullable={nullable}
                onBrowseAll={() => setBrowseOpen(true)}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />
      <EntityPickerDialog
        open={browseOpen}
        onOpenChange={setBrowseOpen}
        items={items}
        value={form.getValues(name) as string}
        onSelect={(id) => {
          form.setValue(name, id as never)
          setBrowseOpen(false)
        }}
        title={dialogTitle ?? label}
        allowCreate={allowCreate && !!CreateDialog}
        onCreateNew={() => setCreateOpen(true)}
      />
      {CreateDialog && (
        <CreateDialog
          open={createOpen}
          onOpenChange={setCreateOpen}
          onCreated={handleCreated}
        />
      )}
    </>
  )
}

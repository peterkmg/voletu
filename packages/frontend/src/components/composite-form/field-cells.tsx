import type { UseQueryResult } from '@tanstack/react-query'
import type { FieldValues } from 'react-hook-form'
import type { HeaderFieldComponentProps } from './types'
import { EntityPickerInput } from '~/components/entity-picker'
import { Input } from '~/components/ui/input'
import { useCatalogBaseList } from '~/generated/hooks/CatalogHooks/useCatalogBaseList'
import { useCatalogCompanyList } from '~/generated/hooks/CatalogHooks/useCatalogCompanyList'
import { useCatalogProductList } from '~/generated/hooks/CatalogHooks/useCatalogProductList'
import { useCatalogStorageList } from '~/generated/hooks/CatalogHooks/useCatalogStorageList'
import { useCatalogWarehouseList } from '~/generated/hooks/CatalogHooks/useCatalogWarehouseList'
import { toDateTimeLocalValue } from '~/lib/datetime-local'

/*
 * Shared bare-input components used as `HeaderFieldSpec.component` in every
 * `*-form-config.tsx`. They render only the input element — labels, required
 * asterisks, FormControl wiring, and validation messages are provided by the
 * outer renderer (`<DocHeaderSection>` / `<DocItemRowDrawer>`). NEVER wrap
 * these in a `<FormField>` again; doing so produces nested Controllers and
 * duplicate validation messages (see `HeaderFieldComponentProps` docs).
 */

type CatalogQuery = UseQueryResult<{ data?: Array<Record<string, unknown>> }>

/* ──────────────────────────────────────────────────────────────────────── */
/*  Text & decimal inputs                                                   */
/* ──────────────────────────────────────────────────────────────────────── */

export function PlainTextInput<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Input
      type="text"
      placeholder={placeholder}
      disabled={disabled}
      {...field}
      value={(field.value as string | undefined) ?? ''}
    />
  )
}

export function NullableTextInput<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Input
      type="text"
      placeholder={placeholder}
      disabled={disabled}
      name={field.name}
      ref={field.ref}
      onBlur={field.onBlur}
      value={(field.value as string | null | undefined) ?? ''}
      onChange={(event) => {
        const raw = event.target.value
        field.onChange(raw === '' ? null : raw)
      }}
    />
  )
}

export function DateInput<TForm extends FieldValues>({
  field,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  const value = (field.value as string | undefined) ?? ''
  return (
    <Input
      type="date"
      disabled={disabled}
      data-empty={value === '' || undefined}
      {...field}
      value={value}
    />
  )
}

export function DateTimeInput<TForm extends FieldValues>({
  field,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  // Wire values may arrive as ISO with seconds/milliseconds; the native
  // `datetime-local` control only renders `YYYY-MM-DDTHH:MM`.
  const value = toDateTimeLocalValue(field.value as string | null | undefined)
  return (
    <Input
      type="datetime-local"
      disabled={disabled}
      data-empty={value === '' || undefined}
      {...field}
      value={value}
    />
  )
}

export function DecimalInput<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Input
      type="text"
      inputMode="decimal"
      placeholder={placeholder}
      disabled={disabled}
      {...field}
      value={(field.value as string | undefined) ?? ''}
    />
  )
}

export function OptionalDecimalInput<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
}: HeaderFieldComponentProps<TForm>) {
  return (
    <Input
      type="text"
      inputMode="decimal"
      placeholder={placeholder}
      disabled={disabled}
      name={field.name}
      ref={field.ref}
      onBlur={field.onBlur}
      value={(field.value as string | null | undefined) ?? ''}
      onChange={(event) => {
        const raw = event.target.value
        field.onChange(raw === '' ? null : raw)
      }}
    />
  )
}

/* Aliases — some callers used `DecimalAmountInput` for the same semantics. */
export const DecimalAmountInput = DecimalInput

/* ──────────────────────────────────────────────────────────────────────── */
/*  Entity-picker inputs (each binds a specific catalog list hook)          */
/* ──────────────────────────────────────────────────────────────────────── */

interface PickerCellProps<TForm extends FieldValues> extends HeaderFieldComponentProps<TForm> {
  queryResult: CatalogQuery
}

function PickerCell<TForm extends FieldValues>({
  field,
  placeholder,
  disabled,
  queryResult,
}: PickerCellProps<TForm>) {
  return (
    <EntityPickerInput
      queryResult={queryResult}
      value={field.value as string | null | undefined}
      onChange={val => field.onChange(val ?? '')}
      placeholder={placeholder}
      disabled={disabled}
    />
  )
}

export function ContractorPicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogCompanyList() as CatalogQuery} />
}

export function ProductPicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogProductList() as CatalogQuery} />
}

/** Storage *base* (depot) — bound to `useCatalogBaseList`. */
export function BasePicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogBaseList() as CatalogQuery} />
}

/** Storage *cell* (tank) — bound to `useCatalogStorageList`. */
export function StoragePicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogStorageList() as CatalogQuery} />
}

export function WarehousePicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogWarehouseList() as CatalogQuery} />
}

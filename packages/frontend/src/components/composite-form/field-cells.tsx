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

type CatalogQuery = UseQueryResult<{ data?: Array<Record<string, unknown>> }>

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
  const value = toDateTimeLocalValue(field.value as string | null | undefined)
  return (
    <Input
      type="datetime-local"
      disabled={disabled}
      data-empty={value === '' || undefined}
      name={field.name}
      ref={field.ref}
      onBlur={field.onBlur}
      value={value}
      onChange={(event) => {
        const raw = event.target.value
        if (raw === '') {
          field.onChange('')
          return
        }
        const parsed = new Date(raw)
        field.onChange(Number.isNaN(parsed.getTime()) ? raw : parsed.toISOString())
      }}
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

export const DecimalAmountInput = DecimalInput

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

export function BasePicker<TForm extends FieldValues>(
  props: HeaderFieldComponentProps<TForm>,
) {
  return <PickerCell {...props} queryResult={useCatalogBaseList() as CatalogQuery} />
}

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

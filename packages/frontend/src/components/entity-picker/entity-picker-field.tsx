import type { ControllerRenderProps, FieldValues, Path } from 'react-hook-form'
import type { EntityPickerInputProps } from './entity-picker-input'
import { useFormContext } from 'react-hook-form'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { EntityPickerInput } from './entity-picker-input'

interface EntityPickerFieldProps<TForm extends FieldValues>
  extends Omit<EntityPickerInputProps, 'value' | 'onChange'> {
  name: Path<TForm>
  label: string
}

export function EntityPickerField<TForm extends FieldValues>({
  name,
  label,
  ...inputProps
}: EntityPickerFieldProps<TForm>) {
  const form = useFormContext<TForm>()
  return (
    <FormField
      control={form.control}
      name={name}
      render={({ field }: { field: ControllerRenderProps<TForm, Path<TForm>> }) => (
        <FormItem>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <EntityPickerInput
              {...inputProps}
              value={field.value as string | null}
              onChange={(val) => {
                field.onChange(inputProps.nullable ? (val ?? '') : (val ?? ''))
              }}
            />
          </FormControl>
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

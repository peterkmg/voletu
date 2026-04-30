import type { InputHTMLAttributes } from 'react'
import type { FieldPath, FieldValues } from 'react-hook-form'
import { useFormContext } from 'react-hook-form'
import { PasswordInput } from '~/components/forms/password-input'
import { Checkbox } from '~/components/ui/checkbox'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { RadioGroup, RadioGroupItem } from '~/components/ui/radio-group'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { Switch } from '~/components/ui/switch'
import { Textarea } from '~/components/ui/textarea'
import { ToggleGroup, ToggleGroupItem } from '~/components/ui/toggle-group'
import { cn } from '~/lib/utils'

interface BaseFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> {
  name: TName
  label: string
  description?: string
  className?: string
}

// ── TextField ─────────────────────────────────

interface TextFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  type?: InputHTMLAttributes<HTMLInputElement>['type']
  placeholder?: string
  nullable?: boolean
}

export function TextField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  type = 'text',
  placeholder,
  nullable = false,
  description,
  className,
}: TextFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <Input
              type={type}
              placeholder={placeholder}
              {...field}
              value={nullable ? (field.value ?? '') : field.value}
            />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── SelectField ───────────────────────────────

interface SelectOption {
  value: string
  label: string
}

interface SelectFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  options: readonly SelectOption[]
  placeholder?: string
}

export function SelectField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  options,
  placeholder,
  description,
  className,
}: SelectFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <Select onValueChange={field.onChange} value={field.value}>
            <FormControl>
              <SelectTrigger>
                <SelectValue placeholder={placeholder} />
              </SelectTrigger>
            </FormControl>
            <SelectContent>
              {options.map(opt => (
                <SelectItem key={opt.value} value={opt.value}>
                  {opt.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── CheckboxField ─────────────────────────────

export function CheckboxField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  description,
  className,
}: BaseFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={cn('flex items-center gap-2', className)}>
          <FormControl>
            <Checkbox
              checked={field.value}
              onCheckedChange={field.onChange}
            />
          </FormControl>
          <FormLabel className="!mt-0">{label}</FormLabel>
          {description && <FormDescription>{description}</FormDescription>}
        </FormItem>
      )}
    />
  )
}

// ── TextAreaField ─────────────────────────────

interface TextAreaFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  placeholder?: string
  rows?: number
  disabled?: boolean
}

export function TextAreaField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  placeholder,
  rows,
  disabled,
  description,
  className,
}: TextAreaFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <Textarea
              placeholder={placeholder}
              rows={rows}
              disabled={disabled}
              {...field}
            />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── NumberField ───────────────────────────────

interface NumberFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  placeholder?: string
  min?: number
  max?: number
  step?: number
  disabled?: boolean
}

export function NumberField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  placeholder,
  min,
  max,
  step,
  disabled,
  description,
  className,
}: NumberFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <Input
              type="number"
              placeholder={placeholder}
              min={min}
              max={max}
              step={step}
              disabled={disabled}
              {...field}
            />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── DatePickerField ──────────────────────────

interface DatePickerFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  includeTime?: boolean
  disabled?: boolean
}

export function DatePickerField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  includeTime = false,
  disabled,
  description,
  className,
}: DatePickerFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <Input
              type={includeTime ? 'datetime-local' : 'date'}
              disabled={disabled}
              {...field}
            />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── PasswordField ────────────────────────────

interface PasswordFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  placeholder?: string
  autoComplete?: string
}

export function PasswordField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  placeholder,
  autoComplete = 'new-password',
  description,
  className,
}: PasswordFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <PasswordInput
              placeholder={placeholder}
              autoComplete={autoComplete}
              {...field}
            />
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

// ── SwitchField ──────────────────────────────

export function SwitchField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  description,
  className,
}: BaseFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem
          className={cn(
            'flex items-center justify-between rounded-lg border bg-card/40 px-4 py-3',
            className,
          )}
        >
          <div className="space-y-0.5">
            <FormLabel className="!mt-0 cursor-pointer">{label}</FormLabel>
            {description && <FormDescription>{description}</FormDescription>}
          </div>
          <FormControl>
            <Switch
              checked={!!field.value}
              onCheckedChange={field.onChange}
              onBlur={field.onBlur}
              ref={field.ref}
            />
          </FormControl>
        </FormItem>
      )}
    />
  )
}

// ── ToggleChipGroupField ─────────────────────

interface ToggleChipOption<TName extends string> {
  /** The name of the boolean RHF field this chip toggles. */
  name: TName
  label: string
  /** Tailwind classes applied when active (e.g. from a colorMap). */
  activeClassName?: string
}

interface ToggleChipGroupFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> {
  label: string
  description?: string
  className?: string
  options: ToggleChipOption<TName>[]
}

/**
 * Renders a group of related boolean fields as a single labeled set of toggleable chips.
 * Each chip writes back to its own RHF path (preserves existing schema shapes).
 */
export function ToggleChipGroupField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  label,
  description,
  className,
  options,
}: ToggleChipGroupFieldProps<TFieldValues, TName>) {
  const { watch, setValue } = useFormContext<TFieldValues>()
  const activeNames = options
    .filter(opt => !!watch(opt.name))
    .map(opt => opt.name)

  const handleValueChange = (next: string[]) => {
    const nextSet = new Set(next)
    for (const opt of options) {
      const isActive = nextSet.has(opt.name)
      setValue(opt.name, isActive as never, {
        shouldDirty: true,
        shouldTouch: true,
      })
    }
  }

  const activeSet = new Set(activeNames)

  return (
    <FormItem className={cn('space-y-2', className)}>
      <FormLabel>{label}</FormLabel>
      <FormControl>
        <ToggleGroup
          type="multiple"
          value={activeNames}
          onValueChange={handleValueChange}
          variant="outline"
          className="flex flex-wrap gap-2"
        >
          {options.map((opt) => {
            const isActive = activeSet.has(opt.name)
            return (
              <ToggleGroupItem
                key={opt.name}
                value={opt.name}
                aria-label={opt.label}
                className={cn(
                  'h-8 rounded-full border px-3 text-xs font-medium',
                  isActive
                    ? opt.activeClassName ?? 'bg-primary text-primary-foreground border-transparent'
                    : 'bg-transparent text-muted-foreground hover:text-foreground',
                )}
              >
                {opt.label}
              </ToggleGroupItem>
            )
          })}
        </ToggleGroup>
      </FormControl>
      {description && <FormDescription>{description}</FormDescription>}
    </FormItem>
  )
}

// ── RadioField ───────────────────────────────

interface RadioFieldProps<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> extends BaseFieldProps<TFieldValues, TName> {
  options: { label: string, value: string }[]
  disabled?: boolean
}

export function RadioField<
  TFieldValues extends FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>({
  name,
  label,
  options,
  disabled,
  description,
  className,
}: RadioFieldProps<TFieldValues, TName>) {
  const { control } = useFormContext<TFieldValues>()
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className={className}>
          <FormLabel>{label}</FormLabel>
          <FormControl>
            <RadioGroup
              onValueChange={field.onChange}
              value={field.value}
              disabled={disabled}
            >
              {options.map(opt => (
                <FormItem
                  key={opt.value}
                  className="flex items-center gap-2"
                >
                  <FormControl>
                    <RadioGroupItem value={opt.value} />
                  </FormControl>
                  <FormLabel className="!mt-0">{opt.label}</FormLabel>
                </FormItem>
              ))}
            </RadioGroup>
          </FormControl>
          {description && <FormDescription>{description}</FormDescription>}
          <FormMessage />
        </FormItem>
      )}
    />
  )
}

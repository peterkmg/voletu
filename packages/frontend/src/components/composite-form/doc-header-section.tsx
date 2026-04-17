import type { FieldValues } from 'react-hook-form'
import type { HeaderFieldSpec } from './types'
import { useFormContext } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { cn } from '~/lib/utils'

interface DocHeaderSectionProps<TForm extends FieldValues> {
  fields: HeaderFieldSpec<TForm>[]
  /** Optional i18n key for a section heading. */
  titleKey?: string
  className?: string
}

export function DocHeaderSection<TForm extends FieldValues>({
  fields,
  titleKey,
  className,
}: DocHeaderSectionProps<TForm>) {
  const { t } = useTranslation()
  const { control } = useFormContext<TForm>()

  return (
    <section data-slot="doc-header-section" className={cn('space-y-4', className)}>
      {titleKey && (
        <h3 className="text-sm font-semibold uppercase text-muted-foreground tracking-wide">
          {t(titleKey)}
        </h3>
      )}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {fields.map((spec) => {
          const Component = spec.component
          return (
            <FormField
              key={spec.name as string}
              control={control}
              name={spec.name}
              render={() => (
                <FormItem
                  className={cn('grid gap-2', spec.colSpan === 2 && 'md:col-span-2')}
                >
                  <FormLabel>
                    {t(spec.labelKey)}
                    {spec.required && <span className="text-destructive ml-1">*</span>}
                  </FormLabel>
                  <FormControl>
                    <Component
                      name={spec.name}
                      placeholder={spec.placeholderKey ? t(spec.placeholderKey) : undefined}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          )
        })}
      </div>
    </section>
  )
}

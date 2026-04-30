import type { ReactNode } from 'react'
import type { DefaultValues, FieldValues, Path } from 'react-hook-form'
import type { RowFieldSpec } from './types'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect, useRef } from 'react'
import {

  FormProvider,

  useForm,
} from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { Button } from '~/components/ui/button'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { cn } from '~/lib/utils'

export interface DocItemRowDrawerProps<TItem extends FieldValues> {

  rowSchema: unknown
  fields: RowFieldSpec<TItem>[]
  defaultValues: DefaultValues<TItem>

  onSave: (data: TItem) => void

  onSaveAndAdd: (data: TItem) => void
  onCancel: () => void

  children?: ReactNode
}

export function DocItemRowDrawer<TItem extends FieldValues>({
  rowSchema,
  fields,
  defaultValues,
  onSave,
  onSaveAndAdd,
  onCancel,
  children,
}: DocItemRowDrawerProps<TItem>) {
  const { t } = useTranslation('forms')
  const form = useForm<TItem>({
    resolver: zodResolver(rowSchema as never),
    defaultValues,
    mode: 'onBlur',
  })

  const containerRef = useRef<HTMLDivElement>(null)
  const didFocusInitialFieldRef = useRef(false)

  useEffect(() => {
    if (didFocusInitialFieldRef.current)
      return

    const firstFieldName = fields[0]?.name
    if (firstFieldName) {
      form.setFocus(firstFieldName as Path<TItem>)
      didFocusInitialFieldRef.current = true
    }
  }, [fields, form])

  useEffect(() => {
    const node = containerRef.current
    if (!node)
      return

    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault()
        void form.handleSubmit(data => onSaveAndAdd(data))()
      }

      if (e.key === 'Escape') {
        e.preventDefault()
        onCancel()
      }
    }

    node.addEventListener('keydown', onKey)

    return () => node.removeEventListener('keydown', onKey)
  }, [form, onSaveAndAdd, onCancel])

  return (
    <div
      ref={containerRef}
      data-slot="doc-item-row-drawer"
      data-state="open"
      className={cn(
        'mt-2 rounded-md border border-ring/40 bg-muted/30 p-4',
        'data-[state=open]:animate-in data-[state=open]:slide-in-from-top-2',
      )}
    >
      <FormProvider {...form}>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {fields.map((spec) => {
            const Component = spec.component
            return (
              <FormField
                key={spec.name as string}
                control={form.control}
                name={spec.name}
                render={({ field, fieldState }) => (
                  <FormItem
                    className={cn('grid gap-2', spec.colSpan === 2 && 'md:col-span-2')}
                  >
                    <FormLabel className="text-xs">
                      {t(spec.labelKey)}
                      {spec.required && <span className="text-destructive ml-1">*</span>}
                    </FormLabel>
                    <FormControl>
                      <Component
                        field={field}
                        fieldState={fieldState}
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

        {children && <div className="mt-4 border-t pt-4">{children}</div>}

        <div className="mt-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
          <Button type="button" variant="outline" size="sm" onClick={onCancel}>
            {t('cancel')}
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            onClick={() => void form.handleSubmit(data => onSaveAndAdd(data))()}
            title={t('saveAndAddHint')}
          >
            {t('saveAndAdd')}
          </Button>
          <Button
            type="button"
            variant="default"
            size="sm"
            onClick={() => void form.handleSubmit(data => onSave(data))()}
          >
            {t('saveRow')}
          </Button>
        </div>
      </FormProvider>
    </div>
  )
}

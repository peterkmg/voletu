import type { ReactNode } from 'react'
import type { DefaultValues, FieldValues, UseFormReturn } from 'react-hook-form'
import type {
  CompositeMode,
  CompositeMutationFn,
  CompositeSuccessHandler,
  ServerValidationIssue,
} from './types'
import { AlertCircleIcon, Loader2Icon } from 'lucide-react'
import { useCallback, useEffect, useRef, useState } from 'react'
import { useFormContext } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { ConfirmDialog } from '~/components/dialogs/confirm-dialog'
import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import { isFormsValidationMessageKey, toFormsValidationTranslationKey } from '~/i18n/form-validation-message'
import { cn } from '~/lib/utils'
import { DocFormProvider } from './doc-form-provider'

export interface CompositeFormDialogProps<TForm extends FieldValues, TResponse> {
  open: boolean
  onOpenChange: (open: boolean) => void
  mode: CompositeMode

  schema: unknown
  defaultValues: DefaultValues<TForm>
  mutationFn: CompositeMutationFn<TForm, TResponse>
  onSuccess?: CompositeSuccessHandler<TResponse>

  titleKey: string

  descriptionKey?: string

  showSaveAndNew?: boolean

  heavy?: boolean
  children: ReactNode
}

export function CompositeFormDialog<TForm extends FieldValues, TResponse>({
  open,
  onOpenChange,
  mode,
  schema,
  defaultValues,
  mutationFn,
  onSuccess,
  titleKey,
  descriptionKey,
  showSaveAndNew = mode === 'create',
  heavy = false,
  children,
}: CompositeFormDialogProps<TForm, TResponse>) {
  const { t } = useTranslation('forms')
  const [globalIssues, setGlobalIssues] = useState<ServerValidationIssue[]>([])

  const submitActionRef = useRef<'save' | 'saveAndNew'>('save')
  const formApiRef = useRef<UseFormReturn<TForm> | null>(null)

  const handleSuccess = useCallback<CompositeSuccessHandler<TResponse>>(
    (saved) => {
      setGlobalIssues([])
      onSuccess?.(saved)

      if (submitActionRef.current === 'saveAndNew' && mode === 'create') {
        formApiRef.current?.reset(defaultValues)
        submitActionRef.current = 'save'
        return
      }

      submitActionRef.current = 'save'
      onOpenChange(false)
    },
    [onSuccess, onOpenChange, defaultValues, mode],
  )

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        data-slot="composite-form-dialog"
        data-heavy={heavy || undefined}
        className={cn(
          'flex flex-col overflow-hidden gap-0 p-0',
          'w-full h-full max-w-full max-h-[100dvh]',
          'md:h-auto md:max-h-[90dvh] md:max-w-2xl md:rounded-lg',
          'lg:max-w-3xl',
          'xl:max-w-4xl',
        )}
      >
        <DocFormProvider
          schema={schema as never}
          defaultValues={defaultValues}
          mutationFn={mutationFn}
          onSuccess={handleSuccess}
          onGlobalErrors={setGlobalIssues}
          formApiRef={formApiRef}
        >
          <DialogHeader className="border-b p-6 pb-4">
            <DialogTitle className="text-xl font-semibold">{t(titleKey)}</DialogTitle>
            {descriptionKey && (
              <DialogDescription className="text-sm text-muted-foreground">
                {t(descriptionKey)}
              </DialogDescription>
            )}
          </DialogHeader>

          {globalIssues.length > 0 && (
            <div
              data-slot="composite-form-banner"
              className="mx-6 mt-4 flex items-start gap-2 rounded-md bg-destructive/10 p-3 text-destructive text-sm"
            >
              <AlertCircleIcon className="size-4 shrink-0 mt-0.5" />
              <ul className="space-y-1">
                {globalIssues.map(issue => (
                  <li key={`${issue.field}-${issue.code}`}>
                    {issue.message && isFormsValidationMessageKey(issue.message)
                      ? t(toFormsValidationTranslationKey(issue.message), { defaultValue: issue.message })
                      : (issue.message ?? t(`validation.${issue.code}`))}
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div data-slot="composite-form-body" className="min-h-0 flex-1 overflow-y-auto p-6 space-y-6">{children}</div>

          <CompositeFormFooter
            mode={mode}
            showSaveAndNew={showSaveAndNew}
            onCancel={() => onOpenChange(false)}
            onSelectAction={(action) => {
              submitActionRef.current = action
            }}
          />
        </DocFormProvider>
      </DialogContent>
    </Dialog>
  )
}

interface FooterProps {
  mode: CompositeMode
  showSaveAndNew: boolean
  onCancel: () => void

  onSelectAction: (action: 'save' | 'saveAndNew') => void
}

function CompositeFormFooter({ mode, showSaveAndNew, onCancel, onSelectAction }: FooterProps) {
  const { t } = useTranslation('forms')
  const form = useFormContext()
  const submitting = form.formState.isSubmitting
  const dirty = form.formState.isDirty
  const [confirmOpen, setConfirmOpen] = useState(false)

  const handleCancel = useCallback(() => {
    if (dirty) {
      setConfirmOpen(true)
      return
    }
    onCancel()
  }, [dirty, onCancel])

  const handleConfirmDiscard = useCallback(() => {
    setConfirmOpen(false)
    onCancel()
  }, [onCancel])

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault()
        form.handleSubmit(() => undefined)()
      }
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  }, [form])

  return (
    <>
      <DialogFooter className="border-t p-6 pt-4 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
        <Button type="button" variant="outline" onClick={handleCancel} disabled={submitting}>
          {t('cancel')}
        </Button>
        {showSaveAndNew && mode === 'create' && (
          <Button
            type="submit"
            variant="outline"
            name="saveAndNew"
            disabled={submitting}
            onClick={() => onSelectAction('saveAndNew')}
          >
            {submitting && <Loader2Icon className="size-4 animate-spin mr-2" />}
            {t('saveAndNew')}
          </Button>
        )}
        <Button
          type="submit"
          variant="default"
          name="save"
          disabled={submitting}
          onClick={() => onSelectAction('save')}
        >
          {submitting && <Loader2Icon className="size-4 animate-spin mr-2" />}
          {t('save')}
        </Button>
      </DialogFooter>
      <ConfirmDialog
        open={confirmOpen}
        onOpenChange={setConfirmOpen}
        title={t('confirmDiscardTitle')}
        description={t('confirmDiscard')}
        confirmLabel={t('discard')}
        cancelLabel={t('keepEditing')}
        variant="destructive"
        onConfirm={handleConfirmDiscard}
      />
    </>
  )
}

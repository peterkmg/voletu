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
import { cn } from '~/lib/utils'
import { DocFormProvider } from './doc-form-provider'

export interface CompositeFormDialogProps<TForm extends FieldValues, TResponse> {
  open: boolean
  onOpenChange: (open: boolean) => void
  mode: CompositeMode
  /** Use `unknown` to mirror DocFormProvider's loosened typing (works around @hookform/resolvers v5 generics). */
  schema: unknown
  defaultValues: DefaultValues<TForm>
  mutationFn: CompositeMutationFn<TForm, TResponse>
  onSuccess?: CompositeSuccessHandler<TResponse>
  /** i18n key for the dialog title. */
  titleKey: string
  /** i18n key for the optional description below the title. */
  descriptionKey?: string
  /** Whether to render Save & New (only meaningful in create mode). Default true in create. */
  showSaveAndNew?: boolean
  /**
   * Vestigial — currently has no effect after the xl two-column grid was dropped.
   * Reserved for future width-extension scenarios. Safe to omit.
   */
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
  // Tracks which footer button initiated the current submit. Set by the
  // button's onClick handler before the native submit event fires, then
  // consulted in the success handler to decide between close-on-success
  // (Save) and reset-and-keep-open (Save & New). See spec §4.5.
  const submitActionRef = useRef<'save' | 'saveAndNew'>('save')
  const formApiRef = useRef<UseFormReturn<TForm> | null>(null)

  const handleSuccess = useCallback<CompositeSuccessHandler<TResponse>>(
    (saved) => {
      setGlobalIssues([])
      onSuccess?.(saved)
      if (submitActionRef.current === 'saveAndNew' && mode === 'create') {
        // Keep the modal open and clear the form for the next entry.
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
          'w-full h-full max-w-full max-h-[100dvh]', // < md: full viewport
          'md:h-auto md:max-h-[90dvh] md:max-w-2xl md:rounded-lg', // md – lg
          'lg:max-w-3xl', // lg – xl
          'xl:max-w-4xl', // ≥ xl, capped (no further growth)
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
                    {issue.message ?? t(`validation.${issue.code}`)}
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div data-slot="composite-form-body" className="min-h-0 flex-1 overflow-y-auto p-6">{children}</div>

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
  /** Notifies the dialog which submit button was clicked, before the form fires. */
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

  // Ctrl/Cmd+S submits
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

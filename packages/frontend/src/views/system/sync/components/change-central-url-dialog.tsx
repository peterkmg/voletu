import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { z } from 'zod'
import { client } from '~/api/client'
import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '~/components/ui/form'
import { Input } from '~/components/ui/input'
import { extractErrorMessage } from '~/lib/error'
import { useNodeStore } from '~/stores/node-store'

const schema = z.object({
  url: z
    .string()
    .trim()
    .url()
    .refine(v => /^https?:\/\//i.test(v), {
      message: 'URL must start with http:// or https://',
    }),
})

type FormValues = z.infer<typeof schema>

interface ChangeCentralUrlDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function ChangeCentralUrlDialog({ open, onOpenChange }: ChangeCentralUrlDialogProps) {
  const { t } = useTranslation('system')
  const queryClient = useQueryClient()
  const currentUrl = useNodeStore(s => s.status.centralApiUrl)

  const form = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: { url: currentUrl ?? '' },
    mode: 'onTouched',
  })

  // Re-seed the form every time the dialog opens so we always start from the
  // currently-persisted URL, not whatever the user last typed.
  useEffect(() => {
    if (open)
      form.reset({ url: currentUrl ?? '' })
  }, [open, currentUrl, form])

  const mutation = useMutation({
    mutationFn: async (url: string) => {
      await client({
        method: 'PATCH',
        url: '/node/central-api-url',
        data: { url },
      })
    },
    onSuccess: () => {
      // The handler returns an updated NodeStatusResponse, but the simplest
      // way to reflect it everywhere is to invalidate the status/bases queries;
      // use-node-status will refetch and apply the snapshot via the existing
      // effect.
      queryClient.invalidateQueries({ queryKey: ['node', 'status'] })
      queryClient.invalidateQueries({ queryKey: ['node', 'bases'] })
      queryClient.invalidateQueries({ queryKey: ['health'] })
      toast.success(t('sync.centralUrl.updated'))
      onOpenChange(false)
    },
    onError: (err) => {
      const message = extractErrorMessage(err, t('sync.centralUrl.updateFailed'))
      toast.error(message)
    },
  })

  function onSubmit(values: FormValues) {
    // Use mutate (fire-and-forget) so react-hook-form's submit promise doesn't
    // surface as an unhandled rejection on API errors. Error handling lives in
    // mutation.onError via a toast.
    mutation.mutate(values.url.trim())
  }

  function handleOpenChange(next: boolean) {
    if (!next && mutation.isPending)
      return
    onOpenChange(next)
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('sync.centralUrl.title')}</DialogTitle>
          <DialogDescription>{t('sync.centralUrl.description')}</DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="url"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{t('sync.centralUrl.urlLabel')}</FormLabel>
                  <FormControl>
                    <Input
                      type="url"
                      placeholder="https://central.example.com"
                      disabled={mutation.isPending}
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>{t('sync.centralUrl.urlHint')}</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <DialogFooter>
              <Button
                type="button"
                variant="outline"
                onClick={() => onOpenChange(false)}
                disabled={mutation.isPending}
              >
                {t('sync.centralUrl.cancel')}
              </Button>
              <Button type="submit" disabled={mutation.isPending}>
                {mutation.isPending ? t('sync.centralUrl.applying') : t('sync.centralUrl.apply')}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}

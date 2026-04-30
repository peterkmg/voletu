import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { client } from '~/api/client'
import { Button } from '~/components/ui/button'
import { Checkbox } from '~/components/ui/checkbox'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import { ScrollArea } from '~/components/ui/scroll-area'
import { useNodeStore } from '~/stores/node-store'

interface BaseAssignmentDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

interface BaseResponse {
  id: string
  commonName: string
  longName?: string | null
}

function diff(
  prev: ReadonlySet<string>,
  next: ReadonlySet<string>,
): { added: string[], removed: string[] } {
  const added: string[] = []
  const removed: string[] = []
  for (const id of next) {
    if (!prev.has(id))
      added.push(id)
  }
  for (const id of prev) {
    if (!next.has(id))
      removed.push(id)
  }
  return { added, removed }
}

export function BaseAssignmentDialog({ open, onOpenChange }: BaseAssignmentDialogProps) {
  const { t } = useTranslation('system')
  const queryClient = useQueryClient()
  const assignedBaseIds = useNodeStore(s => s.status.assignedBaseIds)

  const [pending, setPending] = useState<Set<string>>(() => new Set(assignedBaseIds))
  const [isApplying, setIsApplying] = useState(false)

  useEffect(() => {
    if (open)
      setPending(new Set(assignedBaseIds))
  }, [open, assignedBaseIds])

  const basesQuery = useQuery({
    queryKey: ['catalog', 'bases'],
    queryFn: async () => {
      const res = await client<{ success: true, data: BaseResponse[] }>({
        method: 'GET',
        url: '/catalog/bases',
      })
      return res.data.data
    },
    enabled: open,
  })

  const addMutation = useMutation({
    mutationFn: async (baseId: string) => {
      await client({ method: 'POST', url: '/node/bases', data: { baseId } })
    },
  })

  const removeMutation = useMutation({
    mutationFn: async (baseId: string) => {
      await client({ method: 'DELETE', url: `/node/bases/${baseId}` })
    },
  })

  const bases = basesQuery.data ?? []
  const assignedSet = new Set(assignedBaseIds)
  const { added, removed } = diff(assignedSet, pending)
  const hasChanges = added.length > 0 || removed.length > 0

  function togglePending(baseId: string, checked: boolean) {
    setPending((prev) => {
      const next = new Set(prev)
      if (checked)
        next.add(baseId)
      else next.delete(baseId)
      return next
    })
  }

  async function handleApply() {
    if (!hasChanges || isApplying)
      return
    setIsApplying(true)

    const results = await Promise.allSettled([
      ...added.map(id => addMutation.mutateAsync(id)),
      ...removed.map(id => removeMutation.mutateAsync(id)),
    ])

    const failed = results.filter(r => r.status === 'rejected').length
    const succeeded = results.length - failed

    queryClient.invalidateQueries({ queryKey: ['node', 'bases'] })

    setIsApplying(false)

    if (failed === 0) {
      toast.success(t('sync.bases.applySuccess', { count: succeeded }))
      onOpenChange(false)
    }
    else if (succeeded === 0) {
      toast.error(t('sync.bases.applyAllFailed'))
    }
    else {
      toast.error(t('sync.bases.applyPartial', { failed, succeeded }))
    }
  }

  function handleCancel() {
    if (isApplying)
      return
    setPending(new Set(assignedBaseIds))
    onOpenChange(false)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={v => !v ? handleCancel() : onOpenChange(true)}
    >
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('sync.bases.title')}</DialogTitle>
          <DialogDescription>{t('sync.bases.description')}</DialogDescription>
        </DialogHeader>

        <ScrollArea className="max-h-[60vh] -mx-6 px-6">
          <div className="flex flex-col gap-1">
            {bases.length === 0 && !basesQuery.isLoading && (
              <p className="text-sm text-muted-foreground py-4 text-center">
                {t('sync.bases.noBases')}
              </p>
            )}
            {bases.map((base) => {
              const checked = pending.has(base.id)
              const id = `base-assign-${base.id}`
              return (
                <label
                  key={base.id}
                  htmlFor={id}
                  className="flex items-start gap-3 rounded-md py-2 px-2 hover:bg-accent/40 cursor-pointer"
                >
                  <Checkbox
                    id={id}
                    checked={checked}
                    onCheckedChange={v => togglePending(base.id, v === true)}
                    disabled={isApplying}
                    className="mt-0.5"
                  />
                  <div className="min-w-0 flex-1">
                    <div className="text-sm font-medium break-words">{base.commonName}</div>
                    {base.longName && (
                      <div className="text-xs text-muted-foreground break-words">{base.longName}</div>
                    )}
                  </div>
                </label>
              )
            })}
          </div>
        </ScrollArea>

        <DialogFooter>
          <Button variant="outline" onClick={handleCancel} disabled={isApplying}>
            {t('sync.bases.cancel')}
          </Button>
          <Button onClick={handleApply} disabled={!hasChanges || isApplying}>
            {isApplying ? t('sync.bases.applying') : t('sync.bases.apply')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

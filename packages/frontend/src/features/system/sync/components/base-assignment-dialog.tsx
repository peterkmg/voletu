import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'
import { client } from '~/api/client'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
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

export function BaseAssignmentDialog({ open, onOpenChange }: BaseAssignmentDialogProps) {
  const { t } = useTranslation('system')
  const queryClient = useQueryClient()
  const assignedBaseIds = useNodeStore(s => s.status.assignedBaseIds)

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
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['node', 'bases'] })
    },
  })

  const removeMutation = useMutation({
    mutationFn: async (baseId: string) => {
      await client({ method: 'DELETE', url: `/node/bases/${baseId}` })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['node', 'bases'] })
    },
  })

  function handleToggle(baseId: string) {
    const isAssigned = assignedBaseIds.includes(baseId)
    if (isAssigned) {
      removeMutation.mutate(baseId, {
        onSuccess: () => {
          useNodeStore.getState().setStatus({
            assignedBaseIds: assignedBaseIds.filter(id => id !== baseId),
          })
        },
        onError: () => toast.error('Failed to remove base assignment'),
      })
    }
    else {
      addMutation.mutate(baseId, {
        onSuccess: () => {
          useNodeStore.getState().setStatus({
            assignedBaseIds: [...assignedBaseIds, baseId],
          })
        },
        onError: () => toast.error('Failed to add base assignment'),
      })
    }
  }

  const bases = basesQuery.data ?? []

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('sync.bases.title')}</DialogTitle>
          <DialogDescription>{t('sync.bases.description')}</DialogDescription>
        </DialogHeader>
        <div className="space-y-1.5">
          {bases.length === 0 && (
            <p className="text-sm text-muted-foreground py-4 text-center">
              {t('sync.bases.noBases')}
            </p>
          )}
          {bases.map((base) => {
            const isAssigned = assignedBaseIds.includes(base.id)
            return (
              <Button
                key={base.id}
                variant="ghost"
                className="w-full justify-between h-auto py-2.5"
                onClick={() => handleToggle(base.id)}
                disabled={addMutation.isPending || removeMutation.isPending}
              >
                <span className="text-left">
                  <span className="block font-medium">{base.commonName}</span>
                  {base.longName && (
                    <span className="block text-xs text-muted-foreground">{base.longName}</span>
                  )}
                </span>
                <Badge variant={isAssigned ? 'default' : 'outline'}>
                  {isAssigned ? t('sync.bases.assigned') : t('sync.bases.notAssigned')}
                </Badge>
              </Button>
            )
          })}
        </div>
      </DialogContent>
    </Dialog>
  )
}

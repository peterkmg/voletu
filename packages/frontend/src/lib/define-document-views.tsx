import type { TFunction } from 'i18next'
import type { MutateDialogProps } from './create-entity-dialogs'
import type {
  CrudViewDeleteDialogConfig,
  CrudViewQueryResult,
  CrudViewTableProps,
} from './define-crud-views'
import { useTranslation } from 'react-i18next'
import { LifecycleDialog } from '~/components/dialogs/lifecycle-dialog'
import { DocumentDetailPage } from '~/components/document'
import { Skeleton } from '~/components/ui/skeleton'
import { defineCrudViews } from './define-crud-views'

interface DocumentViewText {
  title: string
  entityLabel: string
  subtitle?: string
}

interface DocumentActionConfig {
  enableRowLifecycle?: boolean
  executeFn: (id: string) => Promise<unknown>
  revertFn: (id: string) => Promise<unknown>
  queryKey: () => readonly unknown[]
}

interface DocumentSummary {
  id: string
  documentNumber: string
  status: string
  [key: string]: unknown
}

interface RowLifecycleDialogProps<TRow extends { id: string }> {
  open: boolean
  onOpenChange: (open: boolean) => void
  currentRow: TRow | null
  variant: 'execute' | 'revert'
}

interface DetailViewContext<TDetailData> {
  data: NonNullable<TDetailData>
  isLocked: boolean
  t: TFunction
}

interface DocumentDetailViewConfig<TDetailData> {
  useDetailId: () => string
  useDetailQuery: (id: string) => { data?: { data?: TDetailData }, isLoading: boolean }
  backTo: string
  statusColorMap?: Record<string, string>
  getDocument: (data: NonNullable<TDetailData>) => DocumentSummary
  renderRelatedContent?: (context: DetailViewContext<TDetailData>) => React.ReactNode
  renderFormContent: (context: DetailViewContext<TDetailData>) => React.ReactNode
  renderItemsContent?: (context: DetailViewContext<TDetailData>) => React.ReactNode
  renderMetadataContent?: (context: DetailViewContext<TDetailData>) => React.ReactNode
  renderLoadingState?: () => React.ReactNode
  renderEmptyState?: (t: TFunction) => React.ReactNode
}

interface DocumentViewConfig<TRow extends { id: string }, TDetailData> {
  displayName: string
  useText: () => DocumentViewText
  useQuery: () => CrudViewQueryResult<TRow>
  Table: React.ComponentType<CrudViewTableProps<TRow>>
  MutateDialog: React.ComponentType<MutateDialogProps<TRow>>
  detail: DocumentDetailViewConfig<TDetailData>
  deleteDialog?: CrudViewDeleteDialogConfig
  documentActions?: DocumentActionConfig
  supportsUpdate?: boolean
  rowActions?: {
    deleteOnly?: boolean
    disableEdit?: boolean
    getDetailPath?: (row: TRow) => string
  }
}

interface ResolvedDetailVariant {
  content?: React.ReactNode
  isLoading: boolean
}

interface ResolvedDetailViewConfig {
  useDetailId: () => string
  useVariants: (id: string) => ResolvedDetailVariant[]
  getNotFoundMessage?: (t: TFunction) => string
  renderLoadingState?: () => React.ReactNode
}

function DefaultLoadingState() {
  return (
    <div className="p-4">
      <Skeleton className="h-64 w-full" />
    </div>
  )
}

function DefaultEmptyState({ message }: { message: string }) {
  return <div className="p-4">{message}</div>
}

function createDocumentLifecycleDialog<TRow extends { id: string }>(
  useText: () => DocumentViewText,
  actions: DocumentActionConfig,
) {
  return function DocumentLifecycleDialog({
    open,
    onOpenChange,
    currentRow,
    variant,
  }: RowLifecycleDialogProps<TRow>) {
    const { entityLabel } = useText()

    return (
      <LifecycleDialog
        open={open}
        onOpenChange={onOpenChange}
        currentRow={currentRow}
        action={variant}
        executeFn={actions.executeFn}
        revertFn={actions.revertFn}
        queryKey={actions.queryKey()}
        entityLabel={entityLabel}
      />
    )
  }
}

export function defineResolvedDetailView({
  useDetailId,
  useVariants,
  getNotFoundMessage,
  renderLoadingState,
}: ResolvedDetailViewConfig) {
  return function ResolvedDocumentDetail() {
    const { t } = useTranslation(['documents', 'common'])
    const id = useDetailId()
    const variants = useVariants(id)
    const matched = variants.find(variant => variant.content != null)
    const notFoundMessage = getNotFoundMessage
      ? getNotFoundMessage(t)
      : t('documents:messages.notFound')

    if (!matched && variants.every(variant => variant.isLoading)) {
      return renderLoadingState ? renderLoadingState() : <DefaultLoadingState />
    }

    if (matched?.content != null) {
      return <>{matched.content}</>
    }

    return <DefaultEmptyState message={notFoundMessage} />
  }
}

export function defineDocumentViews<TRow extends { id: string }, TDetailData>(
  config: DocumentViewConfig<TRow, TDetailData>,
) {
  const LifecycleDialog = config.documentActions?.enableRowLifecycle
    ? createDocumentLifecycleDialog(config.useText, config.documentActions)
    : undefined

  const crudViewDefinition = LifecycleDialog
    ? defineCrudViews<TRow>({
        displayName: config.displayName,
        useTitle: () => config.useText().title,
        useQuery: config.useQuery,
        Table: config.Table,
        MutateDialog: config.MutateDialog,
        deleteDialog: config.deleteDialog,
        supportsUpdate: config.supportsUpdate,
        rowActions: {
          ...config.rowActions,
          lifecycle: true,
        },
        LifecycleDialog,
        lifecyclePropName: 'variant',
      })
    : defineCrudViews<TRow>({
        displayName: config.displayName,
        useTitle: () => config.useText().title,
        useQuery: config.useQuery,
        Table: config.Table,
        MutateDialog: config.MutateDialog,
        deleteDialog: config.deleteDialog,
        supportsUpdate: config.supportsUpdate,
        rowActions: config.rowActions,
      })

  function DetailView() {
    const { t } = useTranslation(['documents', 'common'])
    const { title, entityLabel, subtitle } = config.useText()
    const detailId = config.detail.useDetailId()
    const result = config.detail.useDetailQuery(detailId)

    if (result.isLoading) {
      return config.detail.renderLoadingState
        ? config.detail.renderLoadingState()
        : <DefaultLoadingState />
    }

    const detailData = result.data?.data
    if (!detailData) {
      if (config.detail.renderEmptyState) {
        return <>{config.detail.renderEmptyState(t)}</>
      }

      return (
        <DefaultEmptyState
          message={t('documents:messages.notFound')}
        />
      )
    }

    const resolvedDetailData = detailData as NonNullable<TDetailData>
    const document = config.detail.getDocument(resolvedDetailData)
    const detailContext = {
      data: resolvedDetailData,
      isLocked: document.status === 'EXECUTED',
      t,
    }

    return (
      <DocumentDetailPage
        config={{
          title,
          entityLabel,
          backTo: config.detail.backTo,
          executeFn: config.documentActions?.executeFn,
          revertFn: config.documentActions?.revertFn,
          queryKey: config.documentActions?.queryKey(),
          statusColorMap: config.detail.statusColorMap,
        }}
        document={document}
        subtitle={subtitle}
        relatedContent={config.detail.renderRelatedContent?.(detailContext)}
        formContent={config.detail.renderFormContent(detailContext)}
        itemsContent={config.detail.renderItemsContent?.(detailContext)}
        metadataContent={config.detail.renderMetadataContent?.(detailContext)}
      />
    )
  }

  return {
    ...crudViewDefinition,
    DetailView,
  } as const
}

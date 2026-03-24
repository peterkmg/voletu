import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import { Skeleton } from '~/components/ui/skeleton'

interface EntityPageProps<TRow> {
  provider: React.ComponentType<{ children: React.ReactNode }>
  title: string
  queryResult: { data?: { data?: TRow[] }, isLoading: boolean }
  primaryButtons: React.ComponentType
  table: React.ComponentType<{ data: TRow[] }>
  dialogs: React.ComponentType
}

export function EntityPage<TRow>({
  provider: Provider,
  title,
  queryResult,
  primaryButtons: PrimaryButtons,
  table: Table,
  dialogs: Dialogs,
}: EntityPageProps<TRow>) {
  const data = queryResult.data?.data ?? []

  return (
    <Provider>
      <Header fixed />

      <Main className="flex flex-1 flex-col gap-4 sm:gap-6">
        <div className="flex flex-wrap items-end justify-between gap-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">
              {title}
            </h2>
          </div>
          <PrimaryButtons />
        </div>
        {queryResult.isLoading
          ? (
              <div className="flex flex-1 flex-col gap-4">
                <div className="flex items-center gap-2">
                  <Skeleton className="h-9 w-64" />
                  <Skeleton className="ml-auto h-9 w-24" />
                </div>
                <div className="flex-1 rounded-md border">
                  <div className="space-y-3 p-4">
                    {Array.from({ length: 8 }, (_, i) => (
                      <Skeleton key={i} className="h-8 w-full" />
                    ))}
                  </div>
                </div>
              </div>
            )
          : (
              <Table data={data} />
            )}
      </Main>

      <Dialogs />
    </Provider>
  )
}

import type { FileRoutePath } from './file-route-path'
import { createFileRoute } from '@tanstack/react-router'

type FileRouteFactory = typeof createFileRoute

export function defineDetailRoute<TPath extends FileRoutePath>(path: TPath): ReturnType<typeof createFileRoute<TPath>>
export function defineDetailRoute<TPath extends FileRoutePath>(
  routeFactory: FileRouteFactory,
  path: TPath,
): ReturnType<typeof createFileRoute<TPath>>
export function defineDetailRoute<TPath extends FileRoutePath>(
  pathOrFactory: TPath | FileRouteFactory,
  maybePath?: TPath,
) {
  if (typeof pathOrFactory === 'function') {
    return pathOrFactory(maybePath as TPath)
  }

  return createFileRoute(pathOrFactory)
}

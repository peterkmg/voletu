import { Link, useLocation } from '@tanstack/react-router'
import { Fragment } from 'react'
import { useTranslation } from 'react-i18next'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '~/components/ui/breadcrumb'
import { breadcrumbMap } from './data/breadcrumb-map'

export function Breadcrumbs() {
  const { t } = useTranslation()
  const location = useLocation()

  const segments = location.pathname
    .split('/')
    .filter(s => s && !s.startsWith('_') && !s.startsWith('('))

  if (segments.length === 0)
    return null

  return (
    <Breadcrumb>
      <BreadcrumbList>
        {segments.map((segment, index) => {
          const isLast = index === segments.length - 1
          const path = `/${segments.slice(0, index + 1).join('/')}`
          const label = breadcrumbMap[segment]
            ? t(breadcrumbMap[segment])
            : segment.replace(/-/g, ' ').replace(/\b\w/g, c => c.toUpperCase())

          return (
            <Fragment key={path}>
              {index > 0 && <BreadcrumbSeparator />}
              <BreadcrumbItem>
                {isLast
                  ? (
                      <BreadcrumbPage>{label}</BreadcrumbPage>
                    )
                  : (
                      <BreadcrumbLink asChild>
                        <Link to={path}>{label}</Link>
                      </BreadcrumbLink>
                    )}
              </BreadcrumbItem>
            </Fragment>
          )
        })}
      </BreadcrumbList>
    </Breadcrumb>
  )
}

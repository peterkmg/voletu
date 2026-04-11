import { createFileRoute } from '@tanstack/react-router'
import Settings from '~/views/system/settings'

export const Route = createFileRoute('/_authenticated/settings/')({
  component: Settings,
})

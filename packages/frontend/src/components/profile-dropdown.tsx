import { useNavigate } from '@tanstack/react-router'
import { LogOut, User } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { ConfirmDialog } from '~/components/confirm-dialog'
import { Avatar, AvatarFallback } from '~/components/ui/avatar'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { useAuthStore } from '~/stores/auth-store'

function getInitials(name: string): string {
  return name
    .split(' ')
    .map(part => part[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

export function ProfileDropdown() {
  const { t } = useTranslation()
  const navigate = useNavigate()
  const user = useAuthStore(s => s.auth.user)
  const [signOutOpen, setSignOutOpen] = useState(false)

  const displayName = user?.fullname ?? user?.username ?? 'User'
  const displayEmail = user?.username ?? ''

  const handleSignOut = () => {
    useAuthStore.getState().auth.clearSession()
    navigate({ to: '/sign-in' })
  }

  return (
    <>
      <DropdownMenu modal={false}>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon" className="rounded-full">
            <Avatar className="h-8 w-8">
              <AvatarFallback>{getInitials(displayName)}</AvatarFallback>
            </Avatar>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-56">
          <DropdownMenuLabel className="font-normal">
            <div className="flex flex-col space-y-1">
              <p className="text-sm font-medium leading-none">{displayName}</p>
              <p className="text-xs text-muted-foreground leading-none">{displayEmail}</p>
            </div>
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem onSelect={() => navigate({ to: '/settings' })}>
            <User />
            {t('auth:session.profile')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            variant="destructive"
            onSelect={() => setSignOutOpen(true)}
          >
            <LogOut />
            {t('auth:session.logout')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <ConfirmDialog
        open={signOutOpen}
        onOpenChange={setSignOutOpen}
        title={t('common:titlebar.signOut')}
        description={t('common:titlebar.signOutConfirm')}
        confirmLabel={t('common:actions.confirm')}
        cancelLabel={t('common:actions.cancel')}
        variant="destructive"
        onConfirm={handleSignOut}
      />
    </>
  )
}

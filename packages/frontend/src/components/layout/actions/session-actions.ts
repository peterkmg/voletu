import { signInViewTarget } from '~/router/view-targets'
import { useAuthStore } from '~/stores/auth-store'

type SignOutNavigate = (options: typeof signInViewTarget) => unknown

export function signOutAction(navigate: SignOutNavigate) {
  useAuthStore.getState().logout()
  navigate(signInViewTarget)
}

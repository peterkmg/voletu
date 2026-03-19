import { useNavigate, useRouter } from '@tanstack/react-router'
import { Button } from '~/components/ui/button'

interface GeneralErrorProps {
  statusCode?: number
  title?: string
  message?: string
}

export function GeneralError({
  statusCode = 500,
  title = 'Something went wrong',
  message = 'An unexpected error occurred. Please try again later.',
}: GeneralErrorProps) {
  const navigate = useNavigate()
  const router = useRouter()

  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-4">
      <h1 className="text-7xl font-bold text-muted-foreground">
        {statusCode}
      </h1>
      <p className="text-lg font-semibold">{title}</p>
      <p className="text-muted-foreground">{message}</p>
      <div className="flex gap-2">
        <Button variant="outline" onClick={() => router.history.back()}>
          Go Back
        </Button>
        <Button onClick={() => navigate({ to: '/' })}>Dashboard</Button>
      </div>
    </div>
  )
}

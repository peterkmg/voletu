import { Loader2 } from 'lucide-react'

export function SplashScreen() {
  return (
    <div className="flex h-svh items-center justify-center">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    </div>
  )
}

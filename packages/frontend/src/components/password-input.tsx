import { Eye, EyeOff } from 'lucide-react'
import { useState } from 'react'
import { Button } from '~/components/ui/button'
import { Input } from '~/components/ui/input'
import { cn } from '~/lib/utils'

function PasswordInput({ ref, className, ...props }: React.ComponentProps<'input'>) {
  const [visible, setVisible] = useState(false)

  return (
    <div className="relative">
      <Input
        ref={ref}
        type={visible ? 'text' : 'password'}
        className={cn('pr-10', className)}
        {...props}
      />
      <Button
        type="button"
        variant="ghost"
        size="icon"
        className="absolute top-0 right-0 h-full px-3 py-2 hover:bg-transparent"
        onClick={() => setVisible(prev => !prev)}
        tabIndex={-1}
      >
        {visible
          ? (
              <EyeOff className="size-4 text-muted-foreground" />
            )
          : (
              <Eye className="size-4 text-muted-foreground" />
            )}
        <span className="sr-only">
          {visible ? 'Hide password' : 'Show password'}
        </span>
      </Button>
    </div>
  )
}
PasswordInput.displayName = 'PasswordInput'

export { PasswordInput }

import { useEffect, useState } from 'react'
import { Input } from '~/components/ui/input'

interface DebouncedInputProps
  extends Omit<React.ComponentProps<typeof Input>, 'onChange'> {
  value: string
  onChange: (value: string) => void
  debounce?: number
}

export function DebouncedInput({
  value: externalValue,
  onChange,
  debounce = 250,
  ...props
}: DebouncedInputProps) {
  const [value, setValue] = useState(externalValue)

  useEffect(() => {
    // eslint-disable-next-line react-hooks-extra/no-direct-set-state-in-use-effect
    setValue(externalValue)
  }, [externalValue])

  useEffect(() => {
    const timeout = setTimeout(() => {
      if (value !== externalValue) {
        onChange(value)
      }
    }, debounce)
    return () => clearTimeout(timeout)
  }, [value, debounce]) // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <Input
      {...props}
      value={value}
      onChange={e => setValue(e.target.value)}
    />
  )
}

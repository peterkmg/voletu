import { useEffect, useRef, useState } from 'react'
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
  const onChangeRef = useRef(onChange)
  onChangeRef.current = onChange

  useEffect(() => {
    setValue(externalValue)
  }, [externalValue])

  useEffect(() => {
    const timeout = setTimeout(() => {
      if (value !== externalValue) {
        onChangeRef.current(value)
      }
    }, debounce)
    return () => clearTimeout(timeout)
  }, [value, debounce, externalValue])

  return (
    <Input
      {...props}
      value={value}
      onChange={e => setValue(e.target.value)}
    />
  )
}

import { useState } from 'react'

export default function useDialogState<T extends string | boolean>(
  initialState: T | null = null,
) {
  // eslint-disable-next-line react-naming-convention/use-state
  const [open, _setOpen] = useState<T | null>(initialState)

  const setOpen = (str: T | null) =>
    _setOpen(prev => (prev === str ? null : str))

  return [open, setOpen] as const
}

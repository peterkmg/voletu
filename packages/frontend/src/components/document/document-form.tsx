import { createContext, use } from 'react'

interface DocumentFormContextValue {
  isLocked: boolean
}

const DocumentFormContext = createContext<DocumentFormContextValue>({
  isLocked: false,
})

export function useDocumentFormLock() {
  return use(DocumentFormContext)
}

interface DocumentFormProps {
  status: string
  children: React.ReactNode
  className?: string
}

export function DocumentForm({ status, children, className }: DocumentFormProps) {
  const isLocked = status === 'EXECUTED'

  return (
    <DocumentFormContext value={{ isLocked }}>
      <fieldset disabled={isLocked} className={className ?? 'space-y-4'}>
        {children}
      </fieldset>
    </DocumentFormContext>
  )
}

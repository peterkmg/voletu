import { createContext, use } from 'react'
import { isDocEditable } from '~/hooks/use-doc-editable'

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
  const isLocked = !isDocEditable({ status })

  return (
    <DocumentFormContext value={{ isLocked }}>
      <fieldset disabled={isLocked} className={className ?? 'space-y-4'}>
        {children}
      </fieldset>
    </DocumentFormContext>
  )
}

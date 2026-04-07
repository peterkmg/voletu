interface DetailFieldProps {
  label: string
  children: React.ReactNode
}

export function DetailField({ label, children }: DetailFieldProps) {
  return (
    <div>
      <span className="text-sm text-muted-foreground">{label}</span>
      <p>{children}</p>
    </div>
  )
}

import { useNavigate } from '@tanstack/react-router'
import { Card, CardContent } from '~/components/ui/card'

interface BasisLinkProps {
  label: string
  documentNumber: string
  details: { label: string; value: string }[]
  navigateTo: string
}

export function BasisLink({
  label,
  documentNumber,
  details,
  navigateTo,
}: BasisLinkProps) {
  const navigate = useNavigate()

  return (
    <div className="space-y-1">
      <h3 className="text-sm font-medium uppercase tracking-wider text-muted-foreground">
        {label}
      </h3>
      <Card
        className="cursor-pointer transition-colors hover:border-primary"
        onClick={() => navigate({ to: navigateTo })}
      >
        <CardContent className="flex items-center gap-6 p-3 text-sm">
          <span className="font-medium text-primary">{documentNumber}</span>
          {details.map((d) => (
            <span key={d.label}>
              <span className="text-muted-foreground">{d.label}:</span>{' '}
              {d.value}
            </span>
          ))}
          <span className="ml-auto text-muted-foreground">View →</span>
        </CardContent>
      </Card>
    </div>
  )
}

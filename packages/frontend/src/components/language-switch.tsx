import { Check, Globe } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { changeLanguagePreference, languageOptions } from '~/components/layout/actions/language-actions'
import { Button } from '~/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { cn } from '~/lib/utils'

export function LanguageSwitch() {
  const { i18n, t } = useTranslation('common')

  const switchLanguage = (lang: string) => {
    changeLanguagePreference(i18n, lang)
  }

  return (
    <DropdownMenu modal={false}>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="scale-95 rounded-full">
          <Globe className="size-[1.2rem]" />
          <span className="sr-only">{t('language.switch')}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {languageOptions.map(lang => (
          <DropdownMenuItem
            key={lang.code}
            onClick={() => switchLanguage(lang.code)}
          >
            {lang.label}
            <Check
              size={14}
              className={cn('ms-auto', i18n.language !== lang.code && 'hidden')}
            />
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

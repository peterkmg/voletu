import { useTranslation } from 'react-i18next'
import { Header } from '~/components/layout/header'
import { Main } from '~/components/layout/main'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { Label } from '~/components/ui/label'
import { RadioGroup, RadioGroupItem } from '~/components/ui/radio-group'
import { useTheme } from '~/context/theme-provider'
import { useAuthStore } from '~/stores/auth-store'

export default function Settings() {
  const { t, i18n } = useTranslation(['system', 'common'])
  const user = useAuthStore(s => s.user)
  const { theme, setTheme } = useTheme()

  return (
    <>
      <Header fixed />

      <Main fixed className="flex flex-1 flex-col gap-4 sm:gap-6">
        <h2 className="text-2xl font-bold tracking-tight">
          {t('common:nav.settings')}
        </h2>

        <div className="flex-1 overflow-y-auto -mx-4 px-4">
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">

            <Card>
              <CardHeader>
                <CardTitle>{t('system:settings.userInfo')}</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-1">
                  <Label className="text-muted-foreground text-xs">
                    {t('system:settings.username')}
                  </Label>
                  <p className="text-sm font-medium">
                    {user?.username ?? '-'}
                  </p>
                </div>
                <div className="space-y-1">
                  <Label className="text-muted-foreground text-xs">
                    {t('system:settings.fullname')}
                  </Label>
                  <p className="text-sm font-medium">
                    {user?.fullname ?? '-'}
                  </p>
                </div>
                <div className="space-y-1">
                  <Label className="text-muted-foreground text-xs">
                    {t('system:settings.role')}
                  </Label>
                  <p className="text-sm font-medium">
                    {user?.role ?? '-'}
                  </p>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>{t('system:settings.language')}</CardTitle>
              </CardHeader>
              <CardContent>
                <RadioGroup
                  value={i18n.language}
                  onValueChange={lng => i18n.changeLanguage(lng)}
                  className="space-y-2"
                >
                  <div className="flex items-center space-x-2">
                    <RadioGroupItem value="en" id="lang-en" />
                    <Label htmlFor="lang-en">
                      {t('common:language.en')}
                    </Label>
                  </div>
                  <div className="flex items-center space-x-2">
                    <RadioGroupItem value="ru" id="lang-ru" />
                    <Label htmlFor="lang-ru">
                      {t('common:language.ru')}
                    </Label>
                  </div>
                </RadioGroup>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>{t('system:settings.theme')}</CardTitle>
              </CardHeader>
              <CardContent>
                <RadioGroup
                  value={theme}
                  onValueChange={value =>
                    setTheme(value as 'light' | 'dark' | 'system')}
                  className="space-y-2"
                >
                  <div className="flex items-center space-x-2">
                    <RadioGroupItem value="light" id="theme-light" />
                    <Label htmlFor="theme-light">
                      {t('common:theme.light')}
                    </Label>
                  </div>
                  <div className="flex items-center space-x-2">
                    <RadioGroupItem value="dark" id="theme-dark" />
                    <Label htmlFor="theme-dark">
                      {t('common:theme.dark')}
                    </Label>
                  </div>
                  <div className="flex items-center space-x-2">
                    <RadioGroupItem value="system" id="theme-system" />
                    <Label htmlFor="theme-system">
                      {t('common:theme.system')}
                    </Label>
                  </div>
                </RadioGroup>
              </CardContent>
            </Card>
          </div>
        </div>
      </Main>
    </>
  )
}

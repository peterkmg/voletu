import { useTranslation } from 'react-i18next'
import { EntityPickerCombobox } from '~/components/entity-picker/entity-picker-combobox'

export interface ContractorPickerProps {
  value: string | null
  onChange: (id: string | null) => void
  items: Array<{ id: string, label: string }>
  disabled?: boolean
}

export function ContractorPicker({ value, onChange, items, disabled }: ContractorPickerProps) {
  const { t } = useTranslation('dashboard')
  return (
    <EntityPickerCombobox
      items={items}
      value={value}
      onChange={onChange}
      placeholder={t('toolbar.contractor')}
      disabled={disabled}
    />
  )
}

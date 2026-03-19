import Decimal from 'decimal.js'

export type DecimalString = string

export function decimal(value: Decimal.Value): Decimal {
  return new Decimal(value)
}

export function addDecimals(...values: Decimal.Value[]): Decimal {
  return values.reduce<Decimal>(
    (accumulator, current) => accumulator.plus(current),
    new Decimal(0),
  )
}

export function subtractDecimals(base: Decimal.Value, ...values: Decimal.Value[]): Decimal {
  return values.reduce<Decimal>(
    (accumulator, current) => accumulator.minus(current),
    new Decimal(base),
  )
}

export function multiplyDecimals(...values: Decimal.Value[]): Decimal {
  if (values.length === 0) {
    return new Decimal(0)
  }

  return values.reduce<Decimal>(
    (accumulator, current) => accumulator.mul(current),
    new Decimal(1),
  )
}

export function decimalToString(value: Decimal.Value): DecimalString {
  return new Decimal(value).toString()
}

export function decimalToNumber(value: Decimal.Value): number {
  return new Decimal(value).toNumber()
}

export function isDecimalString(value: string): boolean {
  try {
    // eslint-disable-next-line no-new
    new Decimal(value)
    return true
  }
  catch {
    return false
  }
}

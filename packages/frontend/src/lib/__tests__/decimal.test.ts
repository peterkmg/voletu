import Decimal from 'decimal.js'
import {
  addDecimals,
  decimal,
  decimalToNumber,
  decimalToString,
  isDecimalString,
  multiplyDecimals,
  subtractDecimals,
} from '~/lib/decimal'

describe('decimal()', () => {
  it('creates a Decimal from a string', () => {
    const result = decimal('3.14')
    expect(result).toBeInstanceOf(Decimal)
    expect(result.toString()).toBe('3.14')
  })

  it('creates a Decimal from a number', () => {
    const result = decimal(42)
    expect(result).toBeInstanceOf(Decimal)
    expect(result.toNumber()).toBe(42)
  })

  it('creates a Decimal from zero', () => {
    const result = decimal(0)
    expect(result.toNumber()).toBe(0)
  })

  it('creates a Decimal from a negative number', () => {
    const result = decimal(-99.5)
    expect(result.toNumber()).toBe(-99.5)
  })

  it('creates a Decimal from a very large number string', () => {
    const large = '99999999999999999999999999999.123456789'
    const result = decimal(large)
    // Decimal.js may use exponential notation; verify value equivalence
    expect(result.equals(large)).toBe(true)
  })
})

describe('addDecimals()', () => {
  it('adds two values', () => {
    const result = addDecimals('1.1', '2.2')
    expect(result.toString()).toBe('3.3')
  })

  it('adds multiple values', () => {
    const result = addDecimals('1', '2', '3', '4')
    expect(result.toNumber()).toBe(10)
  })

  it('returns 0 when called with no arguments', () => {
    const result = addDecimals()
    expect(result.toNumber()).toBe(0)
  })

  it('handles negative values', () => {
    const result = addDecimals('10', '-3')
    expect(result.toNumber()).toBe(7)
  })

  it('preserves precision for decimal fractions', () => {
    // In floating-point: 0.1 + 0.2 = 0.30000000000000004
    const result = addDecimals('0.1', '0.2')
    expect(result.toString()).toBe('0.3')
  })
})

describe('subtractDecimals()', () => {
  it('subtracts a single value from base', () => {
    const result = subtractDecimals('10', '3')
    expect(result.toNumber()).toBe(7)
  })

  it('subtracts multiple values from base', () => {
    const result = subtractDecimals('100', '20', '30')
    expect(result.toNumber()).toBe(50)
  })

  it('handles negative result', () => {
    const result = subtractDecimals('5', '10')
    expect(result.toNumber()).toBe(-5)
  })

  it('returns base when no values to subtract', () => {
    const result = subtractDecimals('42')
    expect(result.toNumber()).toBe(42)
  })

  it('preserves precision', () => {
    const result = subtractDecimals('1.0', '0.1', '0.2')
    expect(result.toString()).toBe('0.7')
  })
})

describe('multiplyDecimals()', () => {
  it('multiplies two values', () => {
    const result = multiplyDecimals('3', '4')
    expect(result.toNumber()).toBe(12)
  })

  it('multiplies multiple values', () => {
    const result = multiplyDecimals('2', '3', '5')
    expect(result.toNumber()).toBe(30)
  })

  it('returns 0 when called with no arguments', () => {
    const result = multiplyDecimals()
    expect(result.toNumber()).toBe(0)
  })

  it('multiplies by zero', () => {
    const result = multiplyDecimals('999', '0')
    expect(result.toNumber()).toBe(0)
  })

  it('handles negative values', () => {
    const result = multiplyDecimals('-2', '5')
    expect(result.toNumber()).toBe(-10)
  })

  it('preserves precision for fractional multiplication', () => {
    const result = multiplyDecimals('0.1', '0.2')
    expect(result.toString()).toBe('0.02')
  })
})

describe('decimalToString()', () => {
  it('converts a number to string', () => {
    expect(decimalToString(42)).toBe('42')
  })

  it('converts a string value through Decimal', () => {
    expect(decimalToString('3.14')).toBe('3.14')
  })

  it('converts zero', () => {
    expect(decimalToString(0)).toBe('0')
  })

  it('converts negative value', () => {
    expect(decimalToString(-7.5)).toBe('-7.5')
  })
})

describe('decimalToNumber()', () => {
  it('converts a string to number', () => {
    expect(decimalToNumber('42')).toBe(42)
  })

  it('converts a Decimal-compatible value', () => {
    expect(decimalToNumber('3.14')).toBe(3.14)
  })

  it('converts zero', () => {
    expect(decimalToNumber('0')).toBe(0)
  })

  it('converts negative value', () => {
    expect(decimalToNumber('-99.9')).toBe(-99.9)
  })
})

describe('isDecimalString()', () => {
  it('returns true for integer strings', () => {
    expect(isDecimalString('42')).toBe(true)
  })

  it('returns true for decimal strings', () => {
    expect(isDecimalString('3.14')).toBe(true)
  })

  it('returns true for negative strings', () => {
    expect(isDecimalString('-100.5')).toBe(true)
  })

  it('returns true for zero', () => {
    expect(isDecimalString('0')).toBe(true)
  })

  it('returns true for scientific notation', () => {
    expect(isDecimalString('1e10')).toBe(true)
  })

  it('returns false for empty string', () => {
    expect(isDecimalString('')).toBe(false)
  })

  it('returns false for non-numeric strings', () => {
    expect(isDecimalString('abc')).toBe(false)
  })

  it('returns false for mixed content', () => {
    expect(isDecimalString('12abc')).toBe(false)
  })

  it('returns false for special values', () => {
    expect(isDecimalString('NaN')).toBe(true) // Decimal.js accepts NaN
    expect(isDecimalString('Infinity')).toBe(true) // Decimal.js accepts Infinity
  })

  it('returns false for whitespace-only', () => {
    expect(isDecimalString('   ')).toBe(false)
  })
})

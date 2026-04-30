import '@testing-library/jest-dom/vitest'
import '~/i18n/config'

Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
})

Object.defineProperty(window, 'scrollTo', {
  writable: true,
  value: vi.fn(),
})

Object.defineProperty(Element.prototype, 'scrollIntoView', {
  writable: true,
  value: vi.fn(),
})

if (typeof globalThis.ResizeObserver === 'undefined') {
  class ResizeObserverPolyfill {
    observe(): void {}
    unobserve(): void {}
    disconnect(): void {}
  }
  globalThis.ResizeObserver = ResizeObserverPolyfill as unknown as typeof ResizeObserver
}

const elementProto = globalThis.Element?.prototype as
  | (Element & { hasPointerCapture?: unknown, releasePointerCapture?: unknown, scrollIntoView?: unknown })
  | undefined
if (elementProto) {
  if (typeof elementProto.hasPointerCapture !== 'function')
    elementProto.hasPointerCapture = (() => false) as typeof Element.prototype.hasPointerCapture
  if (typeof elementProto.releasePointerCapture !== 'function')
    elementProto.releasePointerCapture = (() => {}) as typeof Element.prototype.releasePointerCapture
  if (typeof elementProto.scrollIntoView !== 'function')
    elementProto.scrollIntoView = (() => {}) as typeof Element.prototype.scrollIntoView
}

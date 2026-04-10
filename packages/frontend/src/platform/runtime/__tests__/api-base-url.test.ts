import { getApiBaseUrl, normalizeApiBaseUrl, setApiBaseUrl } from '../api-base-url'

describe('api-base-url', () => {
  beforeEach(() => {
    delete (globalThis as { __VOLETU_API_BASE_URL__?: string }).__VOLETU_API_BASE_URL__
  })

  it('normalizes trailing slashes', () => {
    expect(normalizeApiBaseUrl('http://localhost:3000///')).toBe('http://localhost:3000')
  })

  it('returns the default base URL when no override is active', () => {
    expect(getApiBaseUrl()).toBe('http://127.0.0.1:3000')
  })

  it('returns the normalized runtime override after activation', () => {
    setApiBaseUrl('http://custom:8080///')
    expect(getApiBaseUrl()).toBe('http://custom:8080')
  })
})

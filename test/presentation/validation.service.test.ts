import { describe, expect, it } from 'vitest';
import {
  isStrongPassword,
  isValidEmail,
  isValidPhone,
  sanitizeText,
} from '../../src/lib/services/validation.service';

describe('validation.service', () => {
  it('validates email format', () => {
    expect(isValidEmail('user@example.com')).toBe(true);
    expect(isValidEmail('bad-email')).toBe(false);
  });

  it('validates phone format', () => {
    expect(isValidPhone('+8613800000000')).toBe(true);
    expect(isValidPhone('123')).toBe(false);
  });

  it('checks password strength', () => {
    expect(isStrongPassword('12345678')).toBe(true);
    expect(isStrongPassword('12345')).toBe(false);
  });

  it('sanitizes text by trimming', () => {
    expect(sanitizeText('  hello  ')).toBe('hello');
  });
});

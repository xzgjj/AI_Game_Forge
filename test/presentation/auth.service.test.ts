import { describe, expect, it } from 'vitest';
import {
  buildSession,
  loginWithEmail,
  loginWithOAuth,
  loginWithPhone,
  loginWithWechat,
  registerByEmail,
} from '../../src/lib/services/auth.service';

describe('auth.service', () => {
  it('returns error for invalid email login payload', async () => {
    const result = await loginWithEmail('bad-email', '123');
    expect(result.success).toBe(false);
    expect(result.error).toBeTruthy();
  });

  it('returns mock success for valid email login in non-tauri runtime', async () => {
    const result = await loginWithEmail('tester@example.com', '12345678');
    expect(result.success).toBe(true);
    expect(result.user?.email).toBe('tester@example.com');
    expect(result.token).toBeTruthy();
  });

  it('returns success for valid wechat/phone/oauth flows', async () => {
    const wechat = await loginWithWechat('wechat-code');
    const phone = await loginWithPhone('+8613800000000', '1234');
    const oauth = await loginWithOAuth('github', 'oauth-code');

    expect(wechat.success).toBe(true);
    expect(phone.success).toBe(true);
    expect(oauth.success).toBe(true);
  });

  it('returns error for invalid register payload', async () => {
    const result = await registerByEmail('bad-email', '123', '12');
    expect(result.success).toBe(false);
  });

  it('builds auth session shape', () => {
    const session = buildSession('token-1', 'user-1');
    expect(session.user_id).toBe('user-1');
    expect(session.token).toBe('token-1');
    expect(session.device_type).toBe('desktop');
  });
});

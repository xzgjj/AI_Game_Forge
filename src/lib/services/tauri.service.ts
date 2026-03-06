export type InvokePayload = Record<string, unknown>;

function hasWindow(): boolean {
  return typeof window !== 'undefined';
}

export function isTauriRuntime(): boolean {
  if (!hasWindow()) {
    return false;
  }

  return '__TAURI_INTERNALS__' in window;
}

async function getInvoke(): Promise<(<T>(cmd: string, args?: InvokePayload) => Promise<T>) | null> {
  if (!isTauriRuntime()) {
    return null;
  }

  const mod = await import('@tauri-apps/api/core');
  return mod.invoke as <T>(cmd: string, args?: InvokePayload) => Promise<T>;
}

export async function safeInvoke<T>(
  cmd: string,
  args: InvokePayload,
  fallback: () => Promise<T> | T,
): Promise<T> {
  const invoke = await getInvoke();

  if (!invoke) {
    return await fallback();
  }

  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    console.warn(`[tauri] invoke failed for ${cmd}, using fallback`, error);
    return await fallback();
  }
}

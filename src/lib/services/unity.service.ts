import type {
  UnityInitPayload,
  UnityUpmPayload,
  UnityValidatePayload,
  UnityBatchValidatePayload,
  UnityBridgeResult,
  UnityValidationReport,
  UnityBatchValidateReport,
} from '$lib/types/unity.types';
import { safeInvoke } from '$lib/services/tauri.service';

const FALLBACK_WARNING = '当前未运行在 Tauri 环境，使用前端模拟结果。';

export async function unityInitProject(payload: UnityInitPayload): Promise<UnityBridgeResult> {
  return safeInvoke<UnityBridgeResult>(
    'unity_init_project',
    {
      project_root: payload.projectRoot,
      unity_version: payload.unityVersion,
      template_preset: payload.templatePreset,
      scene_name: payload.sceneName,
      use_urp: payload.useUrp,
      use_input_system: payload.useInputSystem,
    },
    async () => ({
      project_root: payload.projectRoot,
      created: [],
      warnings: [FALLBACK_WARNING],
    }),
  );
}

export async function unityInjectUpm(payload: UnityUpmPayload): Promise<UnityBridgeResult> {
  return safeInvoke<UnityBridgeResult>(
    'unity_inject_upm',
    {
      project_root: payload.projectRoot,
      package_name: payload.packageName ?? null,
      display_name: payload.displayName ?? null,
      version: payload.version ?? null,
      unity: payload.unity ?? null,
      description: payload.description ?? null,
    },
    async () => ({
      project_root: payload.projectRoot,
      created: [],
      warnings: [FALLBACK_WARNING],
    }),
  );
}

export async function unityValidateProject(payload: UnityValidatePayload): Promise<UnityValidationReport> {
  return safeInvoke<UnityValidationReport>(
    'unity_validate_project',
    {
      project_root: payload.projectRoot,
      require_package: payload.requirePackage,
    },
    async () => ({
      project_root: payload.projectRoot,
      ok: true,
      errors: [],
      warnings: [FALLBACK_WARNING],
      checked_files: [],
    }),
  );
}

export async function unityBatchValidate(payload: UnityBatchValidatePayload): Promise<UnityBatchValidateReport> {
  return safeInvoke<UnityBatchValidateReport>(
    'unity_batch_validate',
    {
      project_root: payload.projectRoot,
      editor_path: payload.editorPath,
      log_file: payload.logFile ?? null,
    },
    async () => ({
      project_root: payload.projectRoot,
      ok: true,
      exit_code: null,
      log_file: payload.logFile ?? null,
      log_tail: FALLBACK_WARNING,
      warnings: [FALLBACK_WARNING],
    }),
  );
}

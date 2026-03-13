export interface UnityInitPayload {
  projectRoot: string;
  unityVersion: string;
  templatePreset: string;
  sceneName: string;
  useUrp: boolean;
  useInputSystem: boolean;
}

export interface UnityUpmPayload {
  projectRoot: string;
  packageName?: string;
  displayName?: string;
  version?: string;
  unity?: string;
  description?: string;
}

export interface UnityValidatePayload {
  projectRoot: string;
  requirePackage: boolean;
}

export interface UnityBatchValidatePayload {
  projectRoot: string;
  editorPath: string;
  logFile?: string;
}

export interface UnityBridgeResult {
  project_root: string;
  created: string[];
  warnings: string[];
}

export interface UnityValidationReport {
  project_root: string;
  ok: boolean;
  errors: string[];
  warnings: string[];
  checked_files: string[];
}

export interface UnityBatchValidateReport {
  project_root: string;
  ok: boolean;
  exit_code: number | null;
  log_file: string | null;
  log_tail: string | null;
  warnings: string[];
}

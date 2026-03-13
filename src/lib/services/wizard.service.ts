import type { WizardState } from '$lib/types/wizard.types';
import { safeInvoke } from '$lib/services/tauri.service';

interface SaveWizardStateRaw {
  project_root: string;
  updated_at: string;
}

interface WizardStateRecordRaw {
  project_root: string;
  wizard_state: WizardState;
  updated_at: string;
}

export interface SaveWizardStateResult {
  projectRoot: string;
  updatedAt: string;
}

export interface WizardStateRecord {
  projectRoot: string;
  wizardState: WizardState;
  updatedAt: string;
}

export async function saveWizardState(projectRoot: string, wizardState: WizardState): Promise<SaveWizardStateResult> {
  const raw = await safeInvoke<SaveWizardStateRaw>(
    'save_wizard_state',
    {
      project_root: projectRoot,
      wizard_state: wizardState,
    },
    async () => ({
      project_root: projectRoot,
      updated_at: new Date().toISOString(),
    }),
  );

  return {
    projectRoot: raw.project_root ?? projectRoot,
    updatedAt: raw.updated_at ?? new Date().toISOString(),
  };
}

export async function loadLatestWizardState(): Promise<WizardStateRecord | null> {
  const raw = await safeInvoke<WizardStateRecordRaw | null>(
    'load_latest_wizard_state',
    {},
    async () => null,
  );

  if (!raw) {
    return null;
  }

  return {
    projectRoot: raw.project_root,
    wizardState: raw.wizard_state,
    updatedAt: raw.updated_at,
  };
}

export async function loadWizardStateByProject(projectRoot: string): Promise<WizardStateRecord | null> {
  const raw = await safeInvoke<WizardStateRecordRaw | null>(
    'load_wizard_state_by_project',
    { project_root: projectRoot },
    async () => null,
  );

  if (!raw) {
    return null;
  }

  return {
    projectRoot: raw.project_root,
    wizardState: raw.wizard_state,
    updatedAt: raw.updated_at,
  };
}

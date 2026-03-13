export type WizardStepId =
  | 'design'
  | 'architecture'
  | 'unity-init'
  | 'core-scripts'
  | 'characters'
  | 'assets'
  | 'iteration'
  | 'release';

export type StepStatus = 'pending' | 'active' | 'complete';

export interface WizardStepMeta {
  id: WizardStepId;
  title: string;
  subtitle: string;
}

export interface DesignBrief {
  gameTitle: string;
  genre: string;
  coreLoop: string;
  narrativeTone: string;
  artStyle: string;
  targetPlatforms: string[];
  scopeNotes: string;
}

export interface ArchitectureSpec {
  systemModules: string;
  scriptStructure: string;
  stateMachineNotes: string;
  dataFlowNotes: string;
}

export interface UnityInitSpec {
  unityVersion: string;
  templatePreset: string;
  projectName: string;
  projectPath: string;
  sceneName: string;
  useURP: boolean;
  useInputSystem: boolean;
  unityEditorPath: string;
  enableBatchValidation: boolean;
}

export interface CoreScriptSpec {
  directGenerate: string[];
  assistedGenerate: string[];
  dialogueStyle: string;
  portraitStyle: string;
  scriptNotes: string;
}

export interface CharacterSpec {
  playableCount: number;
  enemyArchetypes: string;
  npcCount: number;
  animationStyle: string;
  voiceRequirement: string;
}

export interface AssetSpec {
  environmentStyle: string;
  uiTheme: string;
  vfxStyle: string;
  audioStyle: string;
  assetNotes: string;
}

export interface IterationSpec {
  iterationGoals: string;
  balanceTargets: string;
  skillAdjustments: string;
  playtestNotes: string;
}

export interface ReleaseSpec {
  buildTargets: string[];
  versionTag: string;
  releaseNotes: string;
  qaChecklist: string;
}

export interface WizardState {
  stepIndex: number;
  steps: WizardStepMeta[];
  design: DesignBrief;
  architecture: ArchitectureSpec;
  unityInit: UnityInitSpec;
  coreScripts: CoreScriptSpec;
  characters: CharacterSpec;
  assets: AssetSpec;
  iteration: IterationSpec;
  release: ReleaseSpec;
  updatedAt: string;
}

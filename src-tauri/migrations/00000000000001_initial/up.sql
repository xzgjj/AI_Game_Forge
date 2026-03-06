CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY NOT NULL,
  email TEXT UNIQUE,
  phone TEXT UNIQUE,
  username TEXT NOT NULL,
  display_name TEXT,
  avatar_url TEXT,
  role TEXT NOT NULL,
  status TEXT NOT NULL,
  preferences TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  last_login_at TIMESTAMP,
  login_count INTEGER NOT NULL DEFAULT 0,
  total_spent REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS projects (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL,
  tags TEXT NOT NULL DEFAULT '[]',
  config_id TEXT,
  version INTEGER NOT NULL DEFAULT 1,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  last_accessed TIMESTAMP NOT NULL,
  total_cost REAL NOT NULL DEFAULT 0,
  is_template BOOLEAN NOT NULL DEFAULT 0,
  template_source TEXT,
  metadata TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS ai_logs (
  id TEXT PRIMARY KEY NOT NULL,
  project_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  provider_name TEXT NOT NULL,
  model_name TEXT NOT NULL,
  prompt TEXT NOT NULL,
  response TEXT NOT NULL,
  status TEXT NOT NULL,
  tokens_used INTEGER NOT NULL DEFAULT 0,
  cost REAL NOT NULL DEFAULT 0,
  response_time_ms BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL,
  completed_at TIMESTAMP,
  error_message TEXT,
  metadata TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (project_id) REFERENCES projects(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS game_specs (
  id TEXT PRIMARY KEY NOT NULL,
  project_id TEXT NOT NULL,
  game_type TEXT NOT NULL,
  art_style TEXT NOT NULL,
  narrative_style TEXT NOT NULL,
  target_platform TEXT NOT NULL,
  age_rating TEXT NOT NULL,
  theme TEXT NOT NULL DEFAULT '',
  setting TEXT NOT NULL DEFAULT '',
  main_characters TEXT NOT NULL DEFAULT '[]',
  key_locations TEXT NOT NULL DEFAULT '[]',
  core_mechanics TEXT NOT NULL DEFAULT '[]',
  story_outline TEXT NOT NULL DEFAULT '',
  visual_references TEXT NOT NULL DEFAULT '[]',
  audio_style TEXT NOT NULL DEFAULT '',
  ui_style TEXT NOT NULL DEFAULT '',
  advanced_settings TEXT NOT NULL DEFAULT '{}',
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  version INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS api_stats (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  project_id TEXT,
  provider_name TEXT NOT NULL,
  endpoint TEXT NOT NULL,
  request_count INTEGER NOT NULL DEFAULT 0,
  token_count INTEGER NOT NULL DEFAULT 0,
  cost REAL NOT NULL DEFAULT 0,
  success_count INTEGER NOT NULL DEFAULT 0,
  error_count INTEGER NOT NULL DEFAULT 0,
  total_response_time_ms BIGINT NOT NULL DEFAULT 0,
  date DATE NOT NULL,
  hour INTEGER,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS auth_sessions (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  auth_method TEXT NOT NULL,
  device_id TEXT NOT NULL,
  device_type TEXT NOT NULL,
  user_agent TEXT NOT NULL,
  ip_address TEXT,
  token TEXT NOT NULL,
  refresh_token TEXT,
  status TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  last_accessed TIMESTAMP NOT NULL,
  revoked_at TIMESTAMP,
  revocation_reason TEXT,
  metadata TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_projects_user_id ON projects(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_logs_project_id ON ai_logs(project_id);
CREATE INDEX IF NOT EXISTS idx_ai_logs_user_id ON ai_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_api_stats_user_provider_date ON api_stats(user_id, provider_name, date);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_user_status ON auth_sessions(user_id, status);

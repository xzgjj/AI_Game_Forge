DROP INDEX IF EXISTS idx_auth_sessions_user_status;
DROP INDEX IF EXISTS idx_api_stats_user_provider_date;
DROP INDEX IF EXISTS idx_ai_logs_user_id;
DROP INDEX IF EXISTS idx_ai_logs_project_id;
DROP INDEX IF EXISTS idx_projects_user_id;

DROP TABLE IF EXISTS auth_sessions;
DROP TABLE IF EXISTS api_stats;
DROP TABLE IF EXISTS game_specs;
DROP TABLE IF EXISTS ai_logs;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS users;

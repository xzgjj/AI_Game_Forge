//! 数据库 schema 定义
//!
//! 说明：
//! 1) 当前为手写最小 schema，后续可通过 Diesel CLI 自动生成并替换。
//! 2) 字段命名与现有模型保持尽可能一致，JSON 数据使用 TEXT 存储。

use diesel::{allow_tables_to_appear_in_same_query, table};

table! {
    users (id) {
        id -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        username -> Text,
        display_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        role -> Text,
        status -> Text,
        preferences -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login_at -> Nullable<Timestamp>,
        login_count -> Integer,
        total_spent -> Double,
    }
}

table! {
    projects (id) {
        id -> Text,
        user_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        status -> Text,
        tags -> Text,
        config_id -> Nullable<Text>,
        version -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_accessed -> Timestamp,
        total_cost -> Double,
        is_template -> Bool,
        template_source -> Nullable<Text>,
        metadata -> Text,
    }
}

table! {
    ai_logs (id) {
        id -> Text,
        project_id -> Text,
        user_id -> Text,
        provider_name -> Text,
        model_name -> Text,
        prompt -> Text,
        response -> Text,
        status -> Text,
        tokens_used -> Integer,
        cost -> Double,
        response_time_ms -> BigInt,
        created_at -> Timestamp,
        completed_at -> Nullable<Timestamp>,
        error_message -> Nullable<Text>,
        metadata -> Text,
    }
}

table! {
    game_specs (id) {
        id -> Text,
        project_id -> Text,
        game_type -> Text,
        art_style -> Text,
        narrative_style -> Text,
        target_platform -> Text,
        age_rating -> Text,
        theme -> Text,
        setting -> Text,
        main_characters -> Text,
        key_locations -> Text,
        core_mechanics -> Text,
        story_outline -> Text,
        visual_references -> Text,
        audio_style -> Text,
        ui_style -> Text,
        advanced_settings -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        version -> Integer,
    }
}

table! {
    api_stats (id) {
        id -> Text,
        user_id -> Text,
        project_id -> Nullable<Text>,
        provider_name -> Text,
        endpoint -> Text,
        request_count -> Integer,
        token_count -> Integer,
        cost -> Double,
        success_count -> Integer,
        error_count -> Integer,
        total_response_time_ms -> BigInt,
        date -> Date,
        hour -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    auth_sessions (id) {
        id -> Text,
        user_id -> Text,
        auth_method -> Text,
        device_id -> Text,
        device_type -> Text,
        user_agent -> Text,
        ip_address -> Nullable<Text>,
        token -> Text,
        refresh_token -> Nullable<Text>,
        status -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        last_accessed -> Timestamp,
        revoked_at -> Nullable<Timestamp>,
        revocation_reason -> Nullable<Text>,
        metadata -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    projects,
    ai_logs,
    game_specs,
    api_stats,
    auth_sessions,
);

use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::handlers::AppState;
use crate::dtos::admin::{SetRolePermissionsPayload, SetUserRolesPayload};
use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

// 设置用户角色的处理器
pub async fn set_user_roles_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<SetUserRolesPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("user:manage_roles")?;

    tracing::info!(
        "管理员 {} 正在为用户 {} 设置角色: {:?}",
        auth_user.username(),
        target_user_id,
        payload.role_ids
    );

    state
        .admin_service
        .set_user_roles(target_user_id, &payload.role_ids)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// 设置角色权限的处理器
pub async fn set_role_permissions_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(role_id):Path<Uuid>,
    Json(payload):Json<SetRolePermissionsPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;

    tracing::info!(
        "管理员 {} 正在为角色 {} 设置权限: {:?}",
        auth_user.username(),
        role_id,
        payload.permission_ids
    );
    
    state.admin_service
        .set_role_permissions(role_id,&payload.permission_ids)
        .await?;
    
    Ok(StatusCode::NO_CONTENT)
}
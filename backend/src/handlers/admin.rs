use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::admin::{CreatePermissionPayload, CreateRolePayload, SetRolePermissionsPayload, SetUserRolesPayload, UpdatePermissionPayload, UpdateRolePayload};
use crate::handlers::AppState;
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
    Path(role_id): Path<Uuid>,
    Json(payload): Json<SetRolePermissionsPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;

    tracing::info!(
        "管理员 {} 正在为角色 {} 设置权限: {:?}",
        auth_user.username(),
        role_id,
        payload.permission_ids
    );

    state.admin_service
        .set_role_permissions(role_id, &payload.permission_ids)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// 获取用户列表的处理器
pub async fn list_users_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 授权检查
    auth_user.require_permission("user:list")?;
    let users = state.admin_service.list_users().await?;
    Ok(Json(users))
}

// 获取角色列表的处理器
pub async fn list_roles_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 复用 'user:manage_roles' 权限，因为能管理角色的用户通常也需要能看到角色列表
    auth_user.require_permission("user:manage_roles")?;
    let roles = state.admin_service.list_roles().await?;
    Ok(Json(roles))
}
// 创建角色
pub async fn create_role_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRolePayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    let new_role = state.admin_service.create_role(payload).await?;
    Ok((StatusCode::CREATED, Json(new_role)))
}
// 更新角色
pub async fn update_role_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
    Json(payload): Json<UpdateRolePayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    let updated_role = state.admin_service.update_role(role_id, payload).await?;
    Ok(Json(updated_role))
}
// 删除角色
pub async fn delete_role_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    state.admin_service.delete_role(role_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
// 获取权限列表的处理器
pub async fn list_permissions_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 同样，复用一个高级权限
    auth_user.require_permission("role:manage_permissions")?;
    let permissions = state.admin_service.list_permissions().await?;
    Ok(Json(permissions))
}

// 创建权限
pub async fn create_permission_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreatePermissionPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    let new_permission = state.admin_service.create_permission(payload).await?;
    Ok((StatusCode::CREATED, Json(new_permission)))
}
// 更新权限
pub async fn update_permission_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(permission_id): Path<Uuid>,
    Json(payload): Json<UpdatePermissionPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    let updated_permission = state.admin_service.update_permission(permission_id, payload).await?;
    Ok(Json(updated_permission))
}
// 删除权限
pub async fn delete_permission_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(permission_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("role:manage_permissions")?;
    state.admin_service.delete_permission(permission_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
use crate::api_error::ApiError;
use crate::auth::AuthUser;
use crate::dtos::admin::{
    CreatePermissionPayload, CreateRolePayload, SetRolePermissionsPayload,
    SetUserRolesPayload, UpdatePermissionPayload, UpdateRolePayload,
};
use crate::dtos::user::{CreateUserPayload, ResetUserPasswordPayload, UpdateUserPayload};
use crate::handlers::AppState;
use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

// 设置用户角色的处理器
pub async fn set_user_roles_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<SetUserRolesPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_management")?;

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
    auth_user.require_permission("admin:role_management")?;

    tracing::info!(
        "管理员 {} 正在为角色 {} 设置权限: {:?}",
        auth_user.username(),
        role_id,
        payload.permission_ids
    );

    state
        .admin_service
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
    auth_user.require_permission("admin:user_list")?;
    let users = state.admin_service.list_users().await?;
    Ok(Json(users))
}

// 获取角色列表的处理器
pub async fn list_roles_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 需要角色管理权限才能查看角色列表
    auth_user.require_permission("admin:role_management")?;
    let roles = state.admin_service.list_roles().await?;
    Ok(Json(roles))
}

// 获取包含权限信息的角色列表的处理器
pub async fn list_roles_with_permissions_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 需要角色管理权限才能查看角色列表
    auth_user.require_permission("admin:role_management")?;
    let roles = state.admin_service.list_roles_with_permissions().await?;
    Ok(Json(roles))
}

// 创建角色
pub async fn create_role_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRolePayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:role_management")?;
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
    auth_user.require_permission("admin:role_management")?;
    let updated_role = state.admin_service.update_role(role_id, payload).await?;
    Ok(Json(updated_role))
}

// 删除角色
pub async fn delete_role_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:role_management")?;
    state.admin_service.delete_role(role_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 获取权限列表的处理器
pub async fn list_permissions_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 需要查看权限的权限
    auth_user.require_permission("admin:view_permissions")?;
    let permissions = state.admin_service.list_permissions().await?;
    Ok(Json(permissions))
}

// 获取当前用户所有的权限列表(每个角色拥有的权限的并集)
pub async fn list_user_permissions_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    // 不需要特殊权限，任何登录用户都可以查看自己的权限
    let user_id = auth_user.user_id();
    let permissions = state
        .admin_service
        .list_user_roles_permissions(user_id)
        .await?;
    Ok(Json(permissions))
}

// 创建权限
pub async fn create_permission_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreatePermissionPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:role_management")?;
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
    auth_user.require_permission("admin:role_management")?;
    let updated_permission = state
        .admin_service
        .update_permission(permission_id, payload)
        .await?;
    Ok(Json(updated_permission))
}

// 删除权限
pub async fn delete_permission_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(permission_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:role_management")?;
    state.admin_service.delete_permission(permission_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 获取仪表板统计数据
pub async fn get_dashboard_stats_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 检查权限 - 需要查看系统统计的权限
    auth_user.require_permission("admin:view_statistics")?;

    let stats = state.admin_service.get_dashboard_stats().await?;
    Ok(Json(stats))
}

// 获取用户统计数据（管理员专用）
pub async fn get_user_stats_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // 检查权限 - 需要用户管理权限
    auth_user.require_permission("admin:user_management")?;

    let stats = state.admin_service.get_user_stats().await?;
    Ok(Json(stats))
}

// === 用户管理相关的handler ===

// 管理员创建用户
pub async fn create_user_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_management")?;

    let user = state.user_service.admin_create_user(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

// 管理员获取用户详情
pub async fn get_user_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_list")?;

    let user = state.user_service.admin_get_user(user_id).await?;
    Ok(Json(user))
}

// 管理员更新用户
pub async fn update_user_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_management")?;

    let user = state
        .user_service
        .admin_update_user(user_id, payload)
        .await?;
    Ok(Json(user))
}

// 管理员删除用户
pub async fn delete_user_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_management")?;

    state.user_service.admin_delete_user(user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// 管理员重置用户密码
pub async fn reset_user_password_handler(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<ResetUserPasswordPayload>,
) -> Result<impl IntoResponse, ApiError> {
    auth_user.require_permission("admin:user_management")?;

    state
        .user_service
        .admin_reset_user_password(user_id, payload)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

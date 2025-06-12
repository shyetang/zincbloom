use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

// 密码哈希(使用 argon2id)
pub fn hash_password(password: &str) -> anyhow::Result<String> {
    // SaltString 会使用 OsRng 自动生成一个加密安全的盐值
    let salt = SaltString::generate(&mut OsRng);

    // Argon2::default() 会使用推荐的、安全的默认参数。
    let argon2 = Argon2::default();

    // 进行哈希计算
    // .hash_password() 方法接收密码字节和盐，返回一个 PasswordHash 对象
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("Argon2 哈希密码失败: {}", e))?;
    //PasswordHash: 这是一个核心的抽象。它代表了一个完整的密码哈希字符串（遵循 PHC 字符串格式），
    // 包含了算法、版本、参数、盐和哈希值本身。这使得验证过程更简单、更不容易出错，
    // 因为我们只需传递整个哈希字符串，验证函数会自动解析出所需的一切。

    // 将 PasswordHash 对象转换为 PHC 格式的字符串，以便存储在数据库中
    // 格式类似于： $argon2id$v=19$m=19456,t=2,p=1$SALT_HERE$HASH_HERE
    Ok(password_hash.to_string())
}

// 密码验证
pub fn verify_password(hash_str: &str, password: &str) -> anyhow::Result<bool> {
    // 从数据库取出的哈希字符串中解析出所有信息（算法、版本、参数、盐、哈希值）
    let parsed_hash =
        PasswordHash::new(hash_str).map_err(|e| anyhow!("解析密码哈希字符串失败: {}", e))?;

    // 使用 Argon2::default() 实例来验证密码
    // .verify_password() 方法会从 parsed_hash 中自动提取盐和参数来进行比较
    let verification_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match verification_result {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false), // 密码不匹配
        Err(e) => Err(anyhow!("密码验证过程中发生错误: {}", e)),  // 其他错误，例如哈希格式不兼容等
    }
}

// 密码强度验证
pub fn validate_password_strength(password: &str) -> anyhow::Result<()> {
    if password.len() < 8 {
        return Err(anyhow!("密码无效：长度不能少于8个字符"));
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(anyhow!("密码无效：必须包含至少一个大写字母"));
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(anyhow!("密码无效：必须包含至少一个小写字母"));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(anyhow!("密码无效：必须包含至少一个数字"));
    }
    if !password.chars().any(|c| c.is_alphanumeric()) {
        return Err(anyhow!("密码无效：必须包含至少一个特殊字符"));
    }
    Ok(())
}
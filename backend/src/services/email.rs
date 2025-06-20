use crate::config::EmailConfig;
use anyhow::{Context, Result};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[derive(Clone)]
pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    // 发送邮件
    pub async fn send_email(&self, to_address: &str, subject: &str, html_body: &str) -> Result<()> {
        let email = Message::builder()
            .from(self.config.from_address.parse()?)
            .to(to_address.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(html_body))
            .context("构建邮件失败")?;

        // 先创建一个基础的 mailer builder
        let mut mailer_builder =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.smtp_host)
                .port(self.config.smtp_port);

        // 只有在配置中提供了 smtp_user 的情况下，才添加认证信息
        if !self.config.smtp_user.is_empty() {
            let credentials =
                Credentials::new(self.config.smtp_user.clone(), self.config.smtp_pass.clone());
            mailer_builder = mailer_builder.credentials(credentials);
        }

        // 构建最终的 mailer
        let mailer = mailer_builder.build();

        // 发送邮件
        mailer.send(email).await.context("发送邮件失败")?;

        tracing::info!("邮件已成功发送至: {}", to_address);

        Ok(())
    }
}

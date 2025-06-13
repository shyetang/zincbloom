use crate::config::EmailConfig;
use anyhow::{Context, Result};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

#[derive(Clone)]
pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
    
    // 发送邮件
    pub async fn send_email(
        &self,
        to_address:&str,
        subject: &str,
        html_body: &str,
    ) -> Result<()> {
        let email = Message::builder()
            .from(self.config.from_address.parse()?)
            .to(to_address.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(html_body))
            .context("构建邮件失败")?;
        
        let credentials = if !self.config.smtp_user.is_empty() { 
            Credentials::new(self.config.smtp_user.clone(),self.config.smtp_pass.clone())
        }else { 
            Credentials::new("".to_string(),"".to_string())
        };
        
        // 创建 mailer
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.config.smtp_host)?
            .credentials(credentials)
            .build();
        
        // 发送邮件
        mailer.send(email).await.context("发送邮件失败")?;
        
        tracing::info!("邮件已成功发送至: {}", to_address);
        
        Ok(())
    }
}


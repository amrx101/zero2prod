//! src/email_client.rs

use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use std::time::Duration;

pub struct EmailClient {
    http_client: Client,
    sender: SubscriberEmail,
    sender_api_url: String,
    authorization_token: Secret<String>,
    timeout: Duration,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub fn new(
        sender: SubscriberEmail,
        sender_api_url: String,
        authorization_token: Secret<String>,
        timeout: Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client
            sender,
            sender_api_url,
            authorization_token,
            timeout,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject: subject.as_ref(),
            html_body: html_content.as_ref(),
            text_body: text_content.as_ref(),
        };
        let url = format!("{}/email", self.sender_api_url);
        self.http_client
            .post(&url)
            .header("Authorization", self.authorization_token.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

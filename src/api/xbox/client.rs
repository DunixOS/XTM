use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use crate::api::xbox::error::XboxApiError;
use crate::api::xbox::models::{FollowEvent, FollowProgress, XstsAuthorizeRequest, XstsAuthorizeResponse};
use crate::models::TokenRecord;

#[derive(Debug, Clone)]
pub struct XboxApiClient {
    client: Client,
    xsts_cache: Arc<Mutex<HashMap<String, String>>>,
}

impl XboxApiClient {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            xsts_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn authenticate(&self, token: &TokenRecord) -> Result<String, XboxApiError> {
        if let Some(cached) = self.xsts_cache.lock().await.get(&token.token) {
            return Ok(cached.clone());
        }

        let request = XstsAuthorizeRequest {
            properties: crate::api::xbox::models::XstsAuthorizeProperties {
                sandbox_id: "RETAIL".to_string(),
                user_tokens: vec![token.token.clone()],
            },
            relying_party: "http://xboxlive.com".to_string(),
            token_type: "JWT".to_string(),
        };

        let response = self
            .client
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&request)
            .send()
            .await
            .map_err(XboxApiError::from)?;

        let status = response.status().as_u16();
        let body = response.text().await.map_err(XboxApiError::from)?;
        if body.trim().is_empty() {
            return Err(XboxApiError::RequestStatus {
                status,
                body: body.clone(),
            });
        }

        let parsed: Option<Value> = serde_json::from_str(&body).ok();
        if let Some(parsed) = parsed.as_ref() {
            if let Some(error_code) = parsed.get("XErr")
                .and_then(|value| value.as_u64())
                .filter(|code| *code == 2148916233 || *code == 2148916238)
            {
                return Err(XboxApiError::ExpiredToken(format!("authentication expired ({error_code})")));
            }
        }

        let response: XstsAuthorizeResponse = serde_json::from_str(&body)
            .map_err(|err| XboxApiError::Serialization(err.to_string()))?;
        let uhs = response
            .display_claims
            .xui
            .first()
            .map(|entry| entry.uhs.clone())
            .context("missing uhs claim")?;
        let header = format!("XBL3.0 x={uhs};{}", response.token);

        self.xsts_cache.lock().await.insert(token.token.clone(), header.clone());
        Ok(header)
    }

    pub async fn follow_target(&self, token: &TokenRecord, target: &str) -> Result<bool, XboxApiError> {
        let header = self.authenticate(token).await?;
        let url = format!("https://social.xboxlive.com/users/me/people/gt({target})");
        let response = self
            .client
            .put(&url)
            .header("Authorization", header)
            .header("X-XBL-Contract-Version", "2")
            .send()
            .await
            .map_err(XboxApiError::from)?;

        match response.status().as_u16() {
            200 | 201 | 202 | 204 => Ok(true),
            _ => Ok(false),
        }
    }

    fn build_message_request(recipient: &str, body: &str) -> Value {
        serde_json::json!({
            "recipient": recipient,
            "body": body,
            "messageType": "text"
        })
    }

    pub async fn send_message(&self, token: &TokenRecord, recipient: &str, body: &str) -> Result<String, XboxApiError> {
        let header = self.authenticate(token).await?;
        let request = Self::build_message_request(recipient, body);
        let response = self
            .client
            .post("https://social.xboxlive.com/users/me/messages")
            .header("Authorization", header)
            .header("X-XBL-Contract-Version", "2")
            .json(&request)
            .send()
            .await
            .map_err(XboxApiError::from)?;

        let status = response.status();
        let body_text = response.text().await.map_err(XboxApiError::from)?;
        if status.is_success() {
            Ok(body_text)
        } else {
            Err(XboxApiError::RequestStatus {
                status: status.as_u16(),
                body: body_text,
            })
        }
    }

    pub async fn run_follow_workflow(&self, tokens: &[TokenRecord], target: &str, limit: Option<usize>, sender: Option<mpsc::UnboundedSender<FollowEvent>>) -> anyhow::Result<FollowProgress> {
        let selected = match limit {
            Some(count) => tokens.iter().take(count).cloned().collect::<Vec<_>>(),
            None => tokens.to_vec(),
        };

        let mut progress = FollowProgress {
            total_tokens: selected.len(),
            successful_follows: 0,
            failed_follows: 0,
            expired_tokens: 0,
            progress: 0.0,
            current_target: target.to_string(),
            log: Vec::new(),
        };

        let mut set: tokio::task::JoinSet<anyhow::Result<(String, bool, bool)>> = tokio::task::JoinSet::new();
        for token in selected {
            let client = self.clone();
            let target = target.to_string();
            let token_clone = token.clone();
            let event_sender = sender.clone();
            set.spawn(async move {
                let _ = event_sender.as_ref().map(|sender| sender.send(FollowEvent::Started { gamertag: token_clone.gamertag.clone() }));
                let result = client.follow_target(&token_clone, &target).await;
                match result {
                    Ok(true) => {
                        let _ = event_sender.as_ref().map(|sender| sender.send(FollowEvent::Completed { gamertag: token_clone.gamertag.clone(), succeeded: true }));
                        Ok((token_clone.gamertag, true, false))
                    }
                    Ok(false) => {
                        let _ = event_sender.as_ref().map(|sender| sender.send(FollowEvent::Completed { gamertag: token_clone.gamertag.clone(), succeeded: false }));
                        Ok((token_clone.gamertag, false, false))
                    }
                    Err(XboxApiError::ExpiredToken(_)) => {
                        let _ = event_sender.as_ref().map(|sender| sender.send(FollowEvent::Expired { gamertag: token_clone.gamertag.clone(), token: token_clone.token.clone() }));
                        Ok((token_clone.gamertag, false, true))
                    }
                    Err(err) => {
                        let _ = event_sender.as_ref().map(|sender| sender.send(FollowEvent::Completed { gamertag: token_clone.gamertag.clone(), succeeded: false }));
                        let _ = err;
                        Ok((token_clone.gamertag, false, false))
                    }
                }
            });
        }

        while let Some(result) = set.join_next().await {
            let (gamertag, success, expired) = result??;
            if expired {
                progress.expired_tokens += 1;
                progress.log.push(format!("{gamertag}: token expired"));
            } else if success {
                progress.successful_follows += 1;
                progress.log.push(format!("{gamertag}: follow succeeded"));
            } else {
                progress.failed_follows += 1;
                progress.log.push(format!("{gamertag}: follow failed"));
            }
            progress.progress = ((progress.successful_follows + progress.failed_follows + progress.expired_tokens) as f32 / progress.total_tokens as f32) * 100.0;
        }

        let _ = sender.as_ref().map(|sender| sender.send(FollowEvent::Finished { progress: progress.clone() }));
        Ok(progress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_payload_contains_recipient_and_body() {
        let payload = XboxApiClient::build_message_request("bevlynous", "hello from xtm");
        assert_eq!(payload["recipient"], "bevlynous");
        assert_eq!(payload["body"], "hello from xtm");
        assert_eq!(payload["messageType"], "text");
    }
}

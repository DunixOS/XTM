use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::models::TokenRecord;

pub fn default_data_dir() -> anyhow::Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "playfairs", "xtm")
        .context("failed to resolve application data directory")?;
    Ok(dirs.data_dir().to_path_buf())
}

pub fn tokens_path(dir: &Path) -> PathBuf {
    dir.join("tokens.json")
}

pub fn load_tokens_from_file(path: &Path) -> anyhow::Result<Vec<TokenRecord>> {
    let content = fs::read_to_string(path).with_context(|| format!("failed to read tokens from {}", path.display()))?;
    let mut records = Vec::new();
    for (index, line) in content.lines().enumerate() {
        let token = line.trim();
        if token.is_empty() {
            continue;
        }
        records.push(TokenRecord {
            gamertag: format!("Account {index}",),
            xuid: String::new(),
            status: "Ready".to_string(),
            expiration: "Unknown".to_string(),
            token: token.to_string(),
        });
    }
    Ok(records)
}

pub fn load_tokens_from_dir(dir: &Path) -> anyhow::Result<Vec<TokenRecord>> {
    let path = tokens_path(dir);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path).with_context(|| format!("failed to read tokens from {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("failed to parse tokens from {}", path.display()))
}

pub fn save_tokens_to_dir(tokens: &[TokenRecord], dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("failed to create tokens directory {}", dir.display()))?;
    let path = tokens_path(dir);
    let content = serde_json::to_string_pretty(tokens)?;
    fs::write(&path, content).with_context(|| format!("failed to write tokens to {}", path.display()))?;
    Ok(())
}

pub fn load_tokens() -> anyhow::Result<Vec<TokenRecord>> {
    let repo_root = std::env::current_dir()?;
    let tokens_path = repo_root.join("tokens.txt");
    if tokens_path.exists() {
        return load_tokens_from_file(&tokens_path);
    }

    let dir = default_data_dir()?;
    load_tokens_from_dir(&dir)
}

pub fn save_tokens(tokens: &[TokenRecord]) -> anyhow::Result<()> {
    let dir = default_data_dir()?;
    save_tokens_to_dir(tokens, &dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("xtm-tokens-test-{nanos}"))
    }

    #[test]
    fn tokens_round_trip() {
        let dir = temp_dir();
        fs::create_dir_all(&dir).expect("temp dir should be created");

        let tokens = vec![TokenRecord {
            gamertag: "PixelMage".to_string(),
            xuid: "2535440000000000".to_string(),
            status: "Active".to_string(),
            expiration: "2026-08-10".to_string(),
            token: "test-token-value".to_string(),
        }];

        super::save_tokens_to_dir(&tokens, &dir).expect("tokens should be saved");
        let loaded = super::load_tokens_from_dir(&dir).expect("tokens should be loaded");

        assert_eq!(loaded.len(), tokens.len());
        assert_eq!(loaded[0].gamertag, tokens[0].gamertag);
        assert_eq!(loaded[0].xuid, tokens[0].xuid);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tokens_file_is_loaded() {
        let dir = temp_dir();
        fs::create_dir_all(&dir).expect("temp dir should be created");
        let path = dir.join("tokens.txt");
        fs::write(&path, "token-value-1\ntoken-value-2\n").expect("test tokens file should be created");

        let loaded = super::load_tokens_from_file(&path).expect("tokens file should be loaded");

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].token, "token-value-1");
        assert_eq!(loaded[1].token, "token-value-2");
    }
}

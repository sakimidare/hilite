mod logs;
mod json;
mod cpp;

use crate::rules::Rule;
use anyhow::Result;

/// 获取指定名称的预设规则
pub fn get_preset(name: &str) -> Result<Vec<Rule>> {
    match name {
        "logs" => Ok(logs::LOGS.to_vec()),
        "json" => Ok(json::JSON.to_vec()),
        "cpp" => Ok(cpp::CPP.to_vec()),
        _ => anyhow::bail!("Unknown preset '{}'", name),
    }
}

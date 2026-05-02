use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Exec,
    FileOpen,
    NetworkConnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEvent {
    pub host_id: String,
    pub timestamp: DateTime<Utc>,

    pub event_type: EventType,

    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,

    pub process_name: String,
    pub parent_name: Option<String>,
    pub command_line: Option<String>,

    pub file_path: Option<String>,

    pub destination_ip: Option<String>,
    pub destination_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReason {
    pub rule_id: String,
    pub description: String,
    pub score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPassport {
    pub host_id: String,

    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,

    pub process_name: String,
    pub parent_chain: Vec<String>,
    pub command_line: Option<String>,

    pub files_touched: Vec<String>,
    pub network_connections: Vec<String>,
    pub children: Vec<String>,

    pub risk_score: u8,
    pub risk_reasons: Vec<RiskReason>,

    pub ai_summary: Option<String>,
    pub likely_scenario: Option<String>,
    pub recommended_actions: Vec<String>,
}

impl ProcessPassport {
    pub fn severity(&self) -> &'static str {
        match self.risk_score {
            0..=29 => "low",
            30..=59 => "medium",
            60..=84 => "high",
            _ => "critical",
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PatienceOption {
    WAIT,
    PAY,
    DELEGATE,
    OUR_LADY,
    #[serde(rename = "FSCK_OFF")]
    FsckOff,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct X420Envelope {
    pub status: u16,
    pub status_text: String,
    pub ticket_id: String,
    pub queue_position: u32,
    pub eta_seconds: u32,
    pub retry_after_ms: u64,
    pub options: Vec<String>,
    pub timestamp: String,
    pub backpressure_notice: String,
}

impl X420Envelope {
    pub fn new(ticket_id: &str, queue_position: u32, eta_seconds: u32) -> Self {
        Self {
            status: 420,
            status_text: "420 Enhance Your Calm — Patience Protocol Active".to_string(),
            ticket_id: ticket_id.to_string(),
            queue_position,
            eta_seconds,
            retry_after_ms: (eta_seconds as u64) * 1000,
            options: vec![
                "WAIT".to_string(),
                "PAY".to_string(),
                "DELEGATE".to_string(),
                "OUR_LADY".to_string(),
                "FSCK_OFF".to_string(),
            ],
            timestamp: "2026-10-23T00:00:00Z".to_string(),
            backpressure_notice: "Local edge solver slot is saturated (WIP=1). Please choose a patience option or gracefully abort via FSCK OFF.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FsckOffReceipt {
    pub status: String,
    pub ticket_id: String,
    pub command: String,
    pub message: String,
    pub released_queue_slot: bool,
    pub timestamp: String,
}

impl FsckOffReceipt {
    pub fn new(ticket_id: &str) -> Self {
        Self {
            status: "ORDER_ABORTED_POLITELY".to_string(),
            ticket_id: ticket_id.to_string(),
            command: "FSCK_OFF".to_string(),
            message: "File system check off: Queue slot released cleanly with zero client penalty. In You We Trust.".to_string(),
            released_queue_slot: true,
            timestamp: "2026-10-23T00:00:00Z".to_string(),
        }
    }
}

pub fn is_fsck_off_command(action_input: &str) -> bool {
    let clean = action_input.trim().to_uppercase().replace(['-', ' '], "_");
    clean == "FSCK_OFF" || clean == "FSCKOFF"
}

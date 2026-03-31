use serde::Deserialize;

#[cfg(debug_assertions)]
pub const API_BASE: &str = "http://localhost:3000";
#[cfg(not(debug_assertions))]
pub const API_BASE: &str = "https://snaptray.onrender.com";

// ---------------------------------------------------------------------------
// State machine
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    /// Waiting for the user to click "Send Code" (no API call made yet)
    Ready,
    /// POST /2fa in flight
    Loading,
    /// Showing 3 choices, polling GET /2fa/status for mobile approval
    Idle,
    /// User tapped a number, POST /2fa/verify in flight
    Verifying,
    /// Mobile approved (or verify succeeded)
    Success,
    /// Wrong code chosen via verify fallback
    Error,
    /// JWT expired (25 min TTL)
    Expired,
}

// ---------------------------------------------------------------------------
// API response types
// ---------------------------------------------------------------------------

/// Response from POST /2fa
#[derive(Deserialize, Clone)]
pub struct TwoFaInitResponse {
    /// Short-lived JWT for subsequent 2FA endpoints
    pub token: String,
    /// The 2-digit code (10–99) returned as an integer by the backend
    pub code: u32,
}

/// Response from GET /2fa/status
#[derive(Deserialize, Clone)]
pub struct TwoFaStatusResponse {
    pub approved: bool,
    pub redirect: Option<String>,
    pub expired: Option<bool>,
}



pub fn generate_choices(correct: u32) -> [u32; 3] {
    let mut a = if correct > 10 { correct - 7 } else { correct + 13 };
    let mut b = if correct < 90 { correct + 11 } else { correct - 17 };

    a = a.clamp(10, 99);
    b = b.clamp(10, 99);

    if a == correct { a = (a + 1).min(99); }
    if b == correct || b == a { b = (b + 2).min(99); }
    if b == correct { b = (b + 3).min(99); }

    match correct % 3 {
        0 => [correct, a, b],
        1 => [a, correct, b],
        _ => [a, b, correct],
    }
}


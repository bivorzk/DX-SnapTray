// HTTP client calls that mirror the /2fa backend routes.
// Each function corresponds to exactly one endpoint.

use super::model::{API_BASE, TwoFaInitResponse, TwoFaStatusResponse};
use serde::Deserialize;
use serde_json;

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

// ---------------------------------------------------------------------------
// POST /2fa
// Initiates a 2FA session after successful credential check.
// Returns the JWT token AND the 2-digit code the desktop should use to build
// the 3-choice grid. The same code is stored server-side (Redis/memory) so
// the mobile app can fetch it separately.
// ---------------------------------------------------------------------------
pub async fn api_start_2fa(email: &str) -> Result<TwoFaInitResponse, String> {
    let resp = client()
        .post(format!("{}/2fa", API_BASE))
        .form(&[("email", email)])
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    match resp.status().as_u16() {
        200..=299 => {}
        400 => return Err("Email is required.".into()),
        403 => return Err("2FA is not enabled for this account.".into()),
        404 => return Err("Account not found.".into()),
        429 => {
            let retry_secs = resp
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            return Err(match retry_secs {
                Some(s) => format!("Too many requests — try again in {s} seconds."),
                None    => "Too many requests — please wait a moment.".into(),
            });
        }
        _ => return Err(format!("Server error ({})", resp.status().as_u16())),
    }

    let raw = resp.text().await.map_err(|e: reqwest::Error| e.to_string())?;
    serde_json::from_str::<TwoFaInitResponse>(&raw)
        .map_err(|e| format!("Parse error: {e} — body: {raw}"))
}

// ---------------------------------------------------------------------------
// GET /2fa/status  (desktop polls every 3 s)
// Returns Ok(Some(redirect_path)) when the mobile device has approved,
// Ok(None) while still pending, Err("expired") when the JWT is gone.
// ---------------------------------------------------------------------------
pub async fn api_poll_status(token: &str) -> Result<Option<String>, String> {
    let resp = client()
        .get(format!("{}/2fa/status", API_BASE))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    if resp.status() == 401 {
        return Err("expired".into());
    }
    if resp.status() == 429 {
        return Ok(None); // rate-limited — treat as "still pending", retry next tick
    }
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_else(|_| "Server error".into()));
    }

    let body: TwoFaStatusResponse = resp.json().await.map_err(|e: reqwest::Error| e.to_string())?;

    if body.expired == Some(true) {
        return Err("expired".into());
    }
    if body.approved {
        Ok(Some(body.redirect.unwrap_or_else(|| "/dashboard".into())))
    } else {
        Ok(None) // still waiting
    }
}

// ---------------------------------------------------------------------------
// POST /2fa/approve  (mobile device — approves the pending login)
// ---------------------------------------------------------------------------
pub async fn api_approve(token: &str) -> Result<(), String> {
    let resp = client()
        .post(format!("{}/2fa/approve", API_BASE))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_else(|_| "Approval failed".into()));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// POST /2fa/verify  (legacy fallback — user submits the code manually)
// The backend compares `code` against the stored Redis/memory value.
// Returns true on match, false on mismatch.
// ---------------------------------------------------------------------------
pub async fn api_verify(token: &str, code: u32) -> Result<bool, String> {
    let resp = client()
        .post(format!("{}/2fa/verify", API_BASE))
        .form(&[("token", token), ("code", &code.to_string())])
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    if resp.status() == 401 {
        return Ok(false); // wrong code or expired
    }
    Ok(resp.status().is_success())
}

// ---------------------------------------------------------------------------
// GET /2fa/code  (mobile device — fetches the code it should display)
// Not called by the desktop flow (code comes from POST /2fa directly).
// ---------------------------------------------------------------------------
pub async fn api_get_code(token: &str) -> Result<u32, String> {
    #[derive(Deserialize)]
    struct CodeResp {
        code: u32,
    }

    let resp = client()
        .get(format!("{}/2fa/code", API_BASE))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e: reqwest::Error| e.to_string())?;

    if resp.status() == 404 {
        return Err("No pending 2FA code.".into());
    }
    if !resp.status().is_success() {
        return Err(resp.text().await.unwrap_or_else(|_| "Server error".into()));
    }

    resp.json::<CodeResp>()
        .await
        .map(|r| r.code)
        .map_err(|e: reqwest::Error| e.to_string())
}

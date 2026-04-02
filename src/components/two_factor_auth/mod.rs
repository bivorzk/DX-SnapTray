// two_factor_auth - SnapTray "Match the Number" 2FA screen.
//
// The desktop app acts as the APPROVER (like Google's phone prompt):
//   1. POST /2fa  -> { token, code }  (idempotent - returns the same code
//      that the web already stored in Redis)
//   2. Generate 3 choices; show them to the user.
//   3. User picks the number displayed on the web's verify page.
//   4. If correct -> POST /2fa/approve -> sets approved flag in Redis.
//   5. The web's polling of GET /2fa/status detects approval and redirects.
//
// The desktop does NOT poll /2fa/status - it is not the challenger.

use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod api;
pub mod model;
pub mod view;

#[cfg(not(target_arch = "wasm32"))]
use api::{api_approve, api_start_2fa};
use model::{generate_choices, Status};

// Stubs for wasm32 so the component compiles on all platforms.
#[cfg(target_arch = "wasm32")]
async fn api_start_2fa(_email: &str) -> Result<model::TwoFaInitResponse, String> {
    Err("2FA not supported on web client".into())
}
#[cfg(target_arch = "wasm32")]
async fn api_approve(_token: &str) -> Result<(), String> {
    Err("2FA not supported on web client".into())
}
use view::number_grid;


#[component]
pub fn TwoFactorAuth(email: String) -> Element {
    let mut status = use_signal(|| Status::Ready);
    let mut error_msg = use_signal(|| String::new());
    let mut jwt_token = use_signal(|| String::new());
    let mut choices = use_signal(|| [0_u32; 3]);
    let mut correct_code = use_signal(|| 0_u32);

    let email_for_start = email.clone();
    let start_session = move |_| {
        let em = email_for_start.clone();
        spawn(async move {
            status.set(Status::Loading);
            error_msg.set(String::new());
            match api_start_2fa(&em).await {
                Ok(init) => {
                    jwt_token.set(init.token);
                    correct_code.set(init.code);
                    choices.set(generate_choices(init.code));
                    status.set(Status::Idle);
                }
                Err(e) => {
                    error_msg.set(e);
                    status.set(Status::Error);
                }
            }
        });
    };

    let on_pick = move |picked: u32| {
        if *status.peek() != Status::Idle {
            return;
        }
        let expected = *correct_code.peek();
        if picked != expected {
            error_msg.set("Wrong number - try again.".into());
            return; // stay Idle so user can pick again
        }
        let token = jwt_token.read().clone();
        spawn(async move {
            status.set(Status::Verifying);
            match api_approve(&token).await {
                Ok(()) => {
                    status.set(Status::Success);
                }
                Err(e) => {
                    error_msg.set(format!("Approval failed: {e}"));
                    status.set(Status::Error);
                }
            }
        });
    };

    let on_resend = start_session.clone();

    let cur = status();
    let is_ready = cur == Status::Ready;
    let is_loading = cur == Status::Loading;
    let is_success = cur == Status::Success;
    let is_expired = cur == Status::Expired;
    let is_error = cur == Status::Error;
    let is_disabled = !matches!(cur, Status::Idle);

    let err = error_msg.read().clone();
    let (status_msg, status_color): (&str, &str) = match &cur {
        Status::Ready     => ("",                                              "#94a3b8"),
        Status::Loading   => ("Connecting\u{2026}",                            "#FFC857"),
        Status::Success   => ("Approved! The web page will redirect shortly.", "#16a34a"),
        Status::Error     => (err.as_str(),                                    "#f97316"),
        Status::Expired   => (err.as_str(),                                    "#f97316"),
        Status::Verifying => ("Approving\u{2026}",                             "#FFC857"),
        Status::Idle      => ("Pick the number shown on the website",          "#94a3b8"),
    };


    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        document::Meta { name: "theme-color", content: "#0f172a" }

        document::Style {
            "*, *::before, *::after {{ box-sizing: border-box; }}
            html, body {{
                margin: 0; padding: 0; width: 100%; height: 100%;
                background: #0f172a !important; color: #f8fafc;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                -webkit-font-smoothing: antialiased;
                overscroll-behavior: none;
            }}
            button:focus {{ outline: none; box-shadow: 0 0 0 3px rgba(255,107,53,0.40) !important; }}"
        }

        div { style: "
                min-height: 100vh; min-height: 100dvh;
                background: #0f172a;
                display: flex; flex-direction: column;
                align-items: center; justify-content: center;
                padding: env(safe-area-inset-top,32px) 20px env(safe-area-inset-bottom,32px);
            ",

            // Logo
            div { style: "margin-bottom: 28px; filter: drop-shadow(0 8px 32px rgba(255,107,53,0.45));",
                svg { view_box: "0 0 500 140", width: "220", height: "62",
                    rect {
                        x: "25",
                        y: "55",
                        width: "90",
                        height: "50",
                        rx: "6",
                        fill: "#FF6B35",
                    }
                    rect {
                        x: "30",
                        y: "60",
                        width: "35",
                        height: "40",
                        rx: "3",
                        fill: "#FFE5DC",
                    }
                    rect {
                        x: "70",
                        y: "60",
                        width: "20",
                        height: "18",
                        rx: "3",
                        fill: "#FFE5DC",
                    }
                    rect {
                        x: "95",
                        y: "60",
                        width: "20",
                        height: "18",
                        rx: "3",
                        fill: "#FFE5DC",
                    }
                    rect {
                        x: "70",
                        y: "82",
                        width: "45",
                        height: "18",
                        rx: "3",
                        fill: "#FFE5DC",
                    }
                    path {
                        d: "M80 25 L65 52 L75 52 L60 80 L85 50 L75 50 L90 25 Z",
                        fill: "#FFC857",
                        stroke: "#FF6B35",
                        stroke_width: "2",
                        stroke_linejoin: "round",
                    }
                    text {
                        x: "150",
                        y: "85",
                        font_family: "system-ui, sans-serif",
                        font_size: "32",
                        font_weight: "bold",
                        fill: "#FF6B35",
                        letter_spacing: "-1",
                        "SnapTray"
                    }
                }
            }

            // Card
            div { style: "
                    background: #111827; border-radius: 24px;
                    padding: 32px 24px 28px; width: 100%; max-width: 380px;
                    box-shadow: 0 32px 80px rgba(0,0,0,0.65), 0 0 0 1px rgba(255,255,255,0.04);
                    text-align: center;
                ",

                h1 { style: "color:#f8fafc; font-size:1.4rem; font-weight:700; margin:0 0 8px;",
                    "Match the number"
                }
                p { style: "color:#94a3b8; font-size:0.9rem; margin:0 0 4px; line-height:1.5;",
                    "Pick the number shown on the website"
                }
                p { style: "color:#64748b; font-size:0.78rem; margin:0 0 20px;",
                    "This confirms it\u{2019}s really you signing in"
                }

                // "Start" button - shown before any API call
                if is_ready {
                    button {
                        r#type: "button",
                        onclick: start_session,
                        style: "
                            width:100%; height:52px; font-size:1rem; font-weight:700;
                            border:none; border-radius:16px; margin-bottom:8px;
                            background: linear-gradient(135deg, #FF6B35, #e85d2c);
                            color:#fff; cursor:pointer;
                            box-shadow: 0 4px 16px rgba(255,107,53,0.38);
                        ",
                        "Load verification"
                    }
                }

                // 3 number buttons - shown once session is active
                if !is_ready && !is_loading {
                    {number_grid(choices(), on_pick, is_disabled)}
                }

                // Status / feedback line
                div {
                    "aria-live": "polite",
                    role: "status",
                    style: "min-height: 28px; margin: 4px 0 16px;",
                    if !status_msg.is_empty() {
                        p { style: "margin:0; font-size:0.875rem; font-weight:600; color:{status_color};",
                            "{status_msg}"
                        }
                    }
                }

                // Success checkmark
                if is_success {
                    div { style: "margin: 8px 0 16px;",
                        p { style: "font-size: 2.5rem; margin: 0;", "\u{2705}" }
                    }
                }

                // "Try again" - API call failed (rate limit, network)
                if is_error {
                    button {
                        r#type: "button",
                        onclick: on_resend.clone(),
                        style: "
                            width:100%; height:48px; font-size:0.9375rem; font-weight:600;
                            border:none; border-radius:14px; margin-top:8px;
                            background: linear-gradient(135deg, #FF6B35, #e85d2c);
                            color:#fff; cursor:pointer;
                            box-shadow: 0 4px 14px rgba(255,107,53,0.32);
                        ",
                        "Try again"
                    }
                }

                // "Resend Code" - JWT expired
                if is_expired {
                    button {
                        r#type: "button",
                        onclick: on_resend,
                        style: "
                            width:100%; height:48px; font-size:0.9375rem; font-weight:600;
                            border:none; border-radius:14px; margin-top:8px;
                            background: linear-gradient(135deg, #FFC857, #f0b840);
                            color:#111827; cursor:pointer;
                            box-shadow: 0 4px 14px rgba(255,200,87,0.32);
                        ",
                        "Resend Code"
                    }
                }
            }
        }
    }
}
//! two_factor_auth.rs — SnapTray Native-Mobile 2FA Screen
//!
//! Site-specific branding, NOT a generic Google Authenticator clone.
//! This is a push-based number-matching MFA experience for the SnapTray
//! cafeteria ordering platform, targeting Dioxus Mobile (Android / iOS
//! embedded WebView) but fully portable to web and desktop.
//!
//! Palette (from tailwind.config.js / public/home_page):
//!   primary   #FF6B35   orange
//!   secondary #FFC857   amber
//!   accent    #FFE5DC   peach
//!   bg        #0f172a   dark navy
//!   card      #111827
//!   text      #f8fafc
//!   success   #16a34a
//!   error     #f97316
//!
//! Timer runtime:
//!   wasm32  -> gloo_timers::future::sleep  (setTimeout via wasm_bindgen)
//!   native  -> tokio::time::sleep          (OS timers; gloo_timers SIGABRT otherwise)

use dioxus::document::eval;
use dioxus::prelude::*;
use std::time::Duration;

pub mod model;
pub mod view;

use model::{Status, AUTH_CODE, code_matches, is_active};
use view::otp_grid;

// =============================================================================
// TwoFactorAuth -- root component
// =============================================================================

// Platform-split async sleep helper
// =============================================================================

/// Non-blocking 1-second sleep.
/// `gloo_timers` is wasm_bindgen FFI -- calling it in a native Android process
/// triggers SIGABRT.  `tokio` is used for all non-wasm32 targets instead.
async fn sleep_1s() {
    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(std::time::Duration::from_millis(1000)).await;
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(Duration::from_millis(1000)).await;
}

/// Focus (and select) an `<input>` by its HTML `id` via JS eval.
fn focus_input(id: &str) {
    let js = format!(
        "(function(){{var e=document.getElementById('{}');if(e){{e.focus();e.select();}}}})()",
        id
    );
    let _ = eval(&js);
}

// =============================================================================
// TwoFactorAuth -- root component
// =============================================================================

/// SnapTray push-based number-matching MFA screen.
///
/// Mount as app root:
/// ```rust
/// fn main() { dioxus::launch(TwoFactorAuth); }
/// ```
#[component]
pub fn TwoFactorAuth() -> Element {
    // -- Reactive state: one signal per digit (3-digit configurable) -------
    let mut digit1       = use_signal(|| String::new());
    let mut digit2       = use_signal(|| String::new());
    let mut digit3       = use_signal(|| String::new());
    let mut seconds_left = use_signal(|| 30_i32);
    let mut status       = use_signal(|| Status::Idle);
    let mut loading      = use_signal(|| false);

    // -- Countdown timer -------------------------------------------------------
    // Ticks only while Idle or Error; Pending/Success/Expired pause it.
    use_future(move || async move {
        loop {
            sleep_1s().await;
            let s = status();
            if s == Status::Idle || s == Status::Error {
                let secs = seconds_left();
                if secs > 0 {
                    *seconds_left.write() -= 1;
                } else {
                    *status.write() = Status::Expired;
                }
            }
        }
    });

    // -- Derived / memoised values ---------------------------------------------
    let all_filled = use_memo(move || {
        !digit1().is_empty()
            && !digit2().is_empty()
            && !digit3().is_empty()
    });

    let secs           = seconds_left();
    let current_status = status();
    let is_active      = current_status == Status::Idle || current_status == Status::Error;
    let is_expired     = current_status == Status::Expired;
    let is_success     = current_status == Status::Success;

    // -- SVG ring (r=20, circumference ~125.664) --------------------------------
    const CIRCUM: f32 = 125.664_f32;
    let ring_frac   = (secs as f32 / 30.0_f32).clamp(0.0, 1.0);
    let ring_offset = CIRCUM * (1.0 - ring_frac);
    // green -> amber -> orange as time runs out (brand palette)
    let ring_color  = if ring_frac > 0.5 { "#16a34a" }
                      else if ring_frac > 0.2 { "#FFC857" }
                      else { "#f97316" };

    // -- ARIA live status message -----------------------------------------------
    let (status_msg, status_color): (&str, &str) = match &current_status {
        Status::Success => ("Code verified — access granted.",            "#16a34a"),
        Status::Error   => ("Incorrect code. Please try again.",            "#f97316"),
        Status::Expired => ("Code expired. Request a new one.",             "#f97316"),
        _               => ("",                                              "#f8fafc"),
    };

    // -- Button availability ---------------------------------------------------
    let verify_enabled = all_filled() && is_active && !loading();
    let resend_enabled = is_expired;

    // -- on_verify: 1-second simulated server call, fixed code "123" --------
    let on_verify = move |_| {
        if !verify_enabled { return; }
        let code = format!(
            "{}{}{}",
            digit1(), digit2(), digit3()
        );
        spawn(async move {
            *loading.write() = true;
            *status.write()  = Status::Pending;
            sleep_1s().await;
            *status.write()  = if code == "123" { Status::Success } else { Status::Error };
            *loading.write() = false;
        });
    };

    // -- on_resend: reset everything to initial state --------------------------
    let on_resend = move |_| {
        *digit1.write()       = String::new();
        *digit2.write()       = String::new();
        *digit3.write()       = String::new();
        *seconds_left.write() = 30;
        *status.write()       = Status::Idle;
        *loading.write()      = false;
        focus_input("otp-1");
    };

    // -- OTP input handlers ----------------------------------------------------
    // Each field: oninput strips non-digits and auto-advances;
    //             onkeydown handles Backspace (clear / retreat).

    // Field 1
    let on_input_1 = move |evt: FormEvent| {
        let ch = evt.value().chars().rev().find(|c| c.is_ascii_digit());
        match ch {
            Some(d) => { *digit1.write() = d.to_string(); focus_input("otp-2"); }
            None    =>   *digit1.write() = String::new(),
        }
    };
    let on_key_1 = move |evt: KeyboardEvent| {
        if evt.key() == Key::Backspace { *digit1.write() = String::new(); }
    };

    // Field 2
    let on_input_2 = move |evt: FormEvent| {
        let ch = evt.value().chars().rev().find(|c| c.is_ascii_digit());
        match ch {
            Some(d) => { *digit2.write() = d.to_string(); focus_input("otp-3"); }
            None    =>   *digit2.write() = String::new(),
        }
    };
    let on_key_2 = move |evt: KeyboardEvent| {
        if evt.key() == Key::Backspace {
            if digit2().is_empty() { focus_input("otp-1"); }
            else { *digit2.write() = String::new(); }
        }
    };

    // Field 3
    let on_input_3 = move |evt: FormEvent| {
        let ch = evt.value().chars().rev().find(|c| c.is_ascii_digit());
        match ch {
            Some(d) => { *digit3.write() = d.to_string(); }
            None    =>   *digit3.write() = String::new(),
        }
    };
    let on_key_3 = move |evt: KeyboardEvent| {
        if evt.key() == Key::Backspace {
            if digit3().is_empty() { focus_input("otp-2"); }
            else { *digit3.write() = String::new(); }
        }
    };

    // -- Paste handler: clipboard -> distribute across 3 fields ---------------
    let on_paste = move |_: Event<ClipboardData>| {
        spawn(async move {
            let mut ev = eval(r#"
                (async () => {
                    try {
                        const text = await navigator.clipboard.readText();
                        dioxus.send(text.replace(/\D/g, '').slice(0, 3));
                    } catch (_) { dioxus.send(''); }
                })()
            "#);
            if let Ok(pasted) = ev.recv::<String>().await {
                let digits: Vec<char> = pasted
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .take(3)
                    .collect();
                if let Some(d) = digits.first().copied()  { *digit1.write() = d.to_string(); }
                if let Some(d) = digits.get(1).copied()   { *digit2.write() = d.to_string(); }
                if let Some(d) = digits.get(2).copied()   { *digit3.write() = d.to_string(); }
                let next = match digits.len() {
                    0 => "otp-1", 1 => "otp-2", _ => "otp-3",
                };
                focus_input(next);
            }
        });
    };

    // =========================================================================
    // RSX -- native-mobile render tree
    // =========================================================================

    rsx! {
        // -- Mobile meta tags --------------------------------------------------
        // theme-color sets the Android status bar / iOS nav bar colour so there
        // is no white flash before the WebView renders our dark background.
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        document::Meta { name: "theme-color", content: "#0f172a" }
        document::Meta { name: "color-scheme", content: "dark" }

        // -- Global CSS reset --------------------------------------------------
        // Focus ring uses brand orange instead of blue.
        document::Style {
            "*, *::before, *::after {{ box-sizing: border-box; }}
            html, body {{
                margin: 0; padding: 0;
                width: 100%; height: 100%;
                background: #0f172a !important;
                color: #f8fafc;
                font-family: -apple-system, 'SF Pro Display', BlinkMacSystemFont,
                             'Segoe UI', Roboto, sans-serif;
                -webkit-font-smoothing: antialiased;
                overscroll-behavior: none;
            }}
            input {{
                -webkit-tap-highlight-color: transparent;
                -webkit-user-select: text;
                font-family: inherit;
            }}
            input[type=tel]::-webkit-inner-spin-button,
            input[type=tel]::-webkit-outer-spin-button {{ display: none; }}
            input:focus {{
                outline: none;
                border-color: #FF6B35 !important;
                box-shadow: 0 0 0 3px rgba(255,107,53,0.30) !important;
            }}"
        }

        // -- Full-screen dark-navy scaffold ------------------------------------
        // 100dvh = dynamic viewport (handles iOS bottom bar / Android nav bar).
        div { style: "
                min-height: 100vh;
                min-height: 100dvh;
                background: #0f172a;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: env(safe-area-inset-top,32px) 20px env(safe-area-inset-bottom,32px);
            ",

            // -- SnapTray site logo (exact replica of public/home_page JSX) ----
            // SVG viewBox 0 0 500 140 scaled down via width/height.
            // Glow shadow matches the hero ring aesthetic from the screenshot.
            div {
                "aria-label": "SnapTray logo",
                role: "img",
                style: "
                    margin-bottom: 28px;
                    filter: drop-shadow(0 8px 32px rgba(255,107,53,0.45));
                ",
                svg { view_box: "0 0 500 140", width: "260", height: "73",
                    // Tray body (primary orange)
                    rect {
                        x: "25",
                        y: "55",
                        width: "90",
                        height: "50",
                        rx: "6",
                        fill: "#FF6B35",
                    }
                    // Tray compartments (accent peach)
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
                    // Lightning bolt (secondary amber)
                    path {
                        d: "M80 25 L65 52 L75 52 L60 80 L85 50 L75 50 L90 25 Z",
                        fill: "#FFC857",
                        stroke: "#FF6B35",
                        stroke_width: "2",
                        stroke_linejoin: "round",
                    }
                    // Wordmark
                    text {
                        x: "150",
                        y: "85",
                        font_family: "system-ui, -apple-system, sans-serif",
                        font_size: "32",
                        font_weight: "bold",
                        fill: "#FF6B35",
                        letter_spacing: "-1",
                        "SnapTray"
                    }
                    // Tagline
                    text {
                        x: "150",
                        y: "105",
                        font_family: "system-ui, -apple-system, sans-serif",
                        font_size: "20",
                        fill: "#6C757D",
                        letter_spacing: "2",
                        "CAFETERIA ORDERING"
                    }
                }
            }

            // -- Heading -------------------------------------------------------
            h1 { style: "
                    color: #f8fafc;
                    font-size: 1.45rem;
                    font-weight: 700;
                    text-align: center;
                    margin: 0 0 10px;
                    letter-spacing: -0.02em;
                    line-height: 1.25;
                ",
                "Two-Factor Authentication"
            }

            // -- Single subtitle (push-specific) --------------------------------
            p {
                id: "otp-desc",
                style: "
                    color: #94a3b8;
                    font-size: 0.9rem;
                    text-align: center;
                    margin: 0 0 28px;
                    line-height: 1.55;
                    max-width: 290px;
                ",
                "Choose the number on your phone that matches the one shown on your sign-in screen."
            }

            // -- Dark card -----------------------------------------------------
            div {
                style: "
                    background: #111827;
                    border-radius: 24px;
                    padding: 24px 18px 22px;
                    width: 100%;
                    max-width: 390px;
                    box-shadow: 0 32px 80px rgba(0,0,0,0.65),
                                0 0 0 1px rgba(255,255,255,0.04);
                ",

                // -- Countdown ring (hidden after success) ---------------------
                if !is_success {
                    div {
                        "aria-hidden": "true",
                        style: "
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            gap: 5px;
                            margin-bottom: 22px;
                        ",
                        // SVG ring: track #334155, arc colour-shifts green->amber->orange.
                        // rotate(-90 25 25) positions arc start at 12 o'clock.
                        svg {
                            view_box: "0 0 50 50",
                            width: "60",
                            height: "60",
                            circle {
                                cx: "25",
                                cy: "25",
                                r: "20",
                                fill: "none",
                                stroke: "#334155",
                                stroke_width: "4",
                            }
                            circle {
                                cx: "25",
                                cy: "25",
                                r: "20",
                                fill: "none",
                                stroke: "{ring_color}",
                                stroke_width: "4",
                                stroke_dasharray: "{CIRCUM}",
                                stroke_dashoffset: "{ring_offset:.2}",
                                stroke_linecap: "round",
                                transform: "rotate(-90 25 25)",
                            }
                            text {
                                x: "25",
                                y: "30",
                                text_anchor: "middle",
                                font_size: "14",
                                font_weight: "700",
                                fill: "{ring_color}",
                                "{secs}"
                            }
                        }
                        span { style: "font-size:0.72rem; color:#64748b; font-weight:500; letter-spacing:0.03em;",
                            if is_expired {
                                "Code expired"
                            } else {
                                "Expires in {secs}s"
                            }
                        }
                    }
                }

                // -- 3-digit OTP grid: one row of 3 large boxes --------------------
                // Modular component style via `view::otp_grid`.
                {
                    otp_grid(
                        digit1(),
                        digit2(),
                        digit3(),
                        on_input_1.clone(),
                        on_input_2.clone(),
                        on_input_3.clone(),
                        on_key_1.clone(),
                        on_key_2.clone(),
                        on_key_3.clone(),
                        on_paste.clone(),
                    )
                }

                // -- ARIA live feedback region ----------------------------------
                // aria-live="polite" queues an announcement without cutting off speech.
                div {
                    "aria-live": "polite",
                    role: "status",
                    style: "min-height:28px; margin-bottom:16px; text-align:center;",
                    if !status_msg.is_empty() {
                        p { style: "margin:0; font-size:0.875rem; font-weight:600; color:{status_color};",
                            "{status_msg}"
                        }
                    }
                }

                // -- Action buttons --------------------------------------------
                // Four palette roles (from tailwind.config.js / prompt spec):
                //   primary   #FF6B35  → Verify Code (active)
                //   secondary #FFC857  → Resend Code (expired)
                //   success   #16a34a  → Continue    (verified)
                //   error     #f97316  → Try Again   (wrong code)
                // All touch targets ≥ 44px iOS / 48dp Android minimum.
                div { style: "display:flex; flex-direction:column; gap:10px;",

                    // Primary: Verify — orange gradient; muted when disabled
                    if !is_success {
                        button {
                            r#type: "button",
                            disabled: !verify_enabled,
                            "aria-label": "Verify the entered 3-digit one-time code",
                            style: if verify_enabled { "
                                    width:100%; height:56px;
                                    font-size:1rem; font-weight:700;
                                    letter-spacing:0.01em;
                                    border:none; border-radius:16px;
                                    background: linear-gradient(135deg, #FF6B35, #e85d2a);
                                    color:#fff; cursor:pointer;
                                    -webkit-tap-highlight-color:transparent;
                                    touch-action:manipulation;
                                    box-shadow: 0 4px 16px rgba(255,107,53,0.38);
                                " } else if current_status == Status::Error { "
                                    width:100%; height:56px;
                                    font-size:1rem; font-weight:700;
                                    border:none; border-radius:16px;
                                    background: linear-gradient(135deg, #f97316, #ea6b10);
                                    color:#fff; cursor:pointer;
                                    -webkit-tap-highlight-color:transparent;
                                    touch-action:manipulation;
                                    box-shadow: 0 4px 16px rgba(249,115,22,0.38);
                                " } else { "
                                    width:100%; height:56px;
                                    font-size:1rem; font-weight:700;
                                    border:none; border-radius:16px;
                                    background:#2d1f18; color:#5a3d2b;
                                    cursor:not-allowed; opacity:0.6;
                                " },
                            onclick: on_verify,
                            if loading() {
                                "Verifying…"
                            } else if current_status == Status::Error {
                                "Try Again"
                            } else {
                                "Verify Code"
                            }
                        }
                    }

                    // Success: Continue — green (#16a34a) full button
                    if is_success {
                        button {
                            r#type: "button",
                            "aria-label": "Continue after successful verification",
                            style: "
                                width:100%; height:56px;
                                font-size:1rem; font-weight:700;
                                letter-spacing:0.01em;
                                border:none; border-radius:16px;
                                background: linear-gradient(135deg, #16a34a, #15803d);
                                color:#fff; cursor:pointer;
                                -webkit-tap-highlight-color:transparent;
                                touch-action:manipulation;
                                box-shadow: 0 4px 16px rgba(22,163,74,0.38);
                            ",
                            "Continue →"
                        }
                    }

                    // Secondary: Resend — amber (#FFC857) shown only when expired
                    if resend_enabled {
                        button {
                            r#type: "button",
                            "aria-label": "Request a new one-time code",
                            style: "
                                width:100%; height:48px;
                                font-size:0.9375rem; font-weight:600;
                                border:none; border-radius:14px;
                                background: linear-gradient(135deg, #FFC857, #f0b840);
                                color:#111827;
                                cursor:pointer;
                                -webkit-tap-highlight-color:transparent;
                                touch-action:manipulation;
                                box-shadow: 0 4px 14px rgba(255,200,87,0.32);
                            ",
                            onclick: on_resend,
                            "Resend Code"
                        }
                    }
                } // end buttons
            } // end card

            // -- Fallback note: push MFA device registration ---------------
            // Site-specific guidance — this is NOT a generic Google 2FA flow.
            // Push notifications are sent by the SnapTray backend to a
            // pre-registered device; no third-party authenticator app is needed.
            p { style: "
                    color:#475569;
                    font-size:0.72rem;
                    text-align:center;
                    margin-top:18px;
                    line-height:1.65;
                    max-width:300px;
                ",
                "Don't have a registered device? Visit "
                a {
                    href: "/account/devices",
                    style: "color:#FFC857; text-decoration:none;",
                    "Account › Devices"
                }
                " to enrol your phone for SnapTray push notifications."
            }
        } // end scaffold
    }
}

use dioxus::prelude::*;

// MongoDB backend context (user sessions), but no backend code.
// This is a native mobile login page for Dioxus Mobile (iOS/Android).

#[component]
pub fn Login(cx: Scope) -> Element {
    let username = use_signal(cx, || String::new());
    let password = use_signal(cx, || String::new());
    let loading = use_signal(cx, || false);
    let message = use_signal(cx, || None::<String>);
    let authenticated = use_signal(cx, || false);

    let on_login = move |_| {
        cx.spawn(async move {
            *loading.write() = true;
            *message.write() = None;
            // Simulate login delay and success (any username/password)
            gloo_timers::future::TimeoutFuture::new(1000).await;
            *loading.write() = false;
            *message.write() = Some("Login successful! Redirecting to 2FA...".to_string());
            // Transition to 2FA
            *authenticated.write() = true;
        });
    };

    let on_forgot_password = move |_| {
        // TODO: Implement forgot password
    };

    rsx! {
        // Include styles
        style {
            "
            body { margin: 0; min-height: 100vh; background: linear-gradient(180deg, #0f172a 0%, #020617 100%); color: #f8fafc; font-family: 'Inter', 'Segoe UI', Roboto, sans-serif; }
            .login-page { min-height: 100vh; display: grid; place-items: center; padding: 20px; background: #0f172a; }
            .login-card { width: min(420px, 100%); background: #111827; border: 1px solid rgba(255,255,255,0.12); border-radius: 18px; box-shadow: 0 20px 40px rgba(0,0,0,0.45); padding: 30px; }
            .login-card h1 { margin-top: 16px; margin-bottom: 12px; text-align: center; color: #f8fafc; font-size: 2.3rem; font-weight: 800; }
            .login-card input { width: 100%; display: block; margin-bottom: 14px; padding: 14px 16px; border: 1px solid rgba(255,255,255,0.18); background: rgba(15,23,42,0.82); color: #f8fafc; border-radius: 12px; outline: none; font-size: 1rem; }
            .login-card button { width: 100%; border: 0; border-radius: 12px; padding: 13px 16px; margin-top: 10px; color: #fff; font-weight: 700; background: #FF6B35; box-shadow: 0 8px 20px rgba(255,107,53,0.35); cursor: pointer; }
            .login-card button:disabled { opacity: 0.58; cursor: not-allowed; }
            .login-card .bottom-note { text-align: center; margin-top: 14px; color: #FFE5DC; font-size: 0.94rem; }
            .login-card .bottom-note a { color: #FFC30D; font-weight: 700; text-decoration: none; }
        "
        }
        div { class: "login-page",
            div { class: "login-card",
                // Logo SVG
                div { style: "text-align: center; margin-bottom: 20px;",
                    svg {
                        view_box: "0 0 500 140",
                        width: "180",
                        height: "50",
                        xmlns: "http://www.w3.org/2000/svg",
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
                            font_family: "system-ui, -apple-system, sans-serif",
                            font_size: "32",
                            font_weight: "bold",
                            fill: "#FF6B35",
                            letter_spacing: "-1",
                            "SnapTray"
                        }
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
                h1 { "Login to SnapTray" }
                form { onsubmit: on_login,
                    input {
                        r#type: "text",
                        placeholder: "Username",
                        value: "{username}",
                        oninput: move |evt| *username.write() = evt.value().clone(),
                    }
                    input {
                        r#type: "password",
                        placeholder: "Password",
                        value: "{password}",
                        oninput: move |evt| *password.write() = evt.value().clone(),
                    }
                    button { r#type: "submit", disabled: "{loading}",
                        if loading {
                            "Logging in..."
                        } else {
                            "Login"
                        }
                    }
                }
                if let Some(msg) = message.read().as_ref() {
                    p { style: "text-align: center; margin-top: 10px; color: #16a34a;",
                        "{msg}"
                    }
                }
                div { style: "text-align: center; margin-top: 10px;",
                    a {
                        href: "#",
                        onclick: on_forgot_password,
                        style: "color: #FFC857; text-decoration: none;",
                        "Forgot Password?"
                    }
                }
                p { class: "bottom-note",
                    "If you don't have an account register at "
                    a { href: "https://snaptray.onrender.com/register",
                        "Snaptray.onrender.com/register"
                    }
                }
            }
        }
    }
}
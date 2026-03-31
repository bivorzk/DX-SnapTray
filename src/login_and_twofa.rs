use crate::components::{LoginScreen, TwoFactorAuth};
use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn LoginAndTwoFa() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut login_message = use_signal(|| None::<String>);
    // Stores the authenticated user's email so we can pass it to TwoFactorAuth
    let mut user_email: Signal<String> = use_signal(|| String::new());
    let mut authenticated = use_signal(|| false);

    let on_login = move |_| {
        spawn(async move {
            *loading.write() = true;
            *login_message.write() = None;

            let uname = username.read().clone();
            let pass = password.read().clone();

            #[cfg(not(target_arch = "wasm32"))]
            match crate::db::authenticate_user(&uname, &pass).await {
                Ok(Some(user)) => {
                    *user_email.write() = user.email.clone();
                    *login_message.write() =
                        Some("Login successful! Redirecting to 2FA...".to_string());
                    *authenticated.write() = true;
                }
                Ok(None) => {
                    *login_message.write() =
                        Some("Invalid username or password.".to_string());
                }
                Err(e) => {
                    *login_message.write() = Some(format!("Database error: {e}"));
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                // Web: replace with a server-function call when available
                TimeoutFuture::new(1000).await;
                *login_message.write() =
                    Some("Login successful! Redirecting to 2FA...".to_string());
                *authenticated.write() = true;
            }

            *loading.write() = false;
        });
    };

    let on_forgot_password = move |_| {
        // TODO: Implement forgot password
    };

    if *authenticated.read() {
        rsx! {
            TwoFactorAuth { email: user_email.read().clone() }
        }
    } else {
        rsx! {
            LoginScreen {
                username,
                password,
                loading,
                login_message,
                on_username: move |evt: FormEvent| *username.write() = evt.value().clone(),
                on_password: move |evt: FormEvent| *password.write() = evt.value().clone(),
                on_submit: on_login,
                on_forgot: on_forgot_password,
            }
        }
    }
}
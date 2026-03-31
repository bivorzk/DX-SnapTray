use crate::components::{LoginScreen, TwoFactorAuth};
use dioxus::prelude::*;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

#[component]
pub fn LoginAndTwoFa() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut login_message = use_signal(|| None::<String>);
    let mut authenticated = use_signal(|| false);

    let on_login = move |_| {
        spawn(async move {
            *loading.write() = true;
            *login_message.write() = None;
            // Simulate login delay and success (any username/password)
            #[cfg(target_arch = "wasm32")]
            TimeoutFuture::new(1000).await;
            #[cfg(not(target_arch = "wasm32"))]
            sleep(Duration::from_millis(1000)).await;
            *loading.write() = false;
            *login_message.write() = Some("Login successful! Redirecting to 2FA...".to_string());
            // Transition to 2FA
            *authenticated.write() = true;
        });
    };

    let on_forgot_password = move |_| {
        // TODO: Implement forgot password
    };



    if *authenticated.read() {
        rsx! {
            TwoFactorAuth {}
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
mod components;

use components::{CountdownRing, Logo, Toast, TokenCard};
use dioxus::prelude::*;
use rand::random;
use std::time::Duration;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut code1 = use_signal(|| random::<u8>() % 90 + 10);
    let mut code2 = use_signal(|| random::<u8>() % 90 + 10);
    let mut code3 = use_signal(|| random::<u8>() % 90 + 10);
    let mut remaining_secs = use_signal(|| 30u8);
    let mut manual_code = use_signal(|| String::new());
    let mut toast = use_signal(|| String::new());

    // Auto-refresh every 30 s
    use_future(move || async move {
        loop {
            std::thread::sleep(Duration::from_secs(1));
            remaining_secs.with_mut(|secs| {
                if *secs == 0 {
                    *secs = 30;
                    code1.set(random::<u8>() % 90 + 10);
                    code2.set(random::<u8>() % 90 + 10);
                    code3.set(random::<u8>() % 90 + 10);
                } else {
                    *secs = secs.saturating_sub(1);
                }
            });
        }
    });

    let mins  = remaining_secs() / 60;
    let secs_r = remaining_secs() % 60;
    let expiry = format!("expires in {:02}:{:02}", mins, secs_r);

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }
        // Screen — full-height gradient background, card centred
        div { class: "min-h-screen flex items-center justify-center p-5
                      bg-gradient-to-br from-accent via-white to-white",

            // Card
            div { class: "w-full max-w-sm bg-white rounded-3xl shadow-2xl
                          overflow-hidden",

                // Brand stripe at top of card
                div { class: "h-1 w-full bg-gradient-to-r from-primary to-secondary" }

                div { class: "px-6 pt-6 pb-5 flex flex-col gap-0",

                    // Title + subtitle
                    h1 { class: "text-2xl font-black text-center text-primary
                                 tracking-tight leading-tight",
                        "SnapTray 2FA"
                    }
                    p { class: "text-xs text-center text-brand-gray font-semibold
                                uppercase tracking-widest mt-1 mb-4",
                        "School cafeteria ordering — one-time password"
                    }

                    Logo {}

                    // Token cards row
                    div { class: "flex gap-2 mb-4",
                        TokenCard { value: code1(), is_primary: true }
                        TokenCard { value: code2(), is_primary: false }
                        TokenCard { value: code3(), is_primary: true }
                    }

                    // Countdown ring + label
                    div { class: "flex flex-col items-center gap-1 py-4
                                  border-y border-gray-100 mb-4",
                        CountdownRing { remaining: remaining_secs(), total: 30 }
                        p { class: "text-[0.68rem] font-bold uppercase tracking-[2px]
                                    text-brand-gray",
                            "{expiry}"
                        }
                    }

                    // Divider
                    div { class: "flex items-center gap-3 mb-4",
                        div { class: "flex-1 h-px bg-gray-100" }
                        span { class: "text-[0.65rem] font-bold uppercase tracking-widest
                                       text-gray-400",
                            "Enter code"
                        }
                        div { class: "flex-1 h-px bg-gray-100" }
                    }

                    // Number input
                    input {
                        class: "w-full px-3 py-4 rounded-2xl border-2 border-gray-100
                                bg-gray-50 text-center text-2xl font-black tracking-[0.5em]
                                text-gray-800 outline-none caret-primary
                                focus:border-primary focus:bg-white
                                focus:ring-4 focus:ring-primary/10
                                transition-all duration-150 mb-4",
                        placeholder: "000000",
                        value: "{manual_code()}",
                        oninput: move |e| manual_code.set(e.value().clone()),
                        maxlength: "6",
                        inputmode: "numeric",
                    }

                    // Buttons
                    div { class: "flex gap-2 mb-4",
                        button {
                            class: "flex-1 py-3.5 rounded-2xl font-bold text-sm
                                    bg-secondary text-gray-800
                                    shadow-[0_4px_12px_rgba(255,200,87,0.4)]
                                    active:scale-95 transition-transform duration-100",
                            onclick: move |_| {
                                code1.set(random::<u8>() % 90 + 10);
                                code2.set(random::<u8>() % 90 + 10);
                                code3.set(random::<u8>() % 90 + 10);
                                remaining_secs.set(30);
                            },
                            "↺  Refresh"
                        }
                        button {
                            class: "flex-1 py-3.5 rounded-2xl font-bold text-sm
                                    bg-primary text-white
                                    shadow-[0_4px_12px_rgba(255,107,53,0.4)]
                                    active:scale-95 transition-transform duration-100",
                            onclick: move |_| {
                                let expected = format!("{:02}{:02}{:02}", code1(), code2(), code3());
                                let entered = manual_code().trim().replace(' ', "").to_string();
                                if entered == expected {
                                    toast.set("✅ Code validated!".to_string());
                                } else {
                                    toast.set(format!("❌ Expected {}", expected));
                                }
                            },
                            "Submit"
                        }
                    }

                    Toast { message: toast() }
                }
            }
        }
    }
}


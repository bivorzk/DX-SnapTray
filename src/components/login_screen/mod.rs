use dioxus::prelude::*;

#[component]
pub fn LoginScreen(
    username: ReadSignal<String>,
    password: ReadSignal<String>,
    loading: ReadSignal<bool>,
    login_message: ReadSignal<Option<String>>,
    on_username: EventHandler<FormEvent>,
    on_password: EventHandler<FormEvent>,
    on_submit: EventHandler<FormEvent>,
    on_forgot: EventHandler<MouseEvent>,
) -> Element {
    let login_css = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/main.css"));
    let tailwind_css = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tailwind.css"));

    rsx! {
        document::Style { "{login_css}" }
        document::Style { "{tailwind_css}" }

        div { class: "min-h-screen bg-[#0f172a] flex items-center justify-center p-4",
            div { class: "bg-[#111827] rounded-lg p-8 w-full max-w-md shadow-xl",
                div { class: "flex justify-center mb-8",
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

                h1 { class: "text-2xl font-bold text-[#f8fafc] text-center mb-6", "Login to SnapTray" }

                form { class: "space-y-4", onsubmit: on_submit,
                    input {
                        r#type: "text",
                        class: "w-full px-3 py-2 bg-[#1f2937] border border-[#374151] rounded-lg text-[#f8fafc] focus:border-[#FF6B35] focus:outline-none",
                        placeholder: "Username",
                        value: "{username.read()}",
                        oninput: on_username,
                    }
                    input {
                        r#type: "password",
                        class: "w-full px-3 py-2 bg-[#1f2937] border border-[#374151] rounded-lg text-[#f8fafc] focus:border-[#FF6B35] focus:outline-none",
                        placeholder: "Password",
                        value: "{password.read()}",
                        oninput: on_password,
                    }
                    button {
                        r#type: "submit",
                        class: "w-full bg-[#FF6B35] hover:bg-[#e55a2b] text-white font-bold py-3 px-4 rounded-lg transition duration-200",
                        disabled: *loading.read(),
                        if *loading.read() {
                            "Logging in..."
                        } else {
                            "Login"
                        }
                    }
                }

                if let Some(msg) = login_message.read().as_ref() {
                    p { class: "text-[#16a34a] text-center mt-4", "{msg}" }
                }

                div { class: "text-center mt-4",
                    a {
                        href: "#",
                        onclick: on_forgot,
                        class: "text-[#FFC857] hover:text-[#FFE5DC]",
                        "Forgot Password?"
                    }
                }

                p { class: "bottom-note text-[#6C757D] text-sm text-center mt-4",
                    "If you don't have an account register at "
                    a {
                        href: "https://snaptray.onrender.com/register",
                        class: "text-[#FFC857] hover:text-[#FFE5DC]",
                        "Snaptray.onrender.com/register"
                    }
                }
            }
        }
    }
}

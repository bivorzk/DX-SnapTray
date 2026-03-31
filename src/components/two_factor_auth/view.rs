use dioxus::prelude::*;

/// Renders 3 large number buttons for the "match the number" 2FA screen.
/// `choices` â€” the 3 numbers to display (one is correct).
/// `on_pick`  â€” called with the chosen number as a String.
/// `disabled` â€” greys out all buttons while a request is in flight.
pub fn number_grid(
    choices: [u32; 3],
    on_pick: impl FnMut(u32) + Clone + 'static,
    disabled: bool,
) -> Element {
    let btn_base = "
        width: 100px; height: 100px;
        font-size: 2rem; font-weight: 800;
        border: 2px solid #334155;
        border-radius: 24px;
        background: #1e293b;
        color: #f8fafc;
        cursor: pointer;
        -webkit-tap-highlight-color: transparent;
        touch-action: manipulation;
        transition: transform 0.1s, box-shadow 0.1s;
        box-shadow: 0 4px 18px rgba(0,0,0,0.3);
    ";
    let btn_disabled = "
        width: 100px; height: 100px;
        font-size: 2rem; font-weight: 800;
        border: 2px solid #1e293b;
        border-radius: 24px;
        background: #111827;
        color: #334155;
        cursor: not-allowed;
        opacity: 0.5;
    ";

    rsx! {
        div { style: "display: flex; justify-content: center; gap: 18px; margin: 24px 0;",
            for (i , & num) in choices.iter().enumerate() {
                {
                    let mut cb = on_pick.clone();
                    rsx! {
                        button {
                            key: "{i}",
                            r#type: "button",
                            disabled,
                            style: if disabled { btn_disabled } else { btn_base },
                            onclick: move |_| cb(num),
                            "{num}"
                        }
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

pub fn otp_box_style() -> &'static str {
    "
        width:76px; height:76px;
        font-size:2rem; font-weight:700;
        text-align:center; color:#f8fafc;
        background:#1e293b; border:2px solid #334155;
        border-radius:18px; caret-color:transparent;
        -webkit-tap-highlight-color:transparent;
        touch-action:manipulation;
    "
}

pub fn otp_grid<FI1,FI2,FI3,FK1,FK2,FK3,FP>(
    digit1: String,
    digit2: String,
    digit3: String,
    on_input_1: FI1,
    on_input_2: FI2,
    on_input_3: FI3,
    on_key_1: FK1,
    on_key_2: FK2,
    on_key_3: FK3,
    on_paste: FP,
) -> Element
where
    FI1: FnMut(FormEvent) + 'static,
    FI2: FnMut(FormEvent) + 'static,
    FI3: FnMut(FormEvent) + 'static,
    FK1: FnMut(KeyboardEvent) + 'static,
    FK2: FnMut(KeyboardEvent) + 'static,
    FK3: FnMut(KeyboardEvent) + 'static,
    FP: FnMut(Event<ClipboardData>) + 'static,
{
    rsx! {
        div {
            role: "group",
            "aria-label": "3-digit one-time passcode",
            "aria-describedby": "otp-desc",
            style: "display:flex; flex-direction:column; align-items:center; gap:10px; margin-bottom:20px;",
            onpaste: on_paste,
            div { style: "display:flex; gap:10px;",
                input {
                    id: "otp-1",
                    r#type: "tel",
                    inputmode: "numeric",
                    maxlength: "1",
                    value: "{digit1}",
                    autocomplete: "one-time-code",
                    "aria-label": "Digit 1 of 3",
                    style: "{otp_box_style()}",
                    oninput: on_input_1,
                    onkeydown: on_key_1,
                }
                input {
                    id: "otp-2",
                    r#type: "tel",
                    inputmode: "numeric",
                    maxlength: "1",
                    value: "{digit2}",
                    "aria-label": "Digit 2 of 3",
                    style: "{otp_box_style()}",
                    oninput: on_input_2,
                    onkeydown: on_key_2,
                }
                input {
                    id: "otp-3",
                    r#type: "tel",
                    inputmode: "numeric",
                    maxlength: "1",
                    value: "{digit3}",
                    "aria-label": "Digit 3 of 3",
                    style: "{otp_box_style()}",
                    oninput: on_input_3,
                    onkeydown: on_key_3,
                }
            }
        }
    }
}

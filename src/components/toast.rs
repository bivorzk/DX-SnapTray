use dioxus::prelude::*;

#[component]
pub fn Toast(message: String) -> Element {
    let cls = if message.is_empty() {
        "min-h-[38px] flex items-center justify-center text-sm font-semibold \
         px-4 py-2 rounded-xl opacity-0 pointer-events-none"
    } else if message.contains('\u{2705}') {
        "min-h-[38px] flex items-center justify-center text-sm font-semibold \
         px-4 py-2 rounded-xl \
         text-green-800 bg-green-50 border border-green-200"
    } else {
        "min-h-[38px] flex items-center justify-center text-sm font-semibold \
         px-4 py-2 rounded-xl \
         text-red-800 bg-red-50 border border-red-200"
    };
    rsx! {
        div { class: "{cls}", "{message}" }
    }
}

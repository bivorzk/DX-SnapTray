use dioxus::prelude::*;

#[component]
pub fn TokenCard(value: u8, is_primary: bool) -> Element {
    let cls = if is_primary {
        "flex-1 h-[74px] rounded-2xl flex items-center justify-center \
         text-[2rem] font-black tracking-widest text-white select-none \
         bg-gradient-to-br from-primary to-primary-d \
         shadow-[0_8px_20px_rgba(0,0,0,0.15)]"
    } else {
        "flex-1 h-[74px] rounded-2xl flex items-center justify-center \
         text-[2rem] font-black tracking-widest text-gray-800 select-none \
         bg-gradient-to-br from-secondary to-secondary-d \
         shadow-[0_8px_20px_rgba(0,0,0,0.15)]"
    };
    rsx! {
        div { class: "{cls}", "{value:02}" }
    }
}

mod components;
mod login_and_twofa;
#[cfg(not(target_arch = "wasm32"))]
pub mod db;

use dioxus::prelude::*;
use login_and_twofa::LoginAndTwoFa;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dotenvy::dotenv().ok();

    dioxus::launch(LoginAndTwoFa);
}

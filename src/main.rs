mod components;
mod login_and_twofa;

use dioxus::prelude::*;
use login_and_twofa::LoginAndTwoFa;

fn main() {
    dioxus::launch(LoginAndTwoFa);
}

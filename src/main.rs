mod components;

use components::TwoFactorAuth;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(TwoFactorAuth);
}

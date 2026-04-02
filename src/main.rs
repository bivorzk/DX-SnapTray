mod components;
mod login_and_twofa;

use login_and_twofa::LoginAndTwoFa;

fn main() {
    // On non-server clients (mobile/desktop), point server functions to the Dioxus server.
    // SERVER_URL is emitted by build.rs from DX_SERVER_URL env var at compile time.
    #[cfg(not(feature = "server"))]
    {
        dioxus::fullstack::set_server_url(env!("SERVER_URL"));
    }

    dioxus::launch(LoginAndTwoFa);
}

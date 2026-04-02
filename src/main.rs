mod components;
mod login_and_twofa;

use login_and_twofa::LoginAndTwoFa;

fn main() {
    // On non-server clients (mobile/desktop), point server functions to the local Dioxus server.
    #[cfg(not(feature = "server"))]
    {
        let server_url = Box::leak(
            std::env::var("DX_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string())
                .into_boxed_str(),
        );
        dioxus::fullstack::set_server_url(server_url);
    }

    dioxus::launch(LoginAndTwoFa);
}

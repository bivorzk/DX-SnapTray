fn main() {
    // Re-run this build script whenever DX_SERVER_URL changes.
    println!("cargo:rerun-if-env-changed=DX_SERVER_URL");

    let url = std::env::var("DX_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    // Emit the URL as a compile-time env var readable via env!("SERVER_URL")
    println!("cargo:rustc-env=SERVER_URL={}", url);
}

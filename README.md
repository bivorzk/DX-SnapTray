# SnapTray Cafeteria Ordering App (2FA)

This repository contains the SnapTray cafeteria ordering app with a focus on **two-factor authentication (2FA)** for secure user login. The app uses Dioxus 0.7 fullstack server functions to unify authentication flows across desktop, web, and mobile.

## What this project includes

- `src/db.rs`: server-side MongoDB auth and 2FA logic under `#[server]` function.
- `src/login_and_twofa.rs`: login form, 2FA code entry, and auth flow.
- `src/components/two_factor_auth/*`: API, model, and UI components for 2FA.
- `src/main.rs`: app entrypoint + server URL config for clients.
- `Cargo.toml`: feature-gated dependencies for server/client builds.

## Quick start (development)

### Prerequisites

- Rust + Cargo
- Dioxus CLI (`dx`)
- Android SDK/NDK + `adb` (for mobile builds)
- MongoDB accessible via URI (local or cloud)

### Setup `.env`

Create a `.env` file in project root with the following **required** values:

```env
MONGODB_URI="mongodb+srv://your-user:your-password@cluster0.kkpdosb.mongodb.net/your-db?retryWrites=true&w=majority"
DB_NAME="snaptray"
USERS_COLLECTION="users"
PORT=8080
```

**Security note**: `MONGODB_URI` must include your MongoDB credentials. Never commit `.env` to Git—add it to `.gitignore`.

### Run local server

Use firewall rules and local IP to allow mobile device access.

```powershell
$env:IP = "0.0.0.0"
$env:PORT = "8080"
dx serve --platform web --addr 0.0.0.0 --port 8080
```

### Set client server URL

`src/main.rs` now supports an environment variable:

- `DX_SERVER_URL` (example: `http://192.168.1.248:8080`)

If not set, it defaults to `http://localhost:8080`.

### Run on desktop

```bash
dx build --platform desktop
dx launch --platform desktop
```

### Run on Android

```powershell
$env:DX_SERVER_URL = "http://<your-lan-ip>:8080"
$env:DX_SERVER_URL | dx build --platform android --target aarch64-linux-android
adb install -r "target\dx\snap-tray-auth\debug\android\app\app\build\outputs\apk\debug\app-debug.apk"
```

## Notes

- In production, the client should point to your real deployed server URL (e.g., `https://snaptray.onrender.com`) instead of localhost.
- 2FA uses an API endpoint with a generated code and validates it before granting access.

## Tailwind support

For Dioxus 0.7 the project includes automatic Tailwind support; simply run `dx serve` and it will resolve `tailwind.css` automatically.

If you use manual Tailwind control:

```bash
npm install -g @tailwindcss/cli
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

## Deploying

1. Build server-enabled binaries with `--features server` and deploy to a machine/host.
2. Deploy mobile/web clients with the proper `DX_SERVER_URL` for the deployed API.
3. Make sure your server can reach MongoDB and has environment variables set.

---

This app is centered around secure SnapTray cafeteria ordering with 2FA, making login safer while still supporting cross-platform (desktop/mobile/web) workflows.


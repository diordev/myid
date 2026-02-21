# MyID SDK — Rust Client

> ⚠️ **Diqqat:** Ushbu kutubxona hozirda **faol ishlab chiqilish jarayonida** (Work in Progress).
> API o'zgarishi mumkin. Production muhitida hali tayor emas.
> Xatolar va takliflar uchun [GitHub Issues](https://github.com/diordev/myid/issues) orqali murojaat qiling.

O'zbekiston Respublikasi **MyID** identifikatsiya tizimi uchun rasmiy bo'lmagan Rust SDK kutubxonasi.

## Imkoniyatlar

- **OAuth 2.0** — MyID API bilan autentifikatsiya, token avtomatik cache'lanadi
- **Type-safe** — PINFL, passport, telefon raqam va boshqa qiymatlar compile-time'da tekshiriladi
- **Async/await** — `tokio` runtime bilan to'liq asinxron ishlash
- **Xavfsizlik** — `client_secret` debug output'da avtomatik `<redacted>` sifatida ko'rsatiladi
- **Thread-safe** — `Send + Sync` compile-time kafolati, `Arc` orqali klon qilish xavfsiz

## O'rnatish

```toml
[dependencies]
myid = "0.1.7"
tokio = { version = "1", features = ["full"] }
```

### Featurelar

| Feature | Default | Tavsif |
|---------|---------|--------|
| `dotenvy` | ✅ yoqilgan | `.env` fayldan konfiguratsiya yuklash |

`.env` qo'llab-quvvatlash kerak bo'lmasa:

```toml
[dependencies]
myid = { version = "0.1.7", default-features = false }
```

## Tez boshlash

```rust
use myid::prelude::*;
use myid::types::BirthDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new("https://myid.uz", "client_id", "client_secret")?;
    let client = MyIdClient::new(config)?;

    // Token avtomatik olinadi va cache'lanadi
    let token = client.get_token().await?;
    println!("Token: {}... ({} belgi)", &token[..20], token.len());

    // Session yaratish — PINFL orqali
    let request = CreateSessionRequest::WithPinfl(
        SessionWithPinfl::new(
            Pinfl::parse("12345678901234")?,
            BirthDate::parse("1990-05-15")?,
        ),
    );

    let session = client.create_session(&request).await?;
    println!("Session ID: {}", session.session_id());

    Ok(())
}
```

## Konfiguratsiya

### To'g'ridan-to'g'ri kod orqali

```rust
use std::time::Duration;
use myid::config::Config;

let config = Config::new("https://myid.uz", "client_id", "client_secret")?
    .with_timeout(Duration::from_secs(30))          // HTTP so'rov timeout
    .with_connect_timeout(Duration::from_secs(5))   // TCP/TLS ulanish timeout
    .with_user_agent("my-backend/1.0")              // Custom User-Agent
    .with_proxy("http://proxy.corp.local:8080")?;   // Korporativ proxy
```

### Environment o'zgaruvchilari orqali

`.env` fayl yarating yoki environment'ni to'g'ridan-to'g'ri o'rnating:

```env
MYID_BASE_URL=https://myid.uz
MYID_CLIENT_ID=your_client_id
MYID_CLIENT_SECRET=your_client_secret

# Ixtiyoriy
MYID_TIMEOUT_MS=30000
MYID_CONNECT_TIMEOUT_MS=5000
MYID_USER_AGENT=my-backend/1.0
MYID_PROXY_URL=http://proxy.corp.local:8080
```

```rust
use myid::config::Config;

let config = Config::from_env(None)?;          // MYID_ prefiksi
let config = Config::from_env(Some("APP"))?;   // APP_ prefiksi
```

### Default qiymatlar

| Parametr | Default | O'zgartirish |
|----------|---------|-------------|
| HTTP timeout | 15 soniya | `with_timeout()` |
| Connection timeout | 2 soniya | `with_connect_timeout()` |
| User-Agent | `myid-client-rust/0.1` | `with_user_agent()` |
| Proxy | yo'q | `with_proxy()` |

## Session yaratish usullari

MyID 3 xil identifikatsiya usulini qo'llab-quvvatlaydi:

### PINFL orqali

```rust
use myid::prelude::*;
use myid::types::BirthDate;

let request = CreateSessionRequest::WithPinfl(
    SessionWithPinfl::new(
        Pinfl::parse("12345678901234")?,
        BirthDate::parse("1990-05-15")?,
    )
    .with_phone_number(PhoneNumber::parse("+998901234567")?)
    .with_threshold(Threshold::parse(0.85)?)
    .with_is_resident(true),
);
```

### Passport orqali

```rust
use myid::prelude::*;
use myid::types::BirthDate;

let request = CreateSessionRequest::WithPassport(
    SessionWithPassport::new(
        PassportData::parse("AB1234567")?,
        BirthDate::parse("1990-05-15")?,
    )
    .with_phone_number(PhoneNumber::parse("+998901234567")?),
);
```

### REUID orqali (secondary flow)

```rust
use myid::prelude::*;

let request = CreateSessionRequest::WithReuid(
    SessionWithReuid::new(Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1")?),
);
```

## Xatolarni boshqarish

Barcha xatolar `MyIdError` enum orqali qaytariladi:

| Variant | Sababi |
|---------|--------|
| `Config { message }` | Noto'g'ri URL, bo'sh credentials, proxy xatosi |
| `Validation { message }` | PINFL, passport, telefon raqam format xatosi |
| `Http(_)` | Tarmoq xatosi — timeout, DNS, TLS |
| `Api { status, message }` | Server 4xx/5xx javobi — 401, 403, 429, 500 |
| `Internal { message }` | SDK ichki xatosi (odatda yuz bermaydi) |

```rust
use myid::config::Config;
use myid::error::MyIdError;

match Config::new("not-a-url", "id", "secret") {
    Ok(_) => unreachable!(),
    Err(MyIdError::Config { message }) => eprintln!("Config xato: {message}"),
    Err(e) => eprintln!("Boshqa xato: {e}"),
}
```

API xatosini tekshirish:

```rust
use myid::error::{MyIdError, MyIdResult};

async fn create(client: &myid::client::MyIdClient) -> MyIdResult<()> {
    use myid::dto::CreateSessionRequest;

    match client.create_session(&CreateSessionRequest::Empty {}).await {
        Ok(resp) => println!("Session: {}", resp.session_id()),
        Err(MyIdError::Api { status: 401, .. }) => eprintln!("Credentials noto'g'ri"),
        Err(MyIdError::Api { status: 429, .. }) => eprintln!("Rate limit — qayta urining"),
        Err(MyIdError::Http(e)) => eprintln!("Tarmoq xatosi: {e}"),
        Err(e) => eprintln!("Xato: {e}"),
    }
    Ok(())
}
```

## Xavfsizlik

- `client_secret` **faqat backend muhitida** saqlang — frontend'ga bermang
- `Debug` output'da secret avtomatik `<redacted>` sifatida ko'rsatiladi
- Production'da credential'larni environment variable orqali bering, kodni hardcode qilmang

```rust
use myid::config::Config;

let config = Config::new("https://myid.uz", "my_app", "super_secret")?;
let debug = format!("{:?}", config);

assert!(debug.contains("<redacted>"));     // ✅ Secret yashirilgan
assert!(!debug.contains("super_secret")); // ✅ Hech qachon ko'rinmaydi
```

## Misollar ishga tushirish

```bash
# Config yaratish va xatolarni ko'rish
cargo run --example config

# Client va session misollari (.env fayl kerak)
cargo run --example client

```

`.env` fayl namunasi:

```env
MYID_BASE_URL=https://myid.uz
MYID_CLIENT_ID=your_id
MYID_CLIENT_SECRET=your_secret
```

## Minimal Rust versiyasi (MSRV)

Rust **1.93.0** yoki undan yuqori talab qilinadi.

## Litsenziya

Ushbu loyiha ikki litsenziya ostida tarqatiladi — o'zingizga qulayini tanlang:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)

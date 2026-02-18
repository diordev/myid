# MyID SDK — Rust Client
> ⚠️ **Diqqat:** Ushbu kutubxona hozirda **faol ishlab chiqilish jarayonida** (Work in Progress).
> API o'zgarishi mumkin. Production muhitida hali tayor emas.
> Xatolar va takliflar uchun [GitHub Issues](https://github.com/diordev/myid/issues) orqali murojaat qiling.

O'zbekiston Respublikasi **MyID** identifikatsiya tizimi uchun rasmiy bo'lmagan Rust SDK kutubxonasi.

## Imkoniyatlar

- **OAuth 2.0** autentifikatsiya
- **Type-safe** konfiguratsiya — compile-time'da URL validatsiyasi
- **Async/await** — `tokio` runtime bilan to'liq asinxron ishlash
- **Xavfsizlik** — `client_secret` debug output'da avtomatik yashiriladi
- **Thread-safe** — `Send + Sync` compile-time kafolati

## O'rnatish

```toml
[dependencies]
myid = "0.1.2"
```

## Tez boshlash

```rust
use myid::config::Config;
use myid::error::MyIdResult;

fn main() -> MyIdResult<()> {
    let config = Config::new(
        "https://myid.uz",
        "your_client_id",
        "your_client_secret",
    )?;

    println!("Base URL: {}", config.base_url());
    Ok(())
}
```

## Konfiguratsiya

`Config` — SDK ning asosiy konfiguratsiya strukturasi. 3 ta majburiy parametr va
ixtiyoriy sozlamalar mavjud:

```rust
use std::time::Duration;
use myid::config::Config;
# use myid::error::MyIdResult;

# fn main() -> MyIdResult<()> {
let config = Config::new("https://myid.uz", "client_id", "client_secret")?
    .with_timeout(Duration::from_secs(30))          // HTTP so'rov timeout
    .with_connect_timeout(Duration::from_secs(5))   // TCP/TLS ulanish timeout
    .with_user_agent("my-backend/1.0")              // Custom User-Agent
    .with_proxy("http://proxy.local:8080")?;        // Korporativ proxy
# Ok(())
# }
```

### Default qiymatlar

| Parametr | Default | Metod |
|----------|---------|-------|
| HTTP timeout | 15 soniya | `with_timeout()` |
| Connection timeout | 2 soniya | `with_connect_timeout()` |
| User-Agent | `myid-client-rust/0.1` | `with_user_agent()` |
| Proxy | `None` | `with_proxy()` |

## Xatolarni boshqarish

Barcha xatolar `MyIdError` enum orqali qaytariladi:

```rust
use myid::config::Config;
use myid::error::MyIdError;

match Config::new("noto'g'ri-url", "id", "secret") {
    Ok(cfg) => println!("Muvaffaqiyatli: {}", cfg.base_url()),
    Err(MyIdError::Config { message }) => {
        eprintln!("Konfiguratsiya xatosi: {message}");
    }
}
```

## Xavfsizlik

- `client_secret` **faqat backend muhitida** saqlang
- `Debug` output'da secret avtomatik `<redacted>` sifatida ko'rsatiladi
- Frontend yoki client-side kodda **ishlatmang**
- Production'da credential'larni environment variable orqali bering

```rust
# use myid::config::Config;
# use myid::error::MyIdResult;
# fn main() -> MyIdResult<()> {
let config = Config::new("https://myid.uz", "app", "my_secret_123")?;
let debug = format!("{:?}", config);

assert!(debug.contains("<redacted>"));       // ✅ Yashirilgan
assert!(!debug.contains("my_secret_123"));   // ✅ Secret ko'rinmaydi
# Ok(())
# }
```

## Misollar

```bash
# Config yaratish misoli
cargo run --example config
```

## Minimal Rust versiyasi (MSRV)

Rust **1.93.0** yoki undan yuqori talab qilinadi.

## Litsenziya

Ushbu loyiha ikki litsenziya ostida tarqatiladi — o'zingizga qulayini tanlang:

- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)

Batafsil ma'lumot uchun tegishli litsenziya fayllarini ko'ring.

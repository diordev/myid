//! MyID OAuth 2.0 autentifikatsiya DTO'lari.

use serde::{Deserialize, Serialize};

/// OAuth 2.0 access token so'rovi.
///
/// MyID API ga `client_id` va `client_secret` yuborib,
/// access token olish uchun ishlatiladi.
///
/// # Xavfsizlik
///
/// - `client_secret` [`Debug`] output'da `[REDACTED]` sifatida ko'rsatiladi
/// - Lifetime `'a` — credential'lar [`Config`](crate::config::Config) dan borrow qilinadi,
///   ortiqcha allokatsiya bo'lmaydi
///
/// # Misol (crate ichida)
///
/// ```rust,ignore
/// let body = AccessTokenRequest {
///     client_id: config.client_id(),
///     client_secret: config.client_secret(),
/// };
/// ```
#[derive(Clone, Serialize)]
pub struct AccessTokenRequest<'a> {
    /// OAuth 2.0 client identifikator (public).
    pub client_id: &'a str,
    /// OAuth 2.0 client secret (**maxfiy**).
    ///
    /// Debug output'da `[REDACTED]` sifatida ko'rsatiladi.
    pub client_secret: &'a str,
}

impl std::fmt::Debug for AccessTokenRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenRequest")
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .finish()
    }
}

/// OAuth 2.0 access token javobi.
///
/// MyID API muvaffaqiyatli autentifikatsiyadan keyin qaytaradi.
///
/// # Fieldlar
///
/// | Field | Tavsif |
/// |-------|--------|
/// | `access_token` | Bearer token — API so'rovlarida `Authorization` headerda yuboriladi |
/// | `expires_in` | Token amal qilish muddati (sekundlarda) |
/// | `token_type` | Har doim `"Bearer"` |
#[derive(Clone, Deserialize)]
pub struct AccessTokenResponse {
    /// Bearer access token qiymati.
    pub access_token: String,
    /// Token amal qilish muddati sekundlarda.
    ///
    /// Masalan: `3600` = 1 soat.
    pub expires_in: u64,
    /// Token turi — odatda `"Bearer"`.
    pub token_type: String,
}

impl std::fmt::Debug for AccessTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenResponse")
            .field("access_token", &"[REDACTED]")
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .finish()
    }
}

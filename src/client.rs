//! MyID API client moduli.
//!
//! [`MyIdClient`] — SDK ning asosiy tashqi interfeysi. Barcha API
//! so'rovlari shu struct orqali yuboriladi.
//!
//! # Arxitektura
//!
//! ```text
//! MyIdClient::new(config)
//!     │
//!     ├── build_http_client()  ← timeout, proxy, user-agent
//!     └── token cache (Arc<Mutex>)
//!           │
//!           ├── get_token()
//!           │     ├── read_cached_token() → Some(token) ✅
//!           │     └── authenticate() → yangi token → write_cached_token()
//!           │
//!           ├── create_session(&request) → SessionResponse
//!           ├── handle_callback(code)   → UserDataResponse
//!           └── recover_session(id)     → SessionStatusResponse
//! ```
//!
//! # Misollar
//!
//! ```rust,no_run
//! use myid::prelude::*;
//! use myid::types::BirthDate;
//!
//! # async fn example() -> MyIdResult<()> {
//! let config = Config::new("https://myid.uz", "client_id", "secret")?;
//! let client = MyIdClient::new(config)?;
//!
//! let request = CreateSessionRequest::WithPinfl(
//!     SessionWithPinfl::new(
//!         Pinfl::parse("12345678901234")?,
//!         BirthDate::parse("1990-05-15")?,
//!     ),
//! );
//!
//! let session = client.create_session(&request).await?;
//! println!("Session ID: {}", session.session_id());
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use reqwest::{Client, Proxy, StatusCode};
use url::Url;

use crate::config::Config;
use crate::dto::{
    AccessTokenRequest, AccessTokenResponse, ApiErrorBody, CreateSessionRequest, SessionResponse,
    SessionStatusResponse, UserDataResponse,
};
use crate::error::{MyIdError, MyIdResult};
use crate::types::SessionId;

const ACCESS_TOKEN_PATH: &str = "api/v1/auth/clients/access-token";
const CREATE_SESSION_PATH: &str = "api/v2/sdk/sessions";
const USER_DATA_PATH: &str = "api/v1/sdk/data";
const SESSION_RECOVERY_PATH: &str = "api/v1/sdk/sessions";
const AUTH_MAX_ATTEMPTS: u8 = 4; // 1 asosiy urinish + 3 retry
const AUTH_RETRY_BASE_MS: u64 = 100;
const AUTH_RETRY_MAX_MS: u64 = 2_000;

/// MyID API client.
///
/// Barcha API so'rovlari shu struct orqali yuboriladi.
/// Token avtomatik cache'lanadi va muddati o'tganda yangilanadi.
///
/// # Thread-safety
///
/// `MyIdClient` `Clone` qilganda token cache **umumiy** qoladi (`Arc`).
/// Bu `tokio::spawn` bilan xavfsiz ishlatish imkonini beradi:
///
/// ```rust,no_run
/// # use myid::prelude::*;
/// # async fn example() -> MyIdResult<()> {
/// # let config = Config::new("https://myid.uz", "id", "secret")?;
/// let client = MyIdClient::new(config)?;
///
/// let c1 = client.clone();
/// tokio::spawn(async move {
///     let token = c1.get_token().await;
/// });
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct MyIdClient {
    config: Config,
    http: Client,
    token: Arc<Mutex<Option<TokenState>>>,
    refresh_lock: Arc<Mutex<()>>,
    token_refresh_margin: Duration,
}

#[derive(Clone)]
struct TokenState {
    access_token: String,
    expires_at: Instant,
}

impl TokenState {
    fn is_valid(&self, margin: Duration) -> bool {
        Instant::now() + margin < self.expires_at
    }
}

impl MyIdClient {
    /// Yangi `MyIdClient` yaratadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Http`] qaytaradi agar HTTP client yaratishda xato bo'lsa
    /// (masalan: noto'g'ri proxy URL).
    ///
    /// # Misollar
    ///
    /// ```rust
    /// # use myid::prelude::*;
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?;
    /// let client = MyIdClient::new(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: Config) -> MyIdResult<Self> {
        let http = Self::build_http_client(&config)?;

        Ok(Self {
            config,
            http,
            token: Arc::new(Mutex::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
            token_refresh_margin: Duration::from_secs(60),
        })
    }

    /// Session yaratadi (`POST /api/v2/sdk/sessions`).
    ///
    /// Token avtomatik cache'dan olinadi yoki yangilanadi.
    ///
    /// # Misollar
    ///
    /// ```rust,no_run
    /// # use myid::prelude::*;
    /// # use myid::types::BirthDate;
    /// # async fn example() -> MyIdResult<()> {
    /// # let config = Config::new("https://myid.uz", "id", "secret")?;
    /// # let client = MyIdClient::new(config)?;
    /// let request = CreateSessionRequest::WithPinfl(
    ///     SessionWithPinfl::new(
    ///         Pinfl::parse("12345678901234")?,
    ///         BirthDate::parse("1990-05-15")?,
    ///     ),
    /// );
    ///
    /// let session = client.create_session(&request).await?;
    /// println!("Session: {}", session.session_id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_session(
        &self,
        request: &CreateSessionRequest,
    ) -> MyIdResult<SessionResponse> {
        let token = self.get_token().await?;
        let url = self.endpoint(CREATE_SESSION_PATH)?;

        let response = self
            .http
            .post(url.as_str())
            .bearer_auth(&token)
            .json(request)
            .send()
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            self.invalidate_cached_token().await;

            let retry_token = self.get_token().await?;
            let retry_response = self
                .http
                .post(url.as_str())
                .bearer_auth(retry_token)
                .json(request)
                .send()
                .await?;

            return Self::handle_response(retry_response).await;
        }

        Self::handle_response(response).await
    }

    /// Cache'dan token oladi, muddati o'tgan bo'lsa yangilaydi.
    ///
    /// # Ishlash tartibi
    ///
    /// 1. Cache'dan o'qish → token valid bo'lsa qaytarish
    /// 2. Cache bo'sh yoki expired → API ga so'rov (`authenticate`)
    /// 3. Yangi tokenni cache'ga yozish
    pub async fn get_token(&self) -> MyIdResult<String> {
        if let Some(token) = self.read_cached_token().await {
            return Ok(token);
        }

        // Single-flight: bir vaqtning o'zida faqat bitta refresh ishlaydi.
        let _refresh_guard = self.refresh_lock.lock().await;

        // Lock kutish vaqtida boshqa task tokenni yangilagan bo'lishi mumkin.
        if let Some(token) = self.read_cached_token().await {
            return Ok(token);
        }

        let fresh = self.authenticate().await?;
        self.write_cached_token(fresh).await
    }

    /// Mavjud sessionni tiklaydi (`GET /api/v1/sdk/sessions/{session_id}`).
    ///
    /// Agar mobil ilovadan `code` backend'ga yetib kelmasa,
    /// session tugagandan **10 daqiqa** ichida shu metod orqali holat so'raladi.
    ///
    /// `status` qiymatlari: `in_progress` | `closed` | `expired`.
    /// `closed` holatida `code` field mavjud bo'ladi.
    ///
    /// Token avtomatik cache'dan olinadi yoki yangilanadi.
    ///
    /// # Misollar
    ///
    /// ```rust,no_run
    /// # use myid::prelude::*;
    /// # async fn example() -> MyIdResult<()> {
    /// # let config = Config::new("https://myid.uz", "id", "secret")?;
    /// # let client = MyIdClient::new(config)?;
    /// let session_id = SessionId::parse("550e8400-e29b-41d4-a716-446655440000")?;
    ///
    /// let result = client.recover_session(session_id).await?;
    /// println!("Status: {:?}", result.status());
    /// if let Some(code) = result.code() {
    ///     println!("Code: {}", code);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn recover_session(
        &self,
        session_id: SessionId,
    ) -> MyIdResult<SessionStatusResponse> {
        let token = self.get_token().await?;
        let url = self.endpoint(&format!("{}/{}", SESSION_RECOVERY_PATH, session_id))?;

        let response = self
            .http
            .get(url.as_str())
            .bearer_auth(&token)
            .send()
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            self.invalidate_cached_token().await;

            let retry_token = self.get_token().await?;
            let retry_response = self
                .http
                .get(url.as_str())
                .bearer_auth(retry_token)
                .send()
                .await?;

            return Self::handle_response(retry_response).await;
        }

        Self::handle_response(response).await
    }

    /// MyID callback kodini qayta ishlaydi va foydalanuvchi ma'lumotlarini qaytaradi.
    ///
    /// Mobil ilova MyID dan olgan `code` ni backend ga yuboradi, backend esa
    /// shu metod orqali MyID API dan foydalanuvchining to'liq profilini oladi.
    ///
    /// # Parametrlar
    ///
    /// - `code` — MyID tomonidan berilgan bir martalik UUID (TTL: 5 daqiqa).
    ///
    /// # Xatolar
    ///
    /// - [`MyIdError::Validation`] — `code` bo'sh string bo'lsa.
    /// - [`MyIdError::Api`] — `code` muddati o'tgan yoki allaqachon ishlatilgan bo'lsa
    ///   (MyID server `401`/`400` qaytaradi).
    /// - [`MyIdError::Http`] — tarmoq xatosi.
    ///
    /// # Misol
    ///
    /// ```rust,no_run
    /// # use myid::prelude::*;
    /// # async fn example() -> MyIdResult<()> {
    /// # let config = Config::new("https://myid.uz", "id", "secret")?;
    /// # let client = MyIdClient::new(config)?;
    /// let code = "550e8400-e29b-41d4-a716-446655440000".to_string();
    ///
    /// match client.handle_callback(code).await {
    ///     Ok(user_data) => {
    ///         println!("Foydalanuvchi: {:?}", user_data);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Xato: {}", e);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn handle_callback(&self, code: String) -> MyIdResult<UserDataResponse> {
        if code.trim().is_empty() {
            return Err(MyIdError::validation("code bo'sh bo'lishi mumkin emas"));
        }

        let token = self.get_token().await?;
        let mut url = self.endpoint(USER_DATA_PATH)?;
        url.query_pairs_mut().append_pair("code", &code);
        let callback_url = url.to_string();

        let response = self
            .http
            .get(callback_url.as_str())
            .bearer_auth(&token)
            .send()
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            self.invalidate_cached_token().await;

            let retry_token = self.get_token().await?;
            let retry_response = self
                .http
                .get(callback_url.as_str())
                .bearer_auth(retry_token)
                .send()
                .await?;

            return Self::handle_response(retry_response).await;
        }

        Self::handle_response(response).await
    }

    // --- Private: API methods ---

    /// MyID API dan access token oladi. Cache ishlatmaydi.
    ///
    /// Vaqtinchalik xatolarda (`429`, `5xx`, timeout/connect/request xatolari)
    /// exponential backoff bilan bir necha marta qayta urinadi.
    async fn authenticate(&self) -> MyIdResult<AccessTokenResponse> {
        let url = self.endpoint(ACCESS_TOKEN_PATH)?;
        let mut attempt: u8 = 1;

        loop {
            let body = AccessTokenRequest {
                client_id: self.config.client_id(),
                client_secret: self.config.client_secret(),
            };

            let result = match self.http.post(url.as_str()).json(&body).send().await {
                Ok(response) => Self::handle_response(response).await,
                Err(e) => Err(MyIdError::http(e)),
            };

            match result {
                Ok(token) => return Ok(token),
                Err(err) if attempt < AUTH_MAX_ATTEMPTS && Self::is_retryable_auth_error(&err) => {
                    tokio::time::sleep(Self::auth_retry_backoff(attempt)).await;
                    attempt += 1;
                }
                Err(err) => return Err(err),
            }
        }
    }

    // --- Private: Token cache ---

    /// Cache'dan tokenni o'qiydi.
    async fn read_cached_token(&self) -> Option<String> {
        let guard = self.token.lock().await;
        guard.as_ref().and_then(|state| {
            if state.is_valid(self.token_refresh_margin) {
                Some(state.access_token.clone())
            } else {
                None
            }
        })
    }

    /// Cache'dagi tokenni majburiy tozalaydi.
    async fn invalidate_cached_token(&self) {
        let mut guard = self.token.lock().await;
        *guard = None;
    }

    /// Yangi tokenni cache'ga yozadi va token stringni qaytaradi.
    async fn write_cached_token(&self, token: AccessTokenResponse) -> MyIdResult<String> {
        const MAX_TTL_SECS: u64 = 31_536_000; // 365 kun

        if token.expires_in == 0 {
            return Err(MyIdError::internal("expires_in must be > 0"));
        }

        if token.expires_in > MAX_TTL_SECS {
            return Err(MyIdError::internal(format!(
                "expires_in too large: {}",
                token.expires_in
            )));
        }

        let expires_at = Instant::now()
            .checked_add(Duration::from_secs(token.expires_in))
            .ok_or_else(|| MyIdError::internal("expires_at overflow"))?;

        let mut guard = self.token.lock().await;
        *guard = Some(TokenState {
            access_token: token.access_token.clone(),
            expires_at,
        });

        Ok(token.access_token)
    }

    // --- Private: Helpers ---

    /// Base URL ga endpoint path qo'shadi.
    fn endpoint(&self, path: &str) -> MyIdResult<Url> {
        self.config
            .base_url_parsed()
            .join(path)
            .map_err(|e| MyIdError::config(format!("invalid endpoint `{path}`: {e}")))
    }

    /// API javobini tekshiradi — success bo'lsa deserialize, aks holda xato.
    ///
    /// Xato holatlarda `{"err": "...", "detail": "..."}` strukturasi parse qilinadi.
    /// Parse qilinmasa raw body xato xabari sifatida ishlatiladi.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> MyIdResult<T> {
        if response.status().is_success() {
            return Ok(response.json().await?);
        }

        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "response body o'qib bo'lmadi".to_string());

        let message = serde_json::from_str::<ApiErrorBody>(&body)
            .map(|e| e.message())
            .unwrap_or(body);

        Err(MyIdError::api(status, message))
    }

    /// Reqwest HTTP client yaratadi.
    fn build_http_client(config: &Config) -> MyIdResult<Client> {
        let mut builder = Client::builder()
            .connect_timeout(config.connection_timeout())
            .timeout(config.timeout())
            .user_agent(config.user_agent());

        if let Some(proxy_url) = config.proxy_url() {
            let proxy = Proxy::all(proxy_url)?;
            builder = builder.proxy(proxy);
        }

        Ok(builder.build()?)
    }
    
    fn is_retryable_auth_error(err: &MyIdError) -> bool {
        match err {
            MyIdError::Api { status, .. } => *status == 429 || (500..=599).contains(status),
            MyIdError::Http(source) => {
                source.is_timeout() || source.is_connect() || source.is_request()
            }
            _ => false,
        }
    }

    fn auth_retry_backoff(attempt: u8) -> Duration {
        let shift = u32::from(attempt.saturating_sub(1));
        let factor = 1_u64.checked_shl(shift).unwrap_or(u64::MAX);
        let millis = AUTH_RETRY_BASE_MS
            .saturating_mul(factor)
            .min(AUTH_RETRY_MAX_MS);
        Duration::from_millis(millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_retry_backoff_grows_and_caps() {
        assert_eq!(
            MyIdClient::auth_retry_backoff(1),
            Duration::from_millis(100)
        );
        assert_eq!(
            MyIdClient::auth_retry_backoff(2),
            Duration::from_millis(200)
        );
        assert_eq!(
            MyIdClient::auth_retry_backoff(3),
            Duration::from_millis(400)
        );
        assert_eq!(
            MyIdClient::auth_retry_backoff(4),
            Duration::from_millis(800)
        );
        assert_eq!(
            MyIdClient::auth_retry_backoff(10),
            Duration::from_millis(2_000)
        );
    }

    #[test]
    fn retryable_auth_api_statuses() {
        assert!(MyIdClient::is_retryable_auth_error(&MyIdError::api(
            429,
            "rate limit"
        )));
        assert!(MyIdClient::is_retryable_auth_error(&MyIdError::api(
            500, "internal"
        )));
        assert!(MyIdClient::is_retryable_auth_error(&MyIdError::api(
            502,
            "bad gateway"
        )));
        assert!(MyIdClient::is_retryable_auth_error(&MyIdError::api(
            503,
            "unavailable"
        )));
    }

    #[test]
    fn non_retryable_auth_api_statuses() {
        assert!(!MyIdClient::is_retryable_auth_error(&MyIdError::api(
            400,
            "bad request"
        )));
        assert!(!MyIdClient::is_retryable_auth_error(&MyIdError::api(
            401,
            "unauthorized"
        )));
        assert!(!MyIdClient::is_retryable_auth_error(&MyIdError::api(
            403,
            "forbidden"
        )));
    }
}

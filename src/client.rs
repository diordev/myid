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
//!           └── recover_session(id)     → String
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

use reqwest::{Client, Proxy};
use url::Url;

use crate::config::Config;
use crate::dto::{
    AccessTokenRequest, AccessTokenResponse, CreateSessionRequest, SessionResponse,
    UserDataResponse,
};
use crate::error::{MyIdError, MyIdResult};
use crate::types::SessionId;

const ACCESS_TOKEN_PATH: &str = "api/v1/auth/clients/access-token";
const CREATE_SESSION_PATH: &str = "api/v2/sdk/sessions";
const USER_DATA_PATH: &str = "api/v1/sdk/data";
const SESSION_RECOVERY_PATH: &str = "api/v1/sdk/sessions";

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
            .bearer_auth(token)
            .json(request)
            .send()
            .await?;

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

        let fresh = self.authenticate().await?;
        self.write_cached_token(fresh).await
    }

    /// Mavjud sessionni tiklaydi (`GET /api/v1/sdk/sessions/{session_id}`).
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
    /// let session_id = SessionId::parse("some-session-id")?;
    ///
    /// let result = client.recover_session(session_id).await?;
    /// println!("Recovered: {}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn recover_session(&self, session_id: SessionId) -> MyIdResult<String> {
        let token = self.get_token().await?;
        let url = self.endpoint(&format!("{}/{}", SESSION_RECOVERY_PATH, session_id))?;
        
        let response = self
            .http
            .get(url.as_str())
            .bearer_auth(token)
            .send()
            .await?;

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

        let response = self
            .http
            .get(url.as_str())
            .bearer_auth(token)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    // --- Private: API methods ---

    /// MyID API dan access token oladi. Cache ishlatmaydi.
    async fn authenticate(&self) -> MyIdResult<AccessTokenResponse> {
        let url = self.endpoint(ACCESS_TOKEN_PATH)?;

        let body = AccessTokenRequest {
            client_id: self.config.client_id(),
            client_secret: self.config.client_secret(),
        };

        let response = self.http.post(url.as_str()).json(&body).send().await?;

        Self::handle_response(response).await
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

        Err(MyIdError::api(status, body))
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
}

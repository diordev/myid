//! MyID API xato javob strukturasi.

use serde::Deserialize;

/// MyID API xato javobi (4xx/5xx).
///
/// Server quyidagi formatda qaytaradi:
/// ```json
/// {"err": "optional string", "detail": "error message"}
/// ```
///
/// `handle_response()` tomonidan ichki ishlatiladi — xato xabarini
/// tuzilmali ravishda parse qiladi, muvaffaqiyatsiz bo'lsa raw body fallback.
#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorBody {
    /// Qo'shimcha xato kodi yoki nomi (ixtiyoriy).
    pub err: Option<String>,
    /// Asosiy xato xabari.
    pub detail: Option<String>,
}

impl ApiErrorBody {
    /// Eng aniq xato xabarini qaytaradi.
    ///
    /// Ustuvorlik: `err: detail` → `detail` → `err` → `"unknown api error"`.
    pub fn message(&self) -> String {
        match (&self.err, &self.detail) {
            (Some(e), Some(d)) => format!("{e}: {d}"),
            (None, Some(d)) => d.to_owned(),
            (Some(e), None) => e.to_owned(),
            (None, None) => "unknown api error".to_string(),
        }
    }
}

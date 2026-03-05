//! MyID foydalanuvchi ma'lumotlari javob DTO'lari.
//!
//! Primary va secondary flow uchun response strukturalari:
//!
//! | Struct              | Tavsif                              |
//! |---------------------|-------------------------------------|
//! | [`UserDataResponse`]| API asosiy javob wrapper'i          |
//! | [`UserData`]        | Asosiy ma'lumotlar bloki            |
//! | [`Profile`]         | To'liq profil (primary flow)        |
//! | [`CommonData`]      | Shaxsiy ma'lumotlar                 |
//! | [`DocData`]         | Pasport ma'lumotlari                |
//! | [`ReuId`]           | Qayta ishlatiladigan identifikator  |

use serde::Deserialize;

use crate::types::{JobId, Reuid};

/// MyID API foydalanuvchi ma'lumotlari javobi (`GET /api/v1/sdk/data?code=`).
///
/// Ikkala flow uchun umumiy struktura:
///
/// | Flow | `data.profile` | `reuid` |
/// |------|----------------|---------|
/// | Primary | `Some(Profile)` — to'liq shaxsiy ma'lumotlar | `Some(ReuId)` |
/// | Secondary | `None` | `None` |
#[derive(Debug, Deserialize)]
pub struct UserDataResponse {
    /// Foydalanuvchining shaxsiy va pasport ma'lumotlari.
    pub data: UserData,
    /// Sessiya uchun qayta ishlatiladigan unikal identifikator (ReUID).
    ///
    /// Primary flow da `Some(ReuId)`, secondary flow da `None`.
    pub reuid: Option<ReuId>,
}

/// Foydalanuvchining asosiy ma'lumotlari bloki.
///
/// Secondary flow da `profile` `None` bo'ladi.
#[derive(Debug, Deserialize)]
pub struct UserData {
    /// Yuz taqqoslash natijasi (0.0 dan 1.0 gacha). Masalan: `0.92`.
    pub comparison_value: f64,
    /// Shifrlangan pasport ma'lumoti (raw string).
    pub pass_data: String,
    /// Verifikatsiya jarayonining unikal ish identifikatori (UUID).
    pub job_id: JobId,
    /// Foydalanuvchining to'liq profil ma'lumotlari.
    /// Primary flow da `Some(Profile)`, secondary flow da `None`.
    pub profile: Option<Profile>,
}

/// Foydalanuvchi profilining barcha bo'limlari.
#[derive(Debug, Deserialize)]
pub struct Profile {
    /// Umumiy shaxsiy ma'lumotlar: ism, familiya, PINFL, tug'ilgan sana va boshqalar.
    pub common_data: CommonData,
    /// Hujjat ma'lumotlari: pasport seriyasi, berilgan sana, amal qilish muddati va boshqalar.
    pub doc_data: DocData,
    /// Aloqa ma'lumotlari: telefon raqami va elektron pochta. Mavjud bo'lmasa `null`.
    pub contacts: Option<Contacts>,
    /// Ro'yxatdan o'tish manzili (doimiy va vaqtinchalik). Mavjud bo'lmasa `null`.
    pub address: Option<Address>,
}

/// Foydalanuvchining umumiy shaxsiy ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct CommonData {
    /// Ism (kirill alifbosida).
    pub first_name: String,
    /// Otasining ismi. Mavjud bo'lmasa `null`.
    pub middle_name: Option<String>,
    /// Familiya.
    pub last_name: String,
    /// Ism (lotin alifbosida). Mavjud bo'lmasa `null`.
    pub first_name_en: Option<String>,
    /// Familiya (lotin alifbosida). Mavjud bo'lmasa `null`.
    pub last_name_en: Option<String>,
    /// 14 xonali shaxsiy identifikatsiya raqami (PINFL).
    pub pinfl: String,
    /// Jinsi.
    pub gender: String,
    /// Tug'ilgan joyi.
    pub birth_place: String,
    /// Tug'ilgan davlat nomi (agar mavjud bo'lsa).
    pub birth_country: Option<String>,
    /// Tug'ilgan davlat ID (agar mavjud bo'lsa).
    pub birth_country_id: Option<String>,
    /// Tug'ilgan davlat CBU ID (agar mavjud bo'lsa).
    pub birth_country_id_cbu: Option<String>,
    /// Tug'ilgan sanasi (YYYY-MM-DD formatida).
    pub birth_date: String,
    /// Millati.
    pub nationality: String,
    /// Millat ID (agar mavjud bo'lsa).
    pub nationality_id: Option<String>,
    /// Millat CBU ID (agar mavjud bo'lsa).
    pub nationality_id_cbu: Option<String>,
    /// Fuqaroligi.
    pub citizenship: String,
    /// Fuqarolik ID (agar mavjud bo'lsa).
    pub citizenship_id: Option<String>,
    /// Fuqarolik CBU ID (agar mavjud bo'lsa).
    pub citizenship_id_cbu: Option<String>,
    /// SDK tomonidan hisoblangan xesh.
    pub sdk_hash: String,
    /// Pasport ma'lumotlari oxirgi yangilangan vaqt.
    pub last_update_pass_data: String,
    /// Manzil ma'lumotlari oxirgi yangilangan vaqt.
    pub last_update_address: String,
}

/// Foydalanuvchining hujjat (pasport) ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct DocData {
    /// Pasport seriyasi va raqami.
    pub pass_data: String,
    /// Hujjatni bergan organ nomi.
    pub issued_by: String,
    /// Hujjatni bergan organ identifikatori.
    pub issued_by_id: String,
    /// Hujjat berilgan sana.
    pub issued_date: String,
    /// Hujjat amal qilish muddati tugash sanasi.
    pub expiry_date: String,
    /// Hujjat turi nomi.
    pub doc_type: String,
    /// Hujjat turi identifikatori.
    pub doc_type_id: String,
    /// Hujjat turining CBU identifikatori.
    pub doc_type_id_cbu: String,
}

/// Foydalanuvchining aloqa ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct Contacts {
    /// Telefon raqami. Mavjud bo'lmasa `null`.
    pub phone: Option<String>,
    /// Elektron pochta manzili. Mavjud bo'lmasa `null`.
    pub email: Option<String>,
}

/// Foydalanuvchining manzil va ro'yxatdan o'tish ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct Address {
    /// Doimiy yashash manzili (matn ko'rinishida). Mavjud bo'lmasa `null`.
    pub permanent_address: Option<String>,
    /// Vaqtinchalik yashash manzili. Mavjud bo'lmasa `null`.
    pub temporary_address: Option<String>,
    /// Doimiy ro'yxatdan o'tish tafsilotlari. Mavjud bo'lmasa `null`.
    pub permanent_registration: Option<PermanentRegistration>,
    /// Vaqtinchalik ro'yxatdan o'tish tafsilotlari. Mavjud bo'lmasa `null`.
    pub temporary_registration: Option<TemporaryRegistration>,
}

/// Doimiy ro'yxatdan o'tish ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct PermanentRegistration {
    /// MFY (mahalla fuqarolar yig'ini) nomi. Mavjud bo'lmasa `null`.
    pub mfy: Option<String>,
    /// MFY identifikatori. Mavjud bo'lmasa `null`.
    pub mfy_id: Option<String>,
    /// Viloyat nomi.
    pub region: String,
    /// To'liq manzil matni. Mavjud bo'lmasa `null`.
    pub address: Option<String>,
    /// Davlat nomi.
    pub country: String,
    /// Kadastr raqami. Mavjud bo'lmasa `null`.
    pub cadastre: Option<String>,
    /// Tuman nomi. Mavjud bo'lmasa `null`.
    pub district: Option<String>,
    /// Viloyat identifikatori.
    pub region_id: String,
    /// Davlat identifikatori.
    pub country_id: String,
    /// Tuman identifikatori.
    pub district_id: String,
    /// Viloyatning CBU (Markaziy bank) identifikatori.
    pub region_id_cbu: String,
    /// Davlatning CBU identifikatori.
    pub country_id_cbu: String,
    /// Tumanining CBU identifikatori.
    pub district_id_cbu: String,
    /// Ro'yxatdan o'tish sanasi.
    pub registration_date: String,
}

/// Vaqtinchalik ro'yxatdan o'tish ma'lumotlari.
#[derive(Debug, Deserialize)]
pub struct TemporaryRegistration {
    /// MFY nomi. Mavjud bo'lmasa `null`.
    pub mfy: Option<String>,
    /// MFY identifikatori. Mavjud bo'lmasa `null`.
    pub mfy_id: Option<String>,
    /// Viloyat nomi.
    pub region: String,
    /// To'liq manzil matni. Mavjud bo'lmasa `null`.
    pub address: Option<String>,
    /// Kadastr raqami.
    pub cadastre: String,
    /// Tuman nomi. Mavjud bo'lmasa `null`.
    pub district: Option<String>,
    /// Vaqtinchalik ro'yxatdan o'tish boshlanish sanasi. Mavjud bo'lmasa `null`.
    pub date_from: Option<String>,
    /// Vaqtinchalik ro'yxatdan o'tish tugash sanasi.
    pub date_till: String,
    /// Viloyat identifikatori.
    pub region_id: String,
    /// Tuman identifikatori.
    pub district_id: String,
    /// Viloyatning CBU identifikatori.
    pub region_id_cbu: String,
    /// Tumanining CBU identifikatori.
    pub district_id_cbu: String,
}

/// Secondary flow uchun qayta ishlatiladigan identifikator va uning muddati.
///
/// **Nomlash farqi:**
/// - [`ReuId`] — bu struct (metadata: `expires_at` + `value`)
/// - [`Reuid`](crate::types::Reuid) — ichidagi UUID qiymat turi
///
/// Primary flow muvaffaqiyatli tugagandan so'ng `UserDataResponse.reuid` da qaytadi.
/// `value` ni keyingi [`CreateSessionRequest::WithReuid`](crate::dto::CreateSessionRequest)
/// so'rovida ishlating.
#[derive(Debug, Deserialize)]
pub struct ReuId {
    /// Amal qilish muddati (Unix timestamp, soniyalarda). Masalan: `1722412345`.
    ///
    /// Shartnomaga qarab oyning oxirgi kuni yoki yil oxirida tugaydi.
    pub expires_at: i64,
    /// Qayta ishlatiladigan UUID qiymati. Masalan: `"9b7e597e-893e-4e11-92cf-f4e7d4f923b1"`.
    pub value: Reuid,
}

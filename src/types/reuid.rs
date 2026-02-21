use crate::types::uuid_type::define_uuid_type;

define_uuid_type! {
    /// Secondary flow uchun qayta ishlatiladigan foydalanuvchi identifikatori (UUID v4).
    ///
    /// Primary flow muvaffaqiyatli tugagandan so'ng server tomonidan beriladi.
    /// Keyingi secondary flow da pasport ma'lumotlari kerak bo'lmaydi —
    /// shu `Reuid` orqali foydalanuvchi qayta tekshiriladi.
    ///
    /// ```
    /// use myid::types::Reuid;
    ///
    /// let id = Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    /// let new_id = Reuid::generate();
    /// ```
    pub struct Reuid;
    error_field = "reuid";
}

#[cfg(test)]
mod tests {
    use super::Reuid;
    use crate::types::uuid_type::uuid_type_tests;
    uuid_type_tests!(Reuid);
}

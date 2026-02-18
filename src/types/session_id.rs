use crate::types::uuid_type::define_uuid_type;

define_uuid_type! {
    /// MyID session identifikatori (UUID v4).
    ///
    /// Har bir face verification sessiyasiga unikal ID beriladi.
    ///
    /// ```
    /// use myid::types::SessionId;
    ///
    /// let id = SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    /// let new_id = SessionId::generate();
    /// ```
    pub struct SessionId;
    error_field = "session_id";
}

#[cfg(test)]
mod tests {
    use super::SessionId;
    use crate::types::uuid_type::uuid_type_tests;
    uuid_type_tests!(SessionId);
}

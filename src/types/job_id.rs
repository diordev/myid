// src/types/job_id.rs

use crate::types::uuid_type::define_uuid_type;

define_uuid_type! {
    /// MyID job identifikatori.
    ///
    /// Asinxron verifikatsiya jarayonini kuzatish uchun ishlatiladi.
    ///
    /// ```
    /// use myid::types::JobId;
    ///
    /// let id = JobId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    /// let new_id = JobId::generate();
    /// ```
    pub struct JobId;
    error_field = "job_id";
}

#[cfg(test)]
mod tests {
    use super::JobId;
    use crate::types::uuid_type::uuid_type_tests;
    uuid_type_tests!(JobId);
}

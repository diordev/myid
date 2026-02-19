//! MyID SDK — Client ishlatish misollari
//!
//! Ishga tushirish:
//! ```bash
//! # .env faylda MYID_BASE_URL, MYID_CLIENT_ID, MYID_CLIENT_SECRET bo'lishi kerak
//! cargo run --example client
//! ```

use myid::error::MyIdError;
use myid::prelude::*;
use myid::types::BirthDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match Config::from_env(None) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config yuklanmadi: {e}");
            eprintln!("Kerakli env varlar: MYID_BASE_URL, MYID_CLIENT_ID, MYID_CLIENT_SECRET");
            return Ok(());
        }
    };

    let client = MyIdClient::new(config)?;

    session_with_passport(&client).await;
    session_with_pinfl(&client).await;
    session_with_reuid(&client).await;
    session_empty(&client).await;

    Ok(())
}

/// Passport orqali session yaratish
async fn session_with_passport(client: &MyIdClient) {
    println!("=== Passport orqali ===");

    let request = CreateSessionRequest::WithPassport(
        SessionWithPassport::new(
            PassportData::parse("AB1234567").expect("to'g'ri passport"),
            BirthDate::parse("1996-09-01").expect("to'g'ri sana"),
        )
        .with_phone_number(PhoneNumber::parse("+998901234567").expect("to'g'ri raqam"))
        .with_threshold(Threshold::parse(0.75).expect("to'g'ri threshold")),
    );

    print_result(client.create_session(&request).await);
}

/// PINFL orqali session yaratish
async fn session_with_pinfl(client: &MyIdClient) {
    println!("=== PINFL orqali ===");

    let request = CreateSessionRequest::WithPinfl(
        SessionWithPinfl::new(
            Pinfl::parse("12345678901234").expect("to'g'ri PINFL"),
            BirthDate::parse("1990-05-15").expect("to'g'ri sana"),
        )
        .with_is_resident(true),
    );

    print_result(client.create_session(&request).await);
}

/// REUID orqali session yaratish (secondary flow)
async fn session_with_reuid(client: &MyIdClient) {
    println!("=== REUID orqali ===");

    let request = CreateSessionRequest::WithReuid(
        SessionWithReuid::new(Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").expect("to'g'ri REUID")),
    );

    print_result(client.create_session(&request).await);
}

/// Bo'sh session — foydalanuvchi o'zi kiritadi
async fn session_empty(client: &MyIdClient) {
    println!("=== Bo'sh session ===");
    print_result(client.create_session(&CreateSessionRequest::Empty {}).await);
}

fn print_result(result: MyIdResult<SessionResponse>) {
    match result {
        Ok(resp) => println!("Session ID: {}\n", resp.session_id()),
        Err(MyIdError::Api { status, message }) => {
            eprintln!("API xato {status}: {message}\n")
        }
        Err(MyIdError::Http(e)) => eprintln!("Tarmoq xatosi: {e}\n"),
        Err(e) => eprintln!("Xato: {e}\n"),
    }
}

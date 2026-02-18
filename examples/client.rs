//! MyID SDK — Client ishlatish misollari
//!
//! Ishga tushirish:
//! ```bash
//! # .env faylda MYID_BASE_URL, MYID_CLIENT_ID, MYID_CLIENT_SECRET bo'lishi kerak
//! cargo run --example client
//! ```

use myid::prelude::*;
use myid::types::BirthDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env(None)?;
    let client = MyIdClient::new(config)?;

    session_with_passport(&client).await?;
    session_with_pinfl(&client).await?;
    session_empty(&client).await?;

    Ok(())
}

/// 1) Passport orqali session yaratish
async fn session_with_passport(client: &MyIdClient) -> MyIdResult<()> {
    println!("=== Passport orqali ===\n");

    let request = CreateSessionRequest::WithPassport(
        SessionWithPassport::new(
            PassportData::parse("AB1234567")?,
            BirthDate::parse("1996-09-01")?,
        )
        .with_phone_number(PhoneNumber::parse("+998901234567")?)
        .with_threshold(Threshold::parse(0.75)?),
    );

    match client.create_session(&request).await {
        Ok(resp) => println!("Session ID: {}\n", resp.session_id()),
        Err(e) => println!("Xato: {e}\n"),
    }

    Ok(())
}

/// 2) PINFL orqali session yaratish
async fn session_with_pinfl(client: &MyIdClient) -> MyIdResult<()> {
    println!("=== PINFL orqali ===\n");

    let request = CreateSessionRequest::WithPinfl(
        SessionWithPinfl::new(
            Pinfl::parse("12345678901234")?,
            BirthDate::parse("1990-05-15")?,
        )
        .with_is_resident(true),
    );

    match client.create_session(&request).await {
        Ok(resp) => println!("Session ID: {}\n", resp.session_id()),
        Err(e) => println!("Xato: {e}\n"),
    }

    Ok(())
}

// /// 3) REUID orqali session yaratish (secondary flow)
// async fn session_with_reuid(client: &MyIdClient) -> MyIdResult<()> {
//     println!("=== REUID orqali ===\n");

//     let request = CreateSessionRequest::WithReuid(SessionWithReuid::new(Reuid::parse(
//         "9b7e597e-893e-4e11-92cf-f4e7d4f923b1",
//     )?));

//     match client.create_session(&request).await {
//         Ok(resp) => println!("Session ID: {}\n", resp.session_id()),
//         Err(e) => println!("Xato: {e}\n"),
//     }

//     Ok(())
// }

/// 4) Bo'sh session — foydalanuvchi o'zi kiritadi
async fn session_empty(client: &MyIdClient) -> MyIdResult<()> {
    println!("=== Bo'sh session ===\n");

    let request = CreateSessionRequest::Empty {};

    match client.create_session(&request).await {
        Ok(resp) => println!("Session ID: {}\n", resp.session_id()),
        Err(e) => println!("Xato: {e}\n"),
    }

    Ok(())
}

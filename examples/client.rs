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
    recover_session_example(&client).await;
    handle_callback_example(&client).await;

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

/// Mavjud sessionni session_id orqali tiklash
async fn recover_session_example(client: &MyIdClient) {
    println!("=== Session tiklash ===");

    let session_id = match SessionId::parse("550e8400-e29b-41d4-a716-446655440000") {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Noto'g'ri session_id: {e}\n");
            return;
        }
    };

    match client.recover_session(session_id).await {
        Ok(resp) => {
            println!("Status: {:?}", resp.status());
            if let Some(code) = resp.code() {
                println!("Code: {code}");
            }
            for attempt in resp.attempts() {
                println!(
                    "  Urinish: job_id={}, vaqt={}",
                    attempt.job_id(),
                    attempt.timestamp()
                );
                if let Some(reason) = attempt.reason() {
                    println!("Sabab: {reason}");
                }
            }
            println!();
        }
        Err(MyIdError::Api { status, message }) => {
            eprintln!("API xato {status}: {message}\n")
        }
        Err(MyIdError::Http(e)) => eprintln!("Tarmoq xatosi: {e}\n"),
        Err(e) => eprintln!("Xato: {e}\n"),
    }
}

/// MyID callback code orqali foydalanuvchi ma'lumotlarini olish.
///
/// Primary flow da `profile` to'liq keladi.
/// Secondary flow da `profile` `None` — faqat `reuid` qaytadi.
async fn handle_callback_example(client: &MyIdClient) {
    println!("=== Callback qayta ishlash (primary) ===");

    // code — mobil ilova MyID dan olib backendga yuborgan bir martalik UUID (TTL: 5 daqiqa)
    let code = "9b7e597e-893e-4e11-92cf-f4e7d4f923b1".to_string();

    match client.handle_callback(code).await {
        Ok(resp) => {
            match &resp.data.profile {
                // Primary flow: to'liq profil mavjud
                Some(profile) => {
                    let common = &profile.common_data;
                    println!(
                        "Foydalanuvchi: {} {} (PINFL: {})",
                        common.first_name, common.last_name, common.pinfl
                    );
                }
                // Secondary flow: profil yo'q, reuid ham null
                None => {
                    println!("Secondary flow — profil yo'q.");
                    if let Some(reuid) = &resp.reuid {
                        println!("ReUID: {}", reuid.value);
                    }
                }
            }
            println!();
        }
        Err(MyIdError::Api { status, message }) => {
            eprintln!("API xato {status}: {message}\n")
        }
        Err(MyIdError::Http(e)) => eprintln!("Tarmoq xatosi: {e}\n"),
        Err(e) => eprintln!("Xato: {e}\n"),
    }
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

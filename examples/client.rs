use myid::client::MyIdClient;
use myid::config::Config;
use myid::dto::{SessionWithPassport, CreateSessionRequest};
use myid::types::{BirthDate, PassportData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env(None).unwrap();

    let client = MyIdClient::new(config).unwrap();
    
    let pass_data = SessionWithPassport::new(
        PassportData::parse("AD1623289")?,
        BirthDate::parse("1996-09-01")?,
    );
    
    let data = CreateSessionRequest::WithPassport(pass_data);
    

    let session_id = client.create_session(&data).await?;

    println!("Session ID: {:?}", session_id);
    Ok(())
}

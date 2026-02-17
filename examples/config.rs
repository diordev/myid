use myid::config::Config;

fn main() {
    let config = Config::new("https://api.example.com", "client_id", "client_secret");
    println!("{:#?}", config);
}

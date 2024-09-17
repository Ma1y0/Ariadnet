use apollo::aethon;

#[tokio::main]
async fn main() {
    let c = a().unwrap();
    println!("Hello World");
}

fn a() -> Result<u32, aethon::Error> {
    Err(aethon::Error::WrongMethod)
}

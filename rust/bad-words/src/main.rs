use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("BAD_WORDS_API_KEY").expect("No environment variable");
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body("a list with shit words")
        .send()
        .await?
        .text()
        .await?;
    println!("{}", res);
    Ok(())
}

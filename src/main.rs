#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // get user input per (https://users.rust-lang.org/t/how-to-get-user-input/5176/3)
    use std::io::{stdin,stdout,Write};
    let mut permit_id = String::new();
    print!("Please Enter rec.gov Permit ID ");
    let _=stdout().flush();
    stdin().read_line(&mut permit_id).expect("Did not enter a correct string");
    if let Some('\n')=permit_id.chars().next_back() {
        permit_id.pop();
    }
    if let Some('\r')=permit_id.chars().next_back() {
        permit_id.pop();
    }
    println!("River ID Selected: {}", permit_id);

    // do HTML GET request stuff
    //let res = reqwest::get("https://hyper.rs").await?;
    let mut permit_url = "https://www.recreation.gov/permits/".to_string();
    permit_url.push_str(&permit_id);
    let res = reqwest::get(&permit_url).await?;
    let res = reqwest::get("https://www.recreation.gov/api/permits/250986/divisions/704/availability?start_date=2020-06-12T06:00:00.000Z&end_date=2020-12-31T00:00:00.000Z&commercial_acct=false&is_lottery=false").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}

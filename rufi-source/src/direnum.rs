use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

#[tokio::main]
pub async fn dirs(domain: &str, s: &str) -> Result<reqwest::Response, Box<dyn Error>> {
    let requrl = String::from(domain);
    let res = reqwest::get(&requrl).await?;

    let file = File::open(s)?;
    let reader = BufReader::new(file);
    let mut cnt  = 0;
    
    for line in reader.lines() {
        cnt = cnt + 1;
        let urls = requrl.trim_end_matches(&['\n', '\r'][..]).to_owned() + "/" + &mut line?.to_string();
        let res = reqwest::get(&urls).await?;
        // This wall of if statements, could easily be done
        // by using the last if statement alone, but then
        // we couldn't use cool ANSI colors ;)
        // There's probably a better way of doing this, but this is the only way I know how
        if res.status().to_string() == "200 OK" {
            println!("\x1B[36mResponse: \x1B[32m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() == "403 Forbidden" {
            println!("\x1B[36mResponse: \x1B[31m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() == "301 Moved Permanently" {
            println!("\x1B[36mResponse: \x1B[33m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() == "302 Found" {
            println!("\x1B[36mResponse: \x1B[33m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() == "500 Internal Server Error" {
            println!("\x1B[36mResponse: \x1B[31m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() == "503 Service Unavailable" {
            println!("\x1B[36mResponse: \x1B[31m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        } else if res.status().to_string() != "404 Not Found" {
            println!("\x1B[36mResponse: \x1B[31m[{:#?}]\x1B[37m | \x1B[35m{}\x1B[0m", res.status().to_string(), urls);
        }
    }
    
    println!("\x1B[36mFinished!");
    Ok(res)
}
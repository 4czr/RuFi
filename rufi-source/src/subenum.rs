use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use std::net::ToSocketAddrs;

#[tokio::main]
pub async fn sub(domain: &str, w: &str) -> Result<reqwest::Response, Box<dyn Error>> {
    let newdom = domain.replace("https://", "").replace("http://", "");
    
    let requrl = String::from(domain);
    let res = reqwest::get(&requrl).await?;

    let file = File::open(w)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let urls = "https://".to_owned() + &mut line?.to_string() + "." + &mut newdom.trim_end_matches(&['\n', '\r'][..]);
        let server = (urls.replace("https://", "").replace("http://", ""), 80).to_socket_addrs().ok();
        let svrfmt: String = format!("{:?}", server);
        
        match server {
            None => (),
            _ => {
                println!("\x1B[32m{} \n\t\x1B[34m{}\x1B[0m", urls, svrfmt.replace("Some(IntoIter(","").replace("[","").replace("]","").replace(":80","").replace("))",""));
            }
        }
    }
    
    Ok(res)
}
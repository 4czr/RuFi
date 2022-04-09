use std::error::Error;

#[tokio::main]
pub async fn geo(geo: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    
    let mut link: String = "http://ip-api.com/json/".to_owned();
    let ipborrowed: &str = geo;
    link.push_str(ipborrowed);

    let requrl = String::from(link);
    let res = reqwest::get(&requrl).await?;
    let geol: serde_json::Value = res.json().await?;

    println!("\n\x1B[31mIP:\x1B[37m {}", geol["query"].to_string().replace('"',""));
    println!("\x1B[32mLocation:\x1B[37m {}, {}, {}, {}", geol["city"].to_string().replace('"',""), geol["regionName"].to_string().replace('"',""), geol["countryCode"].to_string().replace('"',""), geol["zip"].to_string().replace('"',""));
    println!("\x1B[32mISP:\x1B[37m {}", geol["isp"].to_string().replace('"',""));
    println!("\x1B[32mORG:\x1B[37m {}", geol["org"].to_string().replace('"',""));
    println!("\x1B[32mASN:\x1B[37m {}", geol["as"].to_string().replace('"',""));
    println!("\x1B[32mLat, Long:\x1B[37m {}, {}", geol["lat"], geol["lon"].to_string().replace('"',""));

    
    Ok(geol)
}
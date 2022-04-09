use clap::{Arg, Command};
extern crate base64;
mod geo;
mod direnum;
mod subenum;
mod dns;

fn main() {
    let matches = Command::new("*----------------------------------------*\n|         Welcome to RustFinder!         |\n|   A fast and handy little recon tool   |\n|----------------------------------------|\n|              Version: 0.1              |\n|----------------------------------------|\n|    Developed by Ozz | ozz@riseup.net   |\n|      Twitter: @_Ozz | GitHub: @4czr    |\n*----------------------------------------*\n\n")
        .arg(Arg::new("GeoIP")
                .short('g')
                .long("geo")
                .takes_value(true)
                .help("GeoLocation for IPv4"))
        .arg(Arg::new("DNS")
                .short('r')
                .long("dns")
                .takes_value(true)
                .help("DNS Resolver"))
        .arg(Arg::new("SubEnum")
                .short('s')
                .long("subs")
                .takes_value(true)
                .help("Subdomain Enumuration [requires -w flag]"))
        .arg(Arg::new("DirEnum")
                .short('d')
                .long("dirs")
                .takes_value(true)
                .help("Directory Enumuration [requires -w flag]"))
        .arg(Arg::new("wordlist")
                .short('w')
                .long("words")
                .takes_value(true)
                .help("Wordlist input, use full path\nExample: -w /home/youruser/dir.txt"))
        .get_matches();

    println!("*----------------------------------------*\n|         Welcome to RustFinder!         |\n|   A fast and handy little recon tool   |\n|----------------------------------------|\n|              Version: 0.1              |\n|----------------------------------------|\n|    Developed by Ozz | ozz@riseup.net   |\n|      Twitter: @_Ozz | GitHub: @4czr    |\n*----------------------------------------*");
    

    match matches.value_of("GeoIP") { 
        Some(s) => { geo::geo(s).expect("Unexpected Error:"); }, 
        None => {},
    }

    match matches.value_of("SubEnum") { 
        Some(s) => { 
            match matches.value_of("wordlist") {
                Some(w) => { subenum::sub(s, w).expect("Unexpected Error: "); }
                None => {},
        }
    }, 
        None => {},
    }

    match matches.value_of("DirEnum") { 
        Some(s) => { 
            match matches.value_of("wordlist") {
                Some(w) => { direnum::dirs(s, w).expect("Unexpected Error: "); }
                None => {},
        }
    }, 
        None => {},
    }

    match matches.value_of("DNS") { 
        Some(s) => dns::dns(s), 
        None => {}, 
    }
}
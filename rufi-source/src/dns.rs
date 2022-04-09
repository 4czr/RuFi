use trust_dns_resolver::{config::*, lookup::SoaLookup, lookup::MxLookup, lookup::TxtLookup};
use trust_dns_resolver::Resolver;
use trust_dns_resolver::error::ResolveResult;

// Big Thanks To: https://dev.to/basman/series/11934

pub fn dns(domain: &str) {
    let newdom: &str = &domain.replace("https://","").replace("http://","").replace("/","").replace("www.","").replace("\n","").to_string();
    
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mx_response = resolver.mx_lookup(newdom);
    let soa_response = resolver.soa_lookup(newdom);
    let txt_response = resolver.txt_lookup(newdom);

    display_mx(&mx_response);
    display_soa(&soa_response);
    display_txt(&txt_response);
}

fn display_mx(mx_response: &ResolveResult<MxLookup>){
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    match mx_response {
        Err(_) => println!("Error occured"),
        Ok(mx_response) => {
            println!("\n\x1B[33m---- MX RECORDS ----\x1B[0m");
            let records = mx_response.iter();
            for record in records {
                let lookup_response = resolver.lookup_ip(record.exchange().to_string().as_str());
                match lookup_response {
                    Err(_) => println!("\n\x1B[31mThis exchange host has no address."),
                    Ok(lookup_response) => {
                        let ip_addrs = lookup_response.iter();
                        for ip_addr in ip_addrs {
                            if ip_addr.is_ipv4() {
                                println!("\x1B[32mIPv4:\x1B[0m \t{}", ip_addr);
                            } 
                            
                            if ip_addr.is_ipv6(){
                                println!("\x1B[32mIPv6:\x1B[0m \t{}", ip_addr)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn display_soa(soa_response: &ResolveResult<SoaLookup>) {
    match soa_response {
        Err(_) => println!("Error occured"),
        Ok(soa_response) => {
            println!("\n\x1B[33m---- SOA RECORDS ----\x1B[0m");
            let soa_iter = soa_response.iter();
            for record in soa_iter {
                println!("\x1B[32mNS1:\x1B[0m {}", record.mname());
                println!("\x1B[32mAdmin:\x1B[0m {}", record.rname());
                let email = convert_rname_to_email_address(&record.rname().to_string());
                println!("\x1B[32mAdmin Email:\x1B[0m {}", email);
                println!("\x1B[32mSerial:\x1B[0m {}", record.serial());
            }
        }
    }
}

fn convert_rname_to_email_address(rname: &String) -> String {
    let rname = rname.clone();
    let mut email_address: String = String::new();
    let mut splitter = rname.splitn(2, ".");
    email_address.push_str(splitter.next().unwrap());
    email_address.push_str("@");
    email_address.push_str(splitter.next().unwrap());
    email_address.pop();
    email_address
}

fn display_txt(txt_response: &ResolveResult<TxtLookup>) {
    match txt_response {
        Err(_) => println!("Error occured"),
        Ok(txt_response) => {
            println!("\n\x1B[33m---- TXT RECORDS ----\x1B[0m");
            let mut i = 1;
            for record in txt_response.iter() {
                println!("\x1B[32m[{}]\x1B[0m \t{}", i, record.to_string());
                i = i + 1;
            }
        }
    }
}
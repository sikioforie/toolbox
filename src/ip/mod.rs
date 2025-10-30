
use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
/*


## Providers

| Provider | URL | Rate Limit | API Key | Target Lookup |
| --- | --- | --- | --- | --- |
| FreeIpApi | [https://freeipapi.com](https://freeipapi.com) | 60 / minute | ✔️ | ✔️ |
| IfConfig | [https://ifconfig.co](https://ifconfig.co) | 1 / minute |  | ✔️ |
| IpInfo | [https://ipinfo.io](https://ipinfo.io) | 50000 / month | ✔️ | ✔️ |
| MyIp | [https://my-ip.io](https://my-ip.io) | ? / day | ️ | ️ |
| IpApiCom | [https://ip-api.com](https://ip-api.com) | 45 / minute |  | ✔️ |
| IpWhoIs | [https://ipwhois.io](https://ipwhois.io) | 10000 / month | ️ | ✔️ |
| IpApiCo | [https://ipapi.co](https://ipapi.co) | 30000 / month |  | ✔️ |
| IpApiIo | [https://ip-api.io](https://ip-api.io) | ? / day | ✔️ | ✔️ |
| IpBase | [https://ipbase.com](https://ipbase.com) | 10 / hour | ✔️ | ✔️ |
| IpLocateIo | [https://iplocate.io](https://iplocate.io) | 50 / day | ✔️ | ✔️ |
| IpLeak | [https://ipleak.net](https://ipleak.net) | ? / day | ️ | ✔️ |
| Mullvad | [https://mullvad.net](https://mullvad.net) | ? / day | ️ | ️ |
| AbstractApi | [https://abstractapi.com](https://abstractapi.com) | 1000 / day | ✔️ | ✔️ |
| IpGeolocation | [https://ipgeolocation.io](https://ipgeolocation.io) | 1000 / day | ✔️ | ✔️ |
| IpData | [https://ipdata.co](https://ipdata.co) | 1500 / day | ✔️ | ✔️ |
| Ip2Location | [https://ip2location.io](https://ip2location.io) | 50000 / month | ✔️ | ✔️ |
| MyIpCom | [https://myip.com](https://myip.com) | unlimited | ️ | ️ |
| GetJsonIp | [https://getjsonip.com](https://getjsonip.com) | unlimited | ️ | ️ |
| Ipify | [https://www.ipify.org](https://www.ipify.org) | unlimited | ️ | ️ |
| IpQuery | [https://ipquery.io](https://ipquery.io) | unlimited |  | ✔️ |



*/

/// Get public address
fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    let providers = vec!["google.com", "checkip.amazonaws.com", "api.ipify.org"];

    for provider in providers {
        let address = format!("{provider}:80");
        println!("PROVIDER => {provider} \nADDRESS => {address}");

        let mut stream = TcpStream::connect(&address)?;
    
        // Manual HTTP request
        let request = format!("GET / HTTP/1.1\r\nHost: {provider}\r\nConnection: close\r\n\r\n");
        stream.write_all(request.as_bytes())?;
    
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
    
        // Extract IP from response
        let raw_public_ip = response
            .lines()
            .last()
            .map(|ip| ip.trim().to_string());

        return match raw_public_ip{
            Some(ip) => {
                println!("PUBLIC IP => {ip}");
                // TODO: Validate ip address
                Ok(ip)
            },
            None => continue
        }
    }

    Err("Failed to find public ip".into())
}


#[test]
fn test_get_public_address() {
    let x = get_public_ip();
    assert!(x.is_ok());
} 

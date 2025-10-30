
// Enhanced version that returns detailed validation results
#[derive(Debug, PartialEq)]
pub enum IpValidationResult {
    ValidPublicIp(IpAddr),
    ValidPrivateIp(IpAddr),
    InvalidFormat,
    ReservedAddress,
    Loopback,
    Multicast,
    Unspecified,
}

pub fn validate_ip_detailed(ip_str: &str) -> IpValidationResult {
    let ip_str = ip_str.trim();
    
    if ip_str.is_empty() {
        return IpValidationResult::InvalidFormat;
    }
    
    // Try to parse as IP address
    match ip_str.parse::<IpAddr>() {
        Ok(ip_addr) => {
            match ip_addr {
                IpAddr::V4(ipv4) => validate_ipv4_detailed(ipv4),
                IpAddr::V6(ipv6) => validate_ipv6_detailed(ipv6),
            }
        }
        Err(_) => IpValidationResult::InvalidFormat,
    }
}

fn validate_ipv4_detailed(ip: Ipv4Addr) -> IpValidationResult {
    let octets = ip.octets();
    
    if ip.is_unspecified() {
        return IpValidationResult::Unspecified;
    }
    
    if ip.is_loopback() {
        return IpValidationResult::Loopback;
    }
    
    if ip.is_multicast() {
        return IpValidationResult::Multicast;
    }
    
    if ip.is_broadcast() {
        return IpValidationResult::ReservedAddress;
    }
    
    // Check for private ranges
    if octets[0] == 10
        || (octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31))
        || (octets[0] == 192 && octets[1] == 168)
        || (octets[0] == 169 && octets[1] == 254) // APIPA
    {
        return IpValidationResult::ValidPrivateIp(IpAddr::V4(ip));
    }
    
    // Check for other reserved ranges
    if (octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
        || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
        || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113)
        || (octets[0] >= 224 && octets[0] <= 255)
    {
        return IpValidationResult::ReservedAddress;
    }
    
    IpValidationResult::ValidPublicIp(IpAddr::V4(ip))
}

fn validate_ipv6_detailed(ip: Ipv6Addr) -> IpValidationResult {
    if ip.is_unspecified() {
        return IpValidationResult::Unspecified;
    }
    
    if ip.is_loopback() {
        return IpValidationResult::Loopback;
    }
    
    let segments = ip.segments();
    
    // Check for unique local addresses (FC00::/7)
    if (segments[0] & 0xFE00) == 0xFC00 {
        return IpValidationResult::ValidPrivateIp(IpAddr::V6(ip));
    }
    
    // Check for link-local addresses (FE80::/10)
    if (segments[0] & 0xFFC0) == 0xFE80 {
        return IpValidationResult::ValidPrivateIp(IpAddr::V6(ip));
    }
    
    // Check for multicast (FF00::/8)
    if (segments[0] & 0xFF00) == 0xFF00 {
        return IpValidationResult::Multicast;
    }
    
    // Check for documentation addresses (2001:DB8::/32)
    if segments[0] == 0x2001 && segments[1] == 0x0DB8 {
        return IpValidationResult::ReservedAddress;
    }
    
    // Handle IPv4-mapped and IPv4-compatible
    if let Some(ipv4) = ip.to_ipv4_mapped() {
        return validate_ipv4_detailed(ipv4);
    }
    
    if let Some(ipv4) = ip.to_ipv4() {
        return validate_ipv4_detailed(ipv4);
    }
    
    IpValidationResult::ValidPublicIp(IpAddr::V6(ip))
}


fn validate_ip_address(ip_str: &str) -> bool {
    // Trim any whitespace that might have been in the response
    let ip_str = ip_str.trim();
    
    if ip_str.is_empty() {
        return false;
    }
    
    // Try parsing as IPv4 first (more common for public IPs)
    if let Ok(ipv4) = ip_str.parse::<Ipv4Addr>() {
        return is_valid_public_ipv4(ipv4);
    }
    
    // Try parsing as IPv6
    if let Ok(ipv6) = ip_str.parse::<Ipv6Addr>() {
        return is_valid_public_ipv6(ipv6);
    }
    
    false
}

fn is_valid_public_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    
    // Check for invalid IP ranges
    if ip.is_unspecified() || ip.is_broadcast() {
        return false;
    }
    
    // Check for private ranges (RFC 1918)
    if octets[0] == 10 // 10.0.0.0/8
        || (octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31)) // 172.16.0.0/12
        || (octets[0] == 192 && octets[1] == 168) // 192.168.0.0/16
    {
        return false;
    }
    
    // Check for link-local (APIPA)
    if octets[0] == 169 && octets[1] == 254 {
        return false;
    }
    
    // Check for loopback
    if ip.is_loopback() {
        return false;
    }
    
    // Check for multicast
    if ip.is_multicast() {
        return false;
    }
    
    // Check for reserved/documentation ranges
    if (octets[0] == 192 && octets[1] == 0 && octets[2] == 2) // 192.0.2.0/24 (TEST-NET-1)
        || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100) // 198.51.100.0/24 (TEST-NET-2)
        || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113) // 203.0.113.0/24 (TEST-NET-3)
        || (octets[0] >= 224 && octets[0] <= 239) // Multicast (should already be caught but double-check)
        || (octets[0] >= 240 && octets[0] <= 255) // Reserved for future use
    {
        return false;
    }
    
    true
}

fn is_valid_public_ipv6(ip: Ipv6Addr) -> bool {
    // Check for invalid IPs
    if ip.is_unspecified() {
        return false;
    }
    
    // Check for loopback
    if ip.is_loopback() {
        return false;
    }
    
    // Check for IPv4-mapped IPv6 addresses
    if let Some(ipv4) = ip.to_ipv4_mapped() {
        return is_valid_public_ipv4(ipv4);
    }
    
    // Check for IPv4-compatible (deprecated but still possible)
    if let Some(ipv4) = ip.to_ipv4() {
        return is_valid_public_ipv4(ipv4);
    }
    
    // Check for unique local addresses (FC00::/7)
    let segments = ip.segments();
    if (segments[0] & 0xFE00) == 0xFC00 {
        return false;
    }
    
    // Check for link-local addresses (FE80::/10)
    if (segments[0] & 0xFFC0) == 0xFE80 {
        return false;
    }
    
    // Check for multicast (FF00::/8)
    if (segments[0] & 0xFF00) == 0xFF00 {
        return false;
    }
    
    // Check for documentation addresses (2001:DB8::/32)
    if segments[0] == 0x2001 && segments[1] == 0x0DB8 {
        return false;
    }
    
    true
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_validate_ip_address() {
        // Valid public IPs
        assert!(validate_ip_address("8.8.8.8"));
        assert!(validate_ip_address("1.1.1.1"));
        assert!(validate_ip_address("142.251.16.100")); // google.com
        
        // Private IPs
        assert!(!validate_ip_address("192.168.1.1"));
        assert!(!validate_ip_address("10.0.0.1"));
        assert!(!validate_ip_address("172.16.0.1"));
        
        // Reserved IPs
        assert!(!validate_ip_address("127.0.0.1"));
        assert!(!validate_ip_address("0.0.0.0"));
        assert!(!validate_ip_address("255.255.255.255"));
        assert!(!validate_ip_address("224.0.0.1"));
        assert!(!validate_ip_address("169.254.0.1"));
        
        // Invalid formats
        assert!(!validate_ip_address("not.an.ip"));
        assert!(!validate_ip_address("256.256.256.256"));
        assert!(!validate_ip_address(""));
        assert!(!validate_ip_address("  "));
    }

    #[test]
    fn test_detailed_validation() {
        assert_eq!(
            validate_ip_detailed("8.8.8.8"),
            IpValidationResult::ValidPublicIp("8.8.8.8".parse().unwrap())
        );
        
        assert_eq!(
            validate_ip_detailed("192.168.1.1"),
            IpValidationResult::ValidPrivateIp("192.168.1.1".parse().unwrap())
        );
        
        assert_eq!(
            validate_ip_detailed("127.0.0.1"),
            IpValidationResult::Loopback
        );
        
        assert_eq!(
            validate_ip_detailed("invalid"),
            IpValidationResult::InvalidFormat
        );
    }
}


use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

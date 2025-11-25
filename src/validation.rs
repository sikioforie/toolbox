pub fn is_means_of_auth(value: &str) -> bool {
    is_email(value) || is_phone(value)
}

/// Check if a string is a valid phone number
pub fn is_phone(value: &str) -> bool {
    // Normalize input: trim and remove excessive whitespace
    let normalized = value.trim().to_lowercase();
    if normalized.is_empty() {
        return false;
    }

    let phone_re =
        regex::Regex::new(r"^(?:\+?(\d{1,3}))?[-. (]*(\d{1,4})[-. )]*(\d{1,4})[-. ]*(\d{1,9})$")
            .unwrap();

    // Clean phone number for validation: remove all non-digit characters except leading +
    let phone_clean = normalized
        .chars()
        .enumerate()
        .filter(|(i, c)| *c == '+' && *i == 0 || c.is_ascii_digit())
        .map(|(_, c)| c)
        .collect::<String>();

    // Phone number validation with comprehensive checks
    if phone_re.is_match(&normalized) {
        let digit_count = phone_clean.chars().filter(|c| c.is_ascii_digit()).count();

        // Validate phone number length (international standards)
        if (7..=15).contains(&digit_count) {
            // Additional validation: country code if present should be valid
            if phone_clean.starts_with('+') {
                let country_code = phone_clean.chars().skip(1).take(3).collect::<String>();
                let valid_country_codes = [
                    "1", "7", "20", "27", "30", "31", "32", "33", "34", "36", "39", "40", "41",
                    "43", "44", "45", "46", "47", "48", "49", "51", "52", "53", "54", "55", "56",
                    "57", "58", "60", "61", "62", "63", "64", "65", "66", "81", "82", "84", "86",
                    "90", "91", "92", "93", "94", "95", "98", "211", "212", "213", "216", "218",
                    "220", "221", "222", "223", "224", "225", "226", "227", "228", "229", "230",
                    "231", "232", "233", "234", "235", "236", "237", "238", "239", "240", "241",
                    "242", "243", "244", "245", "246", "247", "248", "249", "250", "251", "252",
                    "253", "254", "255", "256", "257", "258", "260", "261", "262", "263", "264",
                    "265", "266", "267", "268", "269", "290", "291", "297", "298", "299", "350",
                    "351", "352", "353", "354", "355", "356", "357", "358", "359", "370", "371",
                    "372", "373", "374", "375", "376", "377", "378", "379", "380", "381", "382",
                    "383", "385", "386", "387", "389", "420", "421", "423", "500", "501", "502",
                    "503", "504", "505", "506", "507", "508", "509", "590", "591", "592", "593",
                    "594", "595", "596", "597", "598", "599", "670", "672", "673", "674", "675",
                    "676", "677", "678", "679", "680", "681", "682", "683", "685", "686", "687",
                    "688", "689", "690", "691", "692", "850", "852", "853", "855", "856", "880",
                    "886", "960", "961", "962", "963", "964", "965", "966", "967", "968", "970",
                    "971", "972", "973", "974", "975", "976", "977", "992", "993", "994", "995",
                    "996", "998",
                ];

                if country_code.is_empty() || valid_country_codes.contains(&country_code.as_str()) {
                    return true;
                }
            } else {
                // No country code, validate as local number
                return true;
            }
        }
    }

    false
}

/// Check if a string is a valid email address
pub fn is_email(value: &str) -> bool {
    // Normalize input: trim and remove excessive whitespace
    let normalized = value.trim().to_lowercase();
    if normalized.is_empty() {
        return false;
    }

    let email_re = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();

    if email_re.is_match(&normalized) {
        // Additional email validation: domain part must contain a dot
        if let Some(at_pos) = normalized.find('@') {
            let domain = &normalized[at_pos + 1..];
            if domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.') {
                return true;
            }
        }
    }

    false
}

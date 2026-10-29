//! Duration parsing and formatting for the timer CLI.

/// Parses a human duration into seconds.
///
/// Accepts a bare number (seconds) or a sequence of `<number><unit>` groups
/// where units are `d`, `h`, `m`, `s` (case-insensitive), e.g. `1h30m`, `90s`,
/// `2d`. A trailing bare number is treated as seconds (`5m30` → 5m 30s).
///
/// # Errors
/// Returns a message when the input is empty or malformed.
pub fn parse_duration(input: &str) -> Result<u64, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(String::from("empty duration"));
    }

    if let Ok(secs) = trimmed.parse::<u64>() {
        return Ok(secs);
    }

    let mut total: u64 = 0;
    let mut number = String::new();
    let mut saw_unit = false;

    for ch in trimmed.chars() {
        if ch.is_ascii_digit() {
            number.push(ch);
        } else if ch.is_ascii_alphabetic() {
            if number.is_empty() {
                return Err(format!("missing number before '{ch}'"));
            }
            let value: u64 = number
                .parse()
                .map_err(|_| format!("invalid number: {number}"))?;
            let multiplier = match ch.to_ascii_lowercase() {
                'd' => 86_400,
                'h' => 3_600,
                'm' => 60,
                's' => 1,
                other => return Err(format!("unknown unit '{other}'")),
            };
            total = total.saturating_add(value.saturating_mul(multiplier));
            number.clear();
            saw_unit = true;
        } else if ch.is_whitespace() {
            continue;
        } else {
            return Err(format!("invalid character '{ch}'"));
        }
    }

    if !number.is_empty() {
        let value: u64 = number
            .parse()
            .map_err(|_| format!("invalid number: {number}"))?;
        total = total.saturating_add(value);
        saw_unit = true;
    }

    if !saw_unit {
        return Err(format!("invalid duration: {input}"));
    }

    Ok(total)
}

/// Parses a signed delta (`+5m`, `-30s`, or plain) into signed seconds.
///
/// # Errors
/// Returns a message when the magnitude is malformed.
pub fn parse_signed(input: &str) -> Result<i64, String> {
    let trimmed = input.trim();

    if let Some(rest) = trimmed.strip_prefix('-') {
        let secs = parse_duration(rest)?;
        Ok(-(secs as i64))
    } else {
        let rest = trimmed.strip_prefix('+').unwrap_or(trimmed);
        let secs = parse_duration(rest)?;
        Ok(secs as i64)
    }
}

/// Formats seconds as a human-readable largest-to-smallest string.
pub fn format_human(total_secs: u64) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{seconds}s"));
    }

    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_seconds() {
        assert_eq!(parse_duration("90"), Ok(90));
    }

    #[test]
    fn parses_units() {
        assert_eq!(parse_duration("5m"), Ok(300));
        assert_eq!(parse_duration("1h30m"), Ok(5_400));
        assert_eq!(parse_duration("2d"), Ok(172_800));
        assert_eq!(parse_duration("1h2m3s"), Ok(3_723));
    }

    #[test]
    fn parses_trailing_seconds() {
        assert_eq!(parse_duration("5m30"), Ok(330));
    }

    #[test]
    fn rejects_invalid() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("5x").is_err());
    }

    #[test]
    fn parses_signed() {
        assert_eq!(parse_signed("+5m"), Ok(300));
        assert_eq!(parse_signed("-30s"), Ok(-30));
        assert_eq!(parse_signed("10"), Ok(10));
    }

    #[test]
    fn formats_human() {
        assert_eq!(format_human(0), "0s");
        assert_eq!(format_human(65), "1m 5s");
        assert_eq!(format_human(3_723), "1h 2m 3s");
    }
}

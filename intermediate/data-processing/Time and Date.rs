// time_and_date.rs
// Comprehensive examples of time and date handling in Rust

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};

// =========================================
// STANDARD LIBRARY TIME EXAMPLES
// =========================================

pub fn system_time_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SYSTEM TIME OPERATIONS ===");
    
    // Get current system time
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);
    
    // Convert to UNIX timestamp
    let timestamp = now.duration_since(UNIX_EPOCH)?;
    println!("UNIX timestamp: {} seconds", timestamp.as_secs());
    println!("UNIX timestamp: {} milliseconds", timestamp.as_millis());
    println!("UNIX timestamp: {} microseconds", timestamp.as_micros());
    println!("UNIX timestamp: {} nanoseconds", timestamp.as_nanos());
    
    // Create SystemTime from timestamp
    let specific_time = UNIX_EPOCH + Duration::from_secs(1640995200); // 2022-01-01
    println!("Specific time: {:?}", specific_time);
    
    // Duration calculations
    let later_time = now + Duration::from_secs(3600); // 1 hour later
    println!("1 hour later: {:?}", later_time);
    
    // Time difference
    let time_diff = later_time.duration_since(now)?;
    println!("Time difference: {:?}", time_diff);
    
    // SystemTime comparison
    let earlier_time = now - Duration::from_secs(1800); // 30 minutes earlier
    if now > earlier_time {
        println!("Current time is later than earlier time");
    }
    
    Ok(())
}

pub fn instant_operations() {
    println!("=== INSTANT OPERATIONS ===");
    
    // Instant for measuring elapsed time
    let start = Instant::now();
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));
    
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Elapsed milliseconds: {}", elapsed.as_millis());
    println!("Elapsed microseconds: {}", elapsed.as_micros());
    println!("Elapsed nanoseconds: {}", elapsed.as_nanos());
    
    // Instant comparison
    let instant1 = Instant::now();
    std::thread::sleep(Duration::from_millis(50));
    let instant2 = Instant::now();
    
    if instant2 > instant1 {
        println!("instant2 is later than instant1");
    }
    
    // Duration from Instant
    let duration_between = instant2.duration_since(instant1);
    println!("Duration between instants: {:?}", duration_between);
    
    // Instant for benchmarking
    benchmark_operations();
}

fn benchmark_operations() {
    println!("=== BENCHMARKING OPERATIONS ===");
    
    let iterations = 1000;
    
    // Benchmark vector operations
    let start = Instant::now();
    let mut vec = Vec::new();
    for i in 0..iterations {
        vec.push(i);
    }
    let vec_time = start.elapsed();
    
    // Benchmark hash map operations
    let start = Instant::now();
    let mut map = std::collections::HashMap::new();
    for i in 0..iterations {
        map.insert(i, i * 2);
    }
    let map_time = start.elapsed();
    
    println!("Benchmark results ({} iterations):", iterations);
    println!("  Vector push: {:?}", vec_time);
    println!("  HashMap insert: {:?}", map_time);
    
    // Compare performance
    if vec_time < map_time {
        println!("  Vector operations were faster by {:?}", map_time - vec_time);
    } else {
        println!("  HashMap operations were faster by {:?}", vec_time - map_time);
    }
}

pub fn duration_operations() {
    println!("=== DURATION OPERATIONS ===");
    
    // Creating durations
    let one_second = Duration::from_secs(1);
    let one_millisecond = Duration::from_millis(1);
    let one_microsecond = Duration::from_micros(1);
    let one_nanosecond = Duration::from_nanos(1);
    
    println!("One second: {:?}", one_second);
    println!("One millisecond: {:?}", one_millisecond);
    println!("One microsecond: {:?}", one_microsecond);
    println!("One nanosecond: {:?}", one_nanosecond);
    
    // Creating from multiple units
    let custom_duration = Duration::new(3600, 500_000_000); // 1 hour + 500ms
    println!("Custom duration: {:?}", custom_duration);
    
    // Duration arithmetic
    let duration1 = Duration::from_secs(60);
    let duration2 = Duration::from_secs(30);
    
    let sum = duration1 + duration2;
    let difference = duration1 - duration2;
    let scaled = duration1 * 2;
    let halved = duration1 / 2;
    
    println!("Sum: {:?}", sum);
    println!("Difference: {:?}", difference);
    println!("Scaled by 2: {:?}", scaled);
    println!("Halved: {:?}", halved);
    
    // Duration comparison
    if duration1 > duration2 {
        println!("duration1 is longer than duration2");
    }
    
    // Accessing duration components
    let complex_duration = Duration::from_secs(3661); // 1 hour, 1 minute, 1 second
    println!("Total seconds: {}", complex_duration.as_secs());
    println!("Total milliseconds: {}", complex_duration.as_millis());
    println!("Total microseconds: {}", complex_duration.as_micros());
    println!("Total nanoseconds: {}", complex_duration.as_nanos());
    
    // Checked arithmetic (prevents overflow)
    let large_duration = Duration::from_secs(u64::MAX / 2);
    let additional = Duration::from_secs(1000);
    
    match large_duration.checked_add(additional) {
        Some(sum) => println!("Checked sum: {:?}", sum),
        None => println!("Addition would overflow"),
    }
    
    match large_duration.checked_sub(additional) {
        Some(diff) => println!("Checked difference: {:?}", diff),
        None => println!("Subtraction would underflow"),
    }
    
    // Duration in different units
    let minute_duration = Duration::from_secs(90);
    println!("90 seconds is {} minutes", minute_duration.as_secs() as f64 / 60.0);
    println!("90 seconds is {} hours", minute_duration.as_secs() as f64 / 3600.0);
}

// =========================================
// CHRONO LIBRARY EXAMPLES
// =========================================

pub fn chrono_datetime_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CHRONO DATETIME EXAMPLES ===");
    
    use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone};
    
    // Current UTC time
    let utc_now: DateTime<Utc> = Utc::now();
    println!("UTC now: {}", utc_now);
    println!("UTC now formatted: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));
    println!("UTC now ISO: {}", utc_now.to_rfc3339());
    
    // Current local time
    let local_now: DateTime<Local> = Local::now();
    println!("Local now: {}", local_now);
    println!("Local now formatted: {}", local_now.format("%Y-%m-%d %H:%M:%S %Z"));
    
    // NaiveDateTime (no time zone)
    let naive_dt = NaiveDateTime::parse_from_str(
        "2023-12-25 14:30:00",
        "%Y-%m-%d %H:%M:%S"
    )?;
    println!("Naive datetime: {}", naive_dt);
    
    // Create DateTime from components
    let custom_dt = Local.with_ymd_and_hms(2023, 12, 25, 14, 30, 0)?;
    println!("Custom datetime: {}", custom_dt);
    
    // DateTime arithmetic
    let tomorrow = utc_now + chrono::Duration::days(1);
    let next_week = utc_now + chrono::Duration::weeks(1);
    let next_month = utc_now + chrono::Duration::days(30);
    let next_year = utc_now + chrono::Duration::days(365);
    
    println!("Tomorrow: {}", tomorrow);
    println!("Next week: {}", next_week);
    println!("Next month: {}", next_month);
    println!("Next year: {}", next_year);
    
    // Time difference
    let past_dt = utc_now - chrono::Duration::days(7);
    let time_diff = utc_now.signed_duration_since(past_dt);
    println!("Time difference: {:?}", time_diff);
    println!("Days difference: {}", time_diff.num_days());
    
    // DateTime comparison
    let future_dt = utc_now + chrono::Duration::hours(2);
    if future_dt > utc_now {
        println!("Future time is later than current time");
    }
    
    Ok(())
}

pub fn datetime_components() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DATETIME COMPONENTS ===");
    
    use chrono::{Datelike, Timelike, Weekday};
    
    let dt = Local::now();
    
    // Date components
    println!("Year: {}", dt.year());
    println!("Month: {}", dt.month());
    println!("Day: {}", dt.day());
    println!("Day of week: {:?}", dt.weekday());
    println!("Day of year: {}", dt.ordinal()); // 1-366
    println!("ISO week: {}", dt.iso_week().week());
    
    // Time components
    println!("Hour: {}", dt.hour());
    println!("Minute: {}", dt.minute());
    println!("Second: {}", dt.second());
    println!("Nanosecond: {}", dt.nanosecond());
    
    // Weekday operations
    match dt.weekday() {
        Weekday::Mon => println!("It's Monday"),
        Weekday::Tue => println!("It's Tuesday"),
        Weekday::Wed => println!("It's Wednesday"),
        Weekday::Thu => println!("It's Thursday"),
        Weekday::Fri => println!("It's Friday"),
        Weekday::Sat => println!("It's Saturday"),
        Weekday::Sun => println!("It's Sunday"),
    }
    
    // Date calculations
    let first_day_of_month = dt.with_day0(1).unwrap();
    let last_day_of_month = dt.with_day0(0).unwrap();
    
    println!("First day of month: {}", first_day_of_month);
    println!("Last day of month: {}", last_day_of_month);
    
    // Start and end of day
    let start_of_day = dt.date().and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = dt.date().and_hms_opt(23, 59, 59).unwrap();
    
    println!("Start of day: {}", start_of_day);
    println!("End of day: {}", end_of_day);
    
    // Is weekend?
    let is_weekend = dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun;
    println!("Is weekend: {}", is_weekend);
    
    // Is leap year?
    let is_leap = dt.year() % 4 == 0 && (dt.year() % 100 != 0 || dt.year() % 400 == 0);
    println!("Is leap year: {}", is_leap);
    
    Ok(())
}

pub fn timezone_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TIMEZONE EXAMPLES ===");
    
    use chrono::{DateTime, Utc, Local, TimeZone, FixedOffset};
    
    let utc_time: DateTime<Utc> = Utc::now();
    println!("UTC time: {}", utc_time);
    
    // Convert to local time
    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
    println!("Local time: {}", local_time);
    
    // Convert to specific time zone
    let eastern = FixedOffset::east_opt(5 * 3600) // UTC+5
        .ok_or("Invalid offset")?;
    let eastern_time: DateTime<FixedOffset> = utc_time.with_timezone(&eastern);
    println!("Eastern time (UTC+5): {}", eastern_time);
    
    // Parse time zone aware datetime
    let tz_aware_str = "2023-12-25T14:30:00+05:00";
    let parsed_tz: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(tz_aware_str)?;
    println!("Parsed timezone-aware time: {}", parsed_tz);
    
    // Convert between time zones
    let utc_from_eastern = eastern_time.with_timezone(&Utc);
    println!("UTC from eastern: {}", utc_from_eastern);
    
    // Get time zone offset
    let local_offset = local_time.offset().fix();
    println!("Local time zone offset: {}", local_offset);
    
    // Multiple time zones
    let time_zones = vec![
        ("UTC", utc_time),
        ("Local", local_time.with_timezone(&Utc)),
        ("Eastern", eastern_time.with_timezone(&Utc)),
    ];
    
    for (name, time) in time_zones {
        println!("{}: {}", name, time.format("%Y-%m-%d %H:%M:%S %Z"));
    }
    
    Ok(())
}

pub fn chrono_duration_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CHRONO DURATION EXAMPLES ===");
    
    use chrono::{Duration, Datelike};
    
    let dt = Local::now();
    
    // Duration from standard units
    let one_day = Duration::days(1);
    let one_hour = Duration::hours(1);
    let one_minute = Duration::minutes(1);
    let one_second = Duration::seconds(1);
    let one_millisecond = Duration::milliseconds(1);
    
    // Duration arithmetic
    let future_time = dt + Duration::days(7) + Duration::hours(3);
    println!("Future time: {}", future_time);
    
    // Period vs Duration
    let calendar_month = Duration::days(30);
    let exact_month = Duration::days(30);
    
    println!("Calendar month: {:?}", calendar_month);
    println!("Exact month: {:?}", exact_month);
    
    // Duration between dates
    let past_date = dt - Duration::days(365);
    let time_between = dt.signed_duration_since(past_date);
    println!("Time between: {:?}", time_between);
    
    // Check if date is within range
    let start_date = dt - Duration::days(7);
    let end_date = dt + Duration::days(7);
    
    let check_date = dt - Duration::days(3);
    let is_in_range = check_date >= start_date && check_date <= end_date;
    println!("Date {} is in range: {}", check_date, is_in_range);
    
    // Age calculation
    let birth_date = Local.ymd(1990, 6, 15).and_hms_opt(0, 0, 0).unwrap();
    let age = dt.signed_duration_since(birth_date);
    let years = age.num_days() / 365;
    let months = age.num_days() / 30;
    
    println!("Age: approximately {} years", years);
    println!("Age: approximately {} months", months);
    println!("Exact age: {:?}", age);
    
    // Duration in different units
    let large_duration = Duration::days(365 + 30); // 1 year + 30 days
    println!("Duration in days: {}", large_duration.num_days());
    println!("Duration in weeks: {}", large_duration.num_weeks());
    println!("Duration in hours: {}", large_duration.num_hours());
    
    Ok(())
}

// =========================================
// FORMATTING AND PARSING EXAMPLES
// =========================================

pub fn formatting_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FORMATTING EXAMPLES ===");
    
    use chrono::{DateTime, Local, NaiveDate};
    
    let dt: DateTime<Local> = Local::now();
    
    // Standard formatting
    println!("ISO 8601: {}", dt.format("%Y-%m-%dT%H:%M:%S%.f%Z"));
    println!("RFC 2822: {}", dt.format("%a, %d %b %Y %H:%M:%S %z"));
    println!("Custom: {}", dt.format("Today is %A, %B %d, %Y"));
    
    // Different locale formats
    println!("US format: {}", dt.format("%m/%d/%Y"));
    println!("European format: {}", dt.format("%d.%m.%Y"));
    println!("ISO format: {}", dt.format("%Y-%m-%d"));
    println!("Military time: {}", dt.format("%H%M %Z"));
    
    // Time-only formatting
    println!("12-hour: {}", dt.format("%I:%M:%S %p"));
    println!("24-hour: {}", dt.format("%H:%M:%S"));
    println!("With timezone: {}", dt.format("%H:%M:%S %Z"));
    
    // Date-only formatting
    println!("Long date: {}", dt.format("%A, %B %d, %Y"));
    println!("Short date: {}", dt.format("%m/%d/%y"));
    println!("Week number: {}", dt.format("%Y-W%U")); // Year-week
    
    // Relative time formatting
    let now = Local::now();
    let past = now - chrono::Duration::days(7);
    let future = now + chrono::Duration::hours(3);
    
    println!("7 days ago: {}", past.format("%Y-%m-%d"));
    println!("In 3 hours: {}", future.format("%Y-%m-%d %H:%M"));
    
    // Custom formatting with ordinal
    let day = dt.day();
    let ordinal = match day {
        1 | 21 | 31 => "1st",
        2 | 22 => "2nd",
        3 | 23 => "3rd",
        _ => format!("{}th", day),
    };
    println!("Ordinal: {}", ordinal);
    
    Ok(())
}

pub fn parsing_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PARSING EXAMPLES ===");
    
    use chrono::{NaiveDateTime, NaiveDate};
    
    // Parse known formats
    let iso_date = "2023-12-25T14:30:00Z";
    let parsed_iso: chrono::DateTime<chrono::Utc> = chrono::DateTime::parse_from_rfc3339(iso_date)?;
    println!("Parsed ISO: {}", parsed_iso);
    
    // Parse custom format
    let custom_date = "12/25/2023 14:30:00";
    let parsed_custom: NaiveDateTime = NaiveDateTime::parse_from_str(
        custom_date,
        "%m/%d/%Y %H:%M:%S"
    )?;
    println!("Parsed custom: {}", parsed_custom);
    
    // Parse different date formats
    let dates = vec![
        "2023-12-25",
        "12/25/2023",
        "Dec 25, 2023",
        "25.12.2023",
        "25-Dec-2023",
    ];
    
    for date_str in dates {
        if let Ok(parsed) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            println!("'{}' -> {}", date_str, parsed);
        } else if let Ok(parsed) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
            println!("'{}' -> {}", date_str, parsed);
        } else if let Ok(parsed) = NaiveDate::parse_from_str(date_str, "%b %d, %Y") {
            println!("'{}' -> {}", date_str, parsed);
        } else if let Ok(parsed) = NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
            println!("'{}' -> {}", date_str, parsed);
        } else if let Ok(parsed) = NaiveDate::parse_from_str(date_str, "%d-%b-%Y") {
            println!("'{}' -> {}", date_str, parsed);
        } else {
            println!("Could not parse '{}'", date_str);
        }
    }
    
    // Parse with error handling
    let invalid_date = "2023-13-45"; // Invalid month and day
    match NaiveDate::parse_from_str(invalid_date, "%Y-%m-%d") {
        Ok(date) => println!("Parsed: {}", date),
        Err(e) => println!("Error parsing '{}': {}", invalid_date, e),
    }
    
    // Parse time components
    let time_str = "14:30:45";
    match chrono::NaiveTime::parse_from_str(time_str, "%H:%M:%S") {
        Ok(time) => println!("Parsed time: {}", time),
        Err(e) => println!("Error parsing time: {}", e),
    }
    
    Ok(())
}

// =========================================
// CALENDAR OPERATIONS EXAMPLES
// =========================================

pub fn calendar_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CALENDAR OPERATIONS ===");
    
    use chrono::{NaiveDate, Datelike, Weekday};
    
    let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    
    // Calendar calculations
    println!("Date: {}", date);
    println!("Day of week: {:?}", date.weekday());
    println!("Day of year: {}", date.ordinal());
    println!("ISO week: {}", date.iso_week().week());
    
    // Start and end of week
    let start_of_week = date - chrono::Duration::days(date.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + chrono::Duration::days(6);
    
    println!("Start of week: {}", start_of_week);
    println!("End of week: {}", end_of_week);
    
    // Start and end of month
    let start_of_month = date.with_day0(1).unwrap();
    let end_of_month = date.with_day0(0).unwrap();
    
    println!("Start of month: {}", start_of_month);
    println!("End of month: {}", end_of_month);
    
    // Start and end of year
    let start_of_year = date.with_month0(1).unwrap().with_day0(1).unwrap();
    let end_of_year = date.with_month0(1).unwrap().with_day0(0).unwrap();
    
    println!("Start of year: {}", start_of_year);
    println!("End of year: {}", end_of_year);
    
    // Leap year detection
    let years = vec![1900, 2000, 2004, 2020, 2024];
    for year in years {
        let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
        println!("Is {} a leap year: {}", year, is_leap);
    }
    
    // Days in month
    let month = 2; // February
    let leap_year = 2024; // Leap year
    let days_in_month = if month == 2 {
        if NaiveDate::from_ymd_opt(leap_year, 2, 29).is_some() {
            29
        } else {
            28
        }
    } else {
        // For other months, use chrono's month length
        let test_date = NaiveDate::from_ymd_opt(leap_year, month + 1, 1).unwrap();
        test_date.pred_opt().unwrap().day()
    };
    
    println!("Days in February {} ({}): {}", leap_year, month, days_in_month);
    
    // Business days calculation
    let business_days = count_business_days(start_of_month, end_of_month);
    println!("Business days in month: {}", business_days);
    
    // Week of year
    let week_of_year = date.iso_week().week();
    println!("Week of year: {}", week_of_year);
    
    // Quarter of year
    let quarter = (date.month() - 1) / 3 + 1;
    println!("Quarter of year: {}", quarter);
    
    Ok(())
}

fn count_business_days(start: NaiveDate, end: NaiveDate) -> i32 {
    let mut count = 0;
    let mut current = start;
    
    while current <= end {
        match current.weekday() {
            Weekday::Sat | Weekday::Sun => {
                // Weekend, don't count
            }
            _ => {
                count += 1;
            }
        }
        
        if current == end {
            break;
        }
        
        current = current.succ_opt().unwrap();
    }
    
    count
}

// =========================================
// HUMAN-READABLE TIME EXAMPLES
// =========================================

pub fn human_readable_time() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HUMAN-READABLE TIME EXAMPLES ===");
    
    // Using humantime crate for formatting
    // Note: This would require adding humantime dependency
    // For demonstration, we'll implement simple human-readable formatting
    
    let durations = vec![
        Duration::from_secs(45),
        Duration::from_secs(90),
        Duration::from_secs(3600),
        Duration::from_secs(7200),
        Duration::from_secs(86400),
        Duration::from_secs(604800),
        Duration::from_secs(31536000), // 1 year
        Duration::from_secs(63072000), // 2 years
    ];
    
    for duration in durations {
        let human_readable = format_duration_human(duration);
        println!("{:?} -> {}", duration, human_readable);
    }
    
    // Custom relative time formatting
    let now = std::time::SystemTime::now();
    let past = now - Duration::from_secs(86400); // 1 day ago
    let future = now + Duration::from_secs(3600); // 1 hour from now
    
    println!("Past: {}", format_duration_human(past.duration_since(now).unwrap()));
    println!("Future: {}", format_duration_human(future.duration_since(now).unwrap()));
    
    // Time until specific event
    let target_time = now + Duration::from_secs(259200); // 3 days from now
    let time_until = target_time.duration_since(now);
    println!("Time until event: {}", format_duration_human(time_until));
    
    Ok(())
}

fn format_duration_human(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{} seconds", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        if seconds == 0 {
            format!("{} minutes", minutes)
        } else {
            format!("{} minutes {} seconds", minutes, seconds)
        }
    } else if total_seconds < 86400 {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        if minutes == 0 {
            format!("{} hours", hours)
        } else {
            format!("{} hours {} minutes", hours, minutes)
        }
    } else if total_seconds < 2592000 { // 30 days
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        if hours == 0 {
            format!("{} days", days)
        } else {
            format!("{} days {} hours", days, hours)
        }
    } else {
        let days = total_seconds / 86400;
        format!("approximately {} days", days)
    }
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TIME AND DATE DEMONSTRATIONS ===\n");
    
    system_time_operations()?;
    println!();
    
    instant_operations();
    println!();
    
    duration_operations();
    println!();
    
    chrono_datetime_examples()?;
    println!();
    
    datetime_components()?;
    println!();
    
    timezone_examples()?;
    println!();
    
    chrono_duration_examples()?;
    println!();
    
    formatting_examples()?;
    println!();
    
    parsing_examples()?;
    println!();
    
    calendar_operations()?;
    println!();
    
    human_readable_time()?;
    
    println!("\n=== TIME AND DATE DEMONSTRATIONS COMPLETE ===");
    println!("Key takeaways:");
    println!("- Use std::time for basic operations");
    println!("- Use chrono for comprehensive date/time functionality");
    println!("- Store times in UTC, convert to local for display");
    println!("- Handle time zones explicitly");
    println!("- Use Duration for time arithmetic");
    println!("- Validate date/time inputs");
    println!("- Consider leap years and calendar quirks");
    
    Ok(())
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_time() {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).unwrap();
        assert!(timestamp.as_secs() > 0);
    }
    
    #[test]
    fn test_instant() {
        let start = Instant::now();
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 0);
    }
    
    #[test]
    fn test_duration_arithmetic() {
        let duration1 = Duration::from_secs(60);
        let duration2 = Duration::from_secs(30);
        let sum = duration1 + duration2;
        assert_eq!(sum.as_secs(), 90);
        
        let difference = duration1 - duration2;
        assert_eq!(difference.as_secs(), 30);
    }
    
    #[test]
    fn test_chrono_parsing() {
        use chrono::NaiveDate;
        
        let date_str = "2023-12-25";
        let parsed = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        assert_eq!(parsed.year(), 2023);
        assert_eq!(parsed.month(), 12);
        assert_eq!(parsed.day(), 25);
    }
    
    #[test]
    fn test_duration_formatting() {
        let duration = format_duration_human(Duration::from_secs(3661));
        assert!(duration.contains("hour"));
        assert!(duration.contains("minute"));
        assert!(duration.contains("second"));
    }
    
    #[test]
    fn test_leap_year() {
        use chrono::NaiveDate;
        
        // 2020 is a leap year
        assert!(NaiveDate::from_ymd_opt(2020, 2, 29).is_some());
        
        // 2021 is not a leap year
        assert!(!NaiveDate::from_ymd_opt(2021, 2, 29).is_some());
    }
    
    #[test]
    fn test_business_days() {
        use chrono::NaiveDate;
        
        let start = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2023, 1, 8).unwrap(); // Sunday
        
        let business_days = count_business_days(start, end);
        assert_eq!(business_days, 5); // Mon-Fri
    }
}

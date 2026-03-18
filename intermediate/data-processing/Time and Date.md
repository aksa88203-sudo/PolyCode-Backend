# Time and Date in Rust

## Overview

Rust provides robust time and date handling through its standard library and excellent third-party crates. This guide covers working with timestamps, durations, time zones, calendars, and formatting in Rust.

---

## Core Time Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `std::time` | Standard library time handling | SystemTime, Instant, Duration |
| `chrono` | Comprehensive date/time library | Time zones, calendars, formatting |
| `time` | Modern time library | Duration, date, time, formatting |
| `jiff` | Date/time library with time zones | IANA time zones, calendars |
| `humantime` | Human-readable time formatting | "2 hours ago", "in 3 days" |

---

## Standard Library Time

### SystemTime and Instant

```rust
use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};

fn system_time_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Get current system time
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);
    
    // Convert to UNIX timestamp
    let timestamp = now.duration_since(UNIX_EPOCH)?;
    println!("UNIX timestamp: {} seconds", timestamp.as_secs());
    println!("UNIX timestamp: {} milliseconds", timestamp.as_millis());
    
    // Create SystemTime from timestamp
    let specific_time = UNIX_EPOCH + Duration::from_secs(1640995200); // 2022-01-01
    println!("Specific time: {:?}", specific_time);
    
    // Duration calculations
    let later_time = now + Duration::from_secs(3600); // 1 hour later
    println!("1 hour later: {:?}", later_time);
    
    // Time difference
    let time_diff = later_time.duration_since(now)?;
    println!("Time difference: {:?}", time_diff);
    
    Ok(())
}

fn instant_operations() {
    // Instant for measuring elapsed time
    let start = Instant::now();
    
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Elapsed milliseconds: {}", elapsed.as_millis());
    
    // Instant comparison
    let instant1 = Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(50));
    let instant2 = Instant::now();
    
    if instant2 > instant1 {
        println!("instant2 is later than instant1");
    }
    
    // Duration from Instant
    let duration_between = instant2.duration_since(instant1);
    println!("Duration between instants: {:?}", duration_between);
}
```

### Duration Operations

```rust
use std::time::Duration;

fn duration_operations() {
    // Creating durations
    let one_second = Duration::from_secs(1);
    let one_millisecond = Duration::from_millis(1);
    let one_microsecond = Duration::from_micros(1);
    let one_nanosecond = Duration::from_nanos(1);
    
    // Creating from multiple units
    let custom_duration = Duration::new(3600, 500_000_000); // 1 hour + 500ms
    println!("Custom duration: {:?}", custom_duration);
    
    // Duration arithmetic
    let duration1 = Duration::from_secs(60);
    let duration2 = Duration::from_secs(30);
    
    let sum = duration1 + duration2;
    let difference = duration1 - duration2;
    
    println!("Sum: {:?}", sum);
    println!("Difference: {:?}", difference);
    
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
}
```

---

## Chrono Library

### DateTime and NaiveDateTime

```rust
use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone};

fn chrono_datetime_examples() -> Result<(), Box<dyn std::error::Error>> {
    // Current UTC time
    let utc_now: DateTime<Utc> = Utc::now();
    println!("UTC now: {}", utc_now);
    println!("UTC now formatted: {}", utc_now.format("%Y-%m-%d %H:%M:%S"));
    
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
    
    println!("Tomorrow: {}", tomorrow);
    println!("Next week: {}", next_week);
    println!("Next month: {}", next_month);
    
    // Time difference
    let past_dt = utc_now - chrono::Duration::days(7);
    let time_diff = utc_now.signed_duration_since(past_dt);
    println!("Time difference: {:?}", time_diff);
    
    Ok(())
}
```

### Date and Time Components

```rust
use chrono::{Datelike, Timelike, Weekday};

fn datetime_components() -> Result<(), Box<dyn std::error::Error>> {
    let dt = Local::now();
    
    // Date components
    println!("Year: {}", dt.year());
    println!("Month: {}", dt.month());
    println!("Day: {}", dt.day());
    println!("Day of week: {:?}", dt.weekday());
    println!("Day of year: {}", dt.ordinal()); // 1-366
    
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
    println!("First day of month: {}", first_day_of_month);
    
    let last_day_of_month = dt.with_day0(0).unwrap();
    println!("Last day of month: {}", last_day_of_month);
    
    // Start and end of day
    let start_of_day = dt.date().and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = dt.date().and_hms_opt(23, 59, 59).unwrap();
    
    println!("Start of day: {}", start_of_day);
    println!("End of day: {}", end_of_day);
    
    // Is weekend?
    let is_weekend = dt.weekday() == Weekday::Sat || dt.weekday() == Weekday::Sun;
    println!("Is weekend: {}", is_weekend);
    
    Ok(())
}
```

### Time Zones

```rust
use chrono::{DateTime, Utc, Local, TimeZone, FixedOffset};

fn timezone_examples() -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Time zone name (platform-dependent)
    #[cfg(unix)]
    {
        use chrono::offset::Local;
        let time_zone_name = Local::now().offset().to_string();
        println!("Time zone name: {}", time_zone_name);
    }
    
    Ok(())
}
```

### Duration and Period

```rust
use chrono::{Duration, Datelike};

fn chrono_duration_examples() -> Result<(), Box<dyn std::error::Error>> {
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
    // Duration is exact time (86,400 seconds = 1 day)
    // Period is calendar time (1 month = varies in length)
    
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
    println!("Age: approximately {} years", years);
    
    Ok(())
}
```

---

## Formatting and Parsing

### Custom Date Formatting

```rust
use chrono::{DateTime, Local, NaiveDate};

fn formatting_examples() -> Result<(), Box<dyn std::error::Error>> {
    let dt: DateTime<Local> = Local::now();
    
    // Standard formatting
    println!("ISO 8601: {}", dt.format("%Y-%m-%dT%H:%M:%S%.f%Z"));
    println!("RFC 2822: {}", dt.format("%a, %d %b %Y %H:%M:%S %z"));
    println!("Custom: {}", dt.format("Today is %A, %B %d, %Y"));
    
    // Different locale formats
    println!("US format: {}", dt.format("%m/%d/%Y"));
    println!("European format: {}", dt.format("%d.%m.%Y"));
    println!("ISO format: {}", dt.format("%Y-%m-%d"));
    
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
    
    Ok(())
}

fn parsing_examples() -> Result<(), Box<dyn std::error::Error>> {
    // Parse known formats
    let iso_date = "2023-12-25T14:30:00Z";
    let parsed_iso: DateTime<Utc> = DateTime::parse_from_rfc3339(iso_date)?;
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
    
    Ok(())
}
```

---

## Time Library (Modern Alternative)

### Time Crate Usage

```rust
use time::{Date, Duration, OffsetDateTime, UtcOffset, PrimitiveDateTime};

fn time_crate_examples() -> Result<(), Box<dyn std::error::Error>> {
    // Current UTC time
    let now_utc = OffsetDateTime::now_utc();
    println!("Current UTC: {}", now_utc);
    
    // Current local time
    let now_local = OffsetDateTime::now_local()?;
    println!("Current local: {}", now_local);
    
    // Create specific date/time
    let specific_date = Date::from_calendar_date(2023, 12, 25)?;
    let specific_time = PrimitiveDateTime::new(specific_date, time::Time::from_hms(14, 30, 0)?);
    let specific_datetime = specific_time.assume_utc();
    println!("Specific datetime: {}", specific_datetime);
    
    // Duration operations
    let one_day = Duration::days(1);
    let one_hour = Duration::hours(1);
    let future_time = now_utc + one_day + one_hour;
    println!("Future time: {}", future_time);
    
    // Time difference
    let past_time = now_utc - Duration::weeks(2);
    let time_diff = now_utc - past_time;
    println!("Time difference: {:?}", time_diff);
    
    // Formatting
    println!("Formatted: {}", now_utc.format("%Y-%m-%d %H:%M:%S"));
    println!("ISO 8601: {}", now_utc.format(time::format_description::well_known::Rfc3339));
    
    // Parsing
    let parsed = OffsetDateTime::parse("2023-12-25T14:30:00+00:00", &time::format_description::well_known::Rfc3339)?;
    println!("Parsed: {}", parsed);
    
    Ok(())
}
```

---

## Human-Readable Time

### Humantime Library

```rust
use humantime::{format_duration, parse_duration};
use std::time::Duration;

fn human_readable_time() -> Result<(), Box<dyn std::error::Error>> {
    // Format durations as human-readable
    let durations = vec![
        Duration::from_secs(45),
        Duration::from_secs(90),
        Duration::from_secs(3600),
        Duration::from_secs(7200),
        Duration::from_secs(86400),
        Duration::from_secs(604800),
    ];
    
    for duration in durations {
        let human_readable = format_duration(duration);
        println!("{:?} -> {}", duration, human_readable);
    }
    
    // Parse human-readable durations
    let human_durations = vec![
        "30 seconds",
        "5 minutes",
        "2 hours",
        "3 days",
        "1 week",
        "2 weeks",
        "1 month",
        "1 year",
    ];
    
    for human_str in human_durations {
        match parse_duration(human_str) {
            Ok(duration) => println!("'{}' -> {:?}", human_str, duration),
            Err(e) => println!("Error parsing '{}': {}", human_str, e),
        }
    }
    
    // Custom formatting
    let custom_duration = Duration::from_secs(3661); // 1 hour, 1 minute, 1 second
    
    // Using format! with humantime
    println!("Custom formatted: {}", format_duration(custom_duration));
    
    // Relative time formatting
    let now = std::time::SystemTime::now();
    let past = now - Duration::from_secs(86400); // 1 day ago
    let future = now + Duration::from_secs(3600); // 1 hour from now
    
    println!("Past: {}", format_duration(past.duration_since(now).unwrap()));
    println!("Future: {}", format_duration(future.duration_since(now).unwrap()));
    
    Ok(())
}
```

---

## Calendar Operations

### Working with Calendars

```rust
use chrono::{NaiveDate, Datelike, Weekday};

fn calendar_operations() -> Result<(), Box<dyn std::error::Error>> {
    let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    
    // Calendar calculations
    println!("Date: {}", date);
    println!("Day of week: {:?}", date.weekday());
    println!("Day of year: {}", date.ordinal());
    println!("Week of year: {}", date.iso_week().week());
    
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
    let year = 2024;
    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    println!("Is {} a leap year: {}", year, is_leap);
    
    // Days in month
    let month = 2; // February
    let year = 2024; // Leap year
    let days_in_month = if month == 2 {
        if NaiveDate::from_ymd_opt(year, 2, 29).is_some() {
            29
        } else {
            28
        }
    } else {
        // For other months, use chrono's month length
        let test_date = NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap();
        test_date.pred_opt().unwrap().day()
    };
    
    println!("Days in month {} {}: {}", year, month, days_in_month);
    
    // Business days calculation
    let business_days = count_business_days(start_of_month, end_of_month);
    println!("Business days in month: {}", business_days);
    
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
```

---

## Key Takeaways

- **std::time** provides basic time operations
- **chrono** offers comprehensive date/time functionality
- **time** is a modern alternative with better ergonomics
- **humantime** provides human-readable formatting
- **Time zones** require careful handling
- **Duration** arithmetic is essential for time calculations
- **Formatting** and parsing should handle errors gracefully

---

## Time and Date Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Use UTC internally** | Store times in UTC | Convert to local only for display |
| **Handle time zones** | Be explicit about time zones | Use chrono's TimeZone features |
| **Validate inputs** | Check date/time validity | Handle parsing errors |
| **Use appropriate types** | Choose right time type | NaiveDateTime vs DateTime |
| **Consider leap years** | Handle February 29th correctly | Use built-in calendar functions |
| **Business logic** | Account for weekends/holidays | Implement business day calculations |
| **Performance** | Cache expensive calculations | Avoid repeated time zone conversions |
| **Testing** | Test edge cases | Leap years, time zones, DST transitions |

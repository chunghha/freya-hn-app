use jiff::{
    civil::{Date, DateTime},
    tz, Timestamp,
};

pub fn format_timestamp(ts: &Timestamp) -> String {
    let now_ms = Timestamp::now().as_millisecond();
    let ts_ms = ts.as_millisecond();

    let diff_ms = now_ms - ts_ms;

    if diff_ms < 0 {
        return "just now".to_string();
    }

    let total_seconds = diff_ms as f64 / 1000.0;

    if total_seconds < 60.0 {
        return format!("{}s ago", total_seconds.round());
    }

    let total_minutes = total_seconds / 60.0;
    if total_minutes < 60.0 {
        return format!("{}m ago", total_minutes.round());
    }

    let total_hours = total_minutes / 60.0;
    if total_hours < 24.0 {
        return format!("{}h ago", total_hours.round());
    }

    // For durations longer than a day, show the specific date.
    //
    // THE DEFINITIVE CORRECTION:
    // 1. `TimeZone::system()` returns a `TimeZone` directly and cannot fail.
    //    It falls back to UTC if the system timezone cannot be determined.
    //    Therefore, the `.expect()` call is removed.
    let tz = tz::TimeZone::system();

    // 2. The rest of the chain is now correct.
    let zoned_datetime = ts.to_zoned(tz);
    let datetime: DateTime = zoned_datetime.datetime();
    let date: Date = datetime.date();

    format!("{}", date)
}

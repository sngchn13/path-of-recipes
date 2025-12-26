use chrono::{DateTime, FixedOffset, Utc};
use time::OffsetDateTime;

/// chrono::DateTime<FixedOffset> → time::OffsetDateTime
pub fn fixed_offset_to_offset_datetime(dt: DateTime<FixedOffset>) -> OffsetDateTime {
    // DB에 FixedOffset으로 들어왔어도, 도메인에서는 UTC 기준 OffsetDateTime 사용
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);

    OffsetDateTime::from_unix_timestamp(utc.timestamp())
        .expect("invalid unix timestamp when converting from chrono to time")
}

/// Option<chrono::DateTime<FixedOffset>> → Option<OffsetDateTime>
pub fn fixed_offset_opt_to_offset_opt(dt: Option<DateTime<FixedOffset>>) -> Option<OffsetDateTime> {
    dt.map(fixed_offset_to_offset_datetime)
}
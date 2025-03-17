use time::macros::format_description;
use time::OffsetDateTime;

// TODO: make it static
// pub static fmt: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub fn to_timeago(time_input: OffsetDateTime) -> String {
    let now = OffsetDateTime::now_utc();
    let offset = now - time_input;

    if offset.whole_weeks() > 2 {
        // ref: https://github.com/time-rs/time/blob/cf6683a7c25a87a59f169ddea2070f91f4f4f4b8/tests/formatting.rs#L280
        let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        time_input.format(&fmt).unwrap()
    } else if offset.whole_weeks() >= 1 {
        format!(
            "{} week{} ago",
            offset.whole_weeks(),
            if offset.whole_weeks() == 1 { "" } else { "s" }
        )
    } else if offset.whole_days() >= 1 {
        format!(
            "{} day{} ago",
            offset.whole_days(),
            if offset.whole_days() == 1 { "" } else { "s" }
        )
    } else if offset.whole_hours() >= 1 {
        format!(
            "{} hour{} ago",
            offset.whole_hours(),
            if offset.whole_hours() == 1 { "" } else { "s" }
        )
    } else if offset.whole_minutes() >= 1 {
        format!(
            "{} minute{} ago",
            offset.whole_minutes(),
            if offset.whole_minutes() == 1 { "" } else { "s" }
        )
    } else {
        "just now".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::format_description;
    use time::macros::datetime;

    #[test]
    fn test_just_now() {
        let res = OffsetDateTime::now_utc();
        assert_eq!(to_timeago(res), "just now");
    }

    #[test]
    fn test_minutes_ago() {
        let res = OffsetDateTime::now_utc() - Duration::minutes(30);
        assert_eq!(to_timeago(res), "30 minutes ago");
    }

    #[test]
    fn test_hours_ago() {
        let res = OffsetDateTime::now_utc() - Duration::hours(5);
        assert_eq!(to_timeago(res), "5 hours ago");
    }

    #[test]
    fn test_days_ago() {
        let offsetdatetime = OffsetDateTime::now_utc() - Duration::days(3);
        assert_eq!(to_timeago(offsetdatetime), "3 days ago");
    }

    #[test]
    fn test_weeks_ago() {
        let res = OffsetDateTime::now_utc() - Duration::weeks(2);
        assert_eq!(to_timeago(res), "2 weeks ago");
    }

    #[test]
    fn test_more_than_2_weeks_ago() {
        //TODO: format properly later
        let date = datetime!(2023-01-01 0:00 UTC);
        let res = to_timeago(date);
        dbg!(res);
    }
}

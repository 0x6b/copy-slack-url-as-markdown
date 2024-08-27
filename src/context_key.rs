use strum_macros::{AsRefStr, EnumProperty, VariantArray};

#[rustfmt::skip]
#[derive(AsRefStr, EnumProperty, VariantArray, Debug)]
pub enum ContextKey {
    #[strum(
        serialize = "channel_name",
        props(
            description = "The name of the channel the message belongs to.",
            example = "general"
        )
    )]
    ChannelName,

    #[strum(
        serialize = "url",
        props(
            description = "The Slack URL of the message.",
            example = "https://xxx.slack.com/archives/..."
        )
    )]
    Url,

    #[strum(
        serialize = "text",
        props(
            description = "The text of the message, which is the vector of the texts split by the new line.",
            example = "Hello, world!"
        )
    )]
    Text,

    #[strum(
        serialize = "timestamp",
        props(
            format = "%Y-%m-%d %H:%M:%S (%Z)",
            description = "The timestamp of the message."
        )
    )]
    Timestamp,

    #[strum(
        serialize = "iso_date",
        props(
            format = "%F",
            description = "Equivalent to `%Y-%m-%d`."
        )
    )]
    IsoDate,

    #[strum(
        serialize = "clock",
        props(
            format = "%T",
            description = "Equivalent to `%H:%M:%S`."
        )
    )]
    Clock,

    #[strum(
        serialize = "year",
        props(
            format = "%Y",
            description = "A full year, including century. Zero padded to 4 digits."
        )
    )]
    Year,

    #[strum(
        serialize = "year_2digit",
        props(
            format = "%y",
            description = "A two-digit year. Represents only 1969-2068. Zero padded."
        )
    )]
    Year2Digit,

    #[strum(
        serialize = "month",
        props(
            format = "%B",
            description = "The full month name."
        )
    )]
    Month,

    #[strum(
        serialize = "month_abbrev",
        props(
            format = "%b",
            description = "The abbreviated month name,."
        )
    )]
    MonthAbbrev,

    #[strum(
        serialize = "month_2digit",
        props(
            format = "%m",
            description = "The month. Zero padded."
        )
    )]
    Month2Digit,

    #[strum(
        serialize = "day",
        props(
            format = "%d",
            description = "The day of the month. Zero-padded."
        )
    )]
    Day,

    #[strum(
        serialize = "day_space",
        props(
            format = "%e",
            description = "The day of the month. Space padded."
        )
    )]
    DaySpace,

    #[strum(
        serialize = "hour24",
        props(
            format = "%H",
            description = "The hour in a 24 hour clock. Zero padded."
        )
    )]
    Hour24,

    #[strum(
        serialize = "hour12",
        props(
            format = "%I",
            description = "The hour in a 12 hour clock. Zero padded."
        )
    )]
    Hour12,

    #[strum(
        serialize = "minute",
        props(
            format = "%M",
            description = "The minute. Zero padded."
        )
    )]
    Minute,

    #[strum(serialize = "second",
        props(
            format = "%S",
            description = "The second. Zero padded."
        )
    )]
    Second,

    #[strum(
        serialize = "ampm",
        props(
            format = "%p",
            description = "Whether the time is in the AM or PM, uppercase."
        )
    )]
    AmPm,

    #[strum(
        serialize = "ampm_lower",
        props(
            format = "%P",
            description = "Whether the time is in the AM or PM, lowercase."
        )
    )]
    AmPmLower,

    #[strum(serialize = "weekday",
        props(
            format = "%A",
            description = "The full weekday."
        )
    )]
    Weekday,

    #[strum(
        serialize = "weekday_abbrev",
        props(
            format = "%a",
            description = "The abbreviated weekday."
        )
    )]
    WeekdayAbbrev,

    #[strum(
        serialize = "tz_iana",
        props(
            format = "%V",
            description = "An IANA time zone identifier, or `%z` if one doesn't exist."
        )
    )]
    TzIana,

    #[strum(
        serialize = "tz_abbrev",
        props(
            format = "%Z",
            description = "A time zone abbreviation. Supported when formatting only."
        )
    )]
    TzAbbrev,

    #[strum(
        serialize = "offset",
        props(
            format = "%z",
            description = "A time zone offset in the format `[+-]HHMM[SS]`."
        )
    )]
    Offset,

    #[strum(
        serialize = "offset_colon",
        props(
            format = "%:z",
            description = "A time zone offset in the format `[+-]HH:MM[:SS]`."
        )
    )]
    OffsetColon,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use strum::{EnumProperty, VariantArray};

    use crate::context_key::{ContextKey, ContextKey::*};

    #[test]
    fn test_format() -> Result<()> {
        let datetime = jiff::Timestamp::from_microsecond(1724743664325609)?.intz("Asia/Tokyo")?;
        let test = |key: ContextKey, expected: &str| {
            assert_eq!(datetime.strftime(key.get_str("format").unwrap()).to_string(), expected);
        };

        test(Timestamp, "2024-08-27 16:27:44 (JST)");
        test(IsoDate, "2024-08-27");
        test(Clock, "16:27:44");
        test(Year, "2024");
        test(Year2Digit, "24");
        test(Month, "August");
        test(MonthAbbrev, "Aug");
        test(Month2Digit, "08");
        test(Day, "27");
        test(DaySpace, "27");
        test(Hour24, "16");
        test(Hour12, "04");
        test(Minute, "27");
        test(Second, "44");
        test(AmPm, "PM");
        test(AmPmLower, "pm");
        test(Weekday, "Tuesday");
        test(WeekdayAbbrev, "Tue");
        test(TzIana, "Asia/Tokyo");
        test(TzAbbrev, "JST");
        test(Offset, "+0900");
        test(OffsetColon, "+09:00");

        Ok(())
    }

    #[test]
    fn list_context_keys_for_documentation() -> Result<()> {
        let datetime = jiff::Timestamp::from_microsecond(1724261952503309)?.intz("Asia/Tokyo")?;

        println!("| Variable               | `strftime` Specifier     | Example                              | Description                                                                      |");
        println!("|------------------------|--------------------------|--------------------------------------|----------------------------------------------------------------------------------|");
        ContextKey::VARIANTS.iter().for_each(|key| {
            println!(
                "| {:22} | {:24} | {:36} | {:80} |",
                format!("`{{{{ {} }}}}`", key.as_ref()),
                if let Some(format) = key.get_str("format") {
                    format!("`{format}`")
                } else {
                    "(not available)".to_string()
                },
                if let Some(format) = key.get_str("format") {
                    format!("`{}`", datetime.strftime(format))
                } else {
                    format!("`{}`", key.get_str("example").unwrap_or("-"))
                },
                key.get_str("description").unwrap()
            );
        });

        Ok(())
    }
}

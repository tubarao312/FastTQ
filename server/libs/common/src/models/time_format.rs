use serde::Serializer;
use time::OffsetDateTime;

pub fn serialize_datetime<S>(dt: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format without the +00 prefix
    let s = dt
        .format(&time::format_description::well_known::Iso8601::DEFAULT)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s.trim_start_matches("+00"))
}

use std::fmt;

/// Format an array into a comma-delimited string.
pub fn comma_delimited<'a, T: fmt::Display + 'a>(f: &mut fmt::Formatter, mut iter: impl Iterator<Item=&'a T>) -> fmt::Result {
    if let Some(part) = iter.next() {
        fmt::Display::fmt(part, f)?;
    }
    for part in iter {
        f.write_str(", ")?;
        fmt::Display::fmt(part, f)?;
    }
    Ok(())
}

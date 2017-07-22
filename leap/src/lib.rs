
pub fn is_leap_year(year: i64) -> bool {
    match year % 4 {
        0 => match year % 400 {
            0 => true,
            _ => match year % 100 {
                0 => false,
                _ => true,
            },
        },
        _ => false,
    }
}

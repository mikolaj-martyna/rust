// 1.
fn is_leap(year: i16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

// 2.
fn days_in(month: i8, year: i16) -> i8 {
    const DAYS_IN_MONTH_COMMON: [i8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_IN_MONTH_LEAP: [i8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if is_leap(year) {
        DAYS_IN_MONTH_LEAP[month as usize - 1]
    } else {
        DAYS_IN_MONTH_COMMON[month as usize - 1]
    }
}

fn main() {
    // 1.
    let year = 2023;
    println!("Year {} is leap: {}", year, is_leap(year));

    // 2.
    let month = 2;
    let year = 2024;
    println!("{} days in month {} year {}", days_in(month, year), month, year);
}

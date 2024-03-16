// 1.
fn is_leap(year: i16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    // 1.
    let year = 2024;
    println!("Year {} is leap: {}", year, is_leap(year));
}

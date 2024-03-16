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

// 3.
fn to_fahrenheit(celsius: f32) -> f32 {
    32.0 + (9.0 / 5.0) * celsius
}

// 4.
fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

// 5.
fn time_between(hours_from: i8, minutes_from: i8, seconds_from: i8,
                hours_to: i8, minutes_to: i8, seconds_to: i8) -> [i8; 3] {
    let mut time_from = time_to_seconds(hours_from, minutes_from, seconds_from);
    let mut time_to = time_to_seconds(hours_to, minutes_to, seconds_to);

    if time_from > time_to {
        let temp = time_from;
        time_from = time_to;
        time_to = temp;
    }

    time_from_seconds(time_to - time_from)
}

fn time_to_seconds(hours: i8, minutes: i8, seconds: i8) -> i32 {
    hours as i32 * 3600 + minutes as i32 * 60 + seconds as i32
}

fn time_from_seconds(seconds: i32) -> [i8; 3] {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    [hours as i8, minutes as i8, (seconds % 60) as i8]
}

// 6.
fn factorial(n: i8) -> i128 {
    if n == 1 {
        return 1i128
    }

    factorial(n - 1) * n as i128
}

//  7.
fn digits_from_end(mut number: i128) {
    while number != 0 {
        print!("{}", number % 10);
        number /= 10;
    }

    println!();
}

// 8.
fn sum_of_digits(mut number: i128) -> i128 {
    let mut sum = 0;

    while number != 0 {
        sum += number % 10;
        number /= 10;
    }

    sum
}

fn main() {
    // 1.
    let year = 2023;
    println!("Year {} is leap: {}", year, is_leap(year));

    // 2.
    let month = 2;
    let year = 2024;
    println!("{} days in month {} year {}", days_in(month, year), month, year);

    // 3.
    let celsius = 22.3;
    println!("{}*C is {}*F", celsius, to_fahrenheit(celsius));

    // 4.
    let fahrenheit = 72.14;
    println!("{}*F is {}*C", fahrenheit, to_celsius(fahrenheit));

    // 5.
    let hours_from = 14;
    let minutes_from = 58;
    let seconds_from = 14;

    let hours_to = 17;
    let minutes_to = 0;
    let seconds_to = 9;

    let time_between = time_between(hours_from, minutes_from, seconds_from,
                                    hours_to, minutes_to, seconds_to);

    println!("Time from {:0>2}:{:0>2}:{:0>2} to {:0>2}:{:0>2}:{:0>2} is {:0>2}:{:0>2}:{:0>2}",
             hours_from, minutes_from, seconds_from,
             hours_to, minutes_to, seconds_to,
             time_between[0], time_between[1], time_between[2]
    );

    // 6.
    let n = 16;
    println!("Factorial of {} is {}", n, factorial(n));

    // 7.
    let number = 432891;
    print!("Number {} printed from the end is ", number);
    digits_from_end(number);


    // 8.
    let number = 12345;
    println!("Sum of digits of {} is {}", number, sum_of_digits(number))


}

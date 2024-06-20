fn showcase(name_input: Option<String>, surname_input: Option<String>) -> String {
    let mut name;
    let mut surname;

    match name_input {
        Some(x) => name = String::from(x),
        None => name = String::from("Jan")
    }

    name = String::from(name.chars().nth(0).unwrap()).to_uppercase() + ". ";

    match surname_input {
        Some(mut x) => {
            surname = format!("{}{}", x.remove(0).to_uppercase(), x.to_lowercase());
        }
        None => surname = String::from("Kowalski")
    }

    name + surname.as_str()
}

fn root(a: f32, b: f32, c: f32) -> (Option<f32>, Option<f32>) {
    let delta = b * b - 4f32 * a * c;

    if delta < 0.0 {
        (None, None)
    } else if delta == 0.0 {
        (Some((-1.0 * b) / (2.0 * a)), None)
    } else {
        (Some((-1.0 * b - delta.sqrt()) / (2.0 * a)), Some((-1.0 * b + delta.sqrt()) / (2.0 * a)))
    }
}

fn main() {
    let res = showcase(None, Some(String::from("smITh")));
    println!("{res}");

    let res = root(1.0, 2.0, -3.0);
    println!("{:?}", res);

    let res: Vec<_> = ('a'..='z').collect();
    println!("{:?}", res);

    let res: Vec<_> = (1..=10).map(|x| x * x).collect();
    println!("{:?}", res);

    let res: Vec<_> = (1..=10).map(|x| 2_i32.pow(x)).collect();
    println!("{:?}", res);

    let res: Vec<_> = (1..=20).map(|x| 1.0 / x as f32).collect();
    println!("{:?}", res);

    let res: Vec<_> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("{:?}", res);

    let res: Vec<_> = vec!["fjasdkl", "sad", "ab", "", "fsdafasfasdag"].into_iter().filter(|x| x.len() < 4).collect();
    println!("{:?}", res);

    let res: Vec<_> = vec!["fjsdkl", "sbc", "ab", "", "fsdafasfasdag"].into_iter().filter(|x| !x.contains('a')).collect();
    println!("{:?}", res);

    let res: Vec<_> = vec!["fj5dkl", "sb5c", "ab", "5242435345324", "", "fsdafasfasdag"].into_iter().filter(|x| x.contains(['1', '2', '3', '4', '5', '6', '7', '8', '9'])).collect();
    println!("{:?}", res);

    let res: Vec<_> = vec!["abc", "fjadkls", "picto"].into_iter().map(|x| x.chars().rev().collect::<String>()).collect();
    println!("{:?}", res);
}

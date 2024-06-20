#[derive(Debug)]
pub struct Fraction {
    numerator: u32,
    denominator: u32
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be 0!");
        }

        Self {
            numerator,
            denominator
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.denominator
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Self {
        Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator
        }
    }
}

impl From<&str> for Fraction {
    fn from(value: &str) -> Self {
        let str_split: Vec<_> = value.split("/").collect();

        Self {
            numerator: str_split[0].parse::<u32>().unwrap(),
            denominator: str_split[1].parse::<u32>().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Fraction;

    #[test]
    fn test_new() {
        let x = Fraction::new(1, 2);

        assert_eq!(x.numerator(), 1);
        assert_eq!(x.denominator(), 2);
    }

    #[test]
    fn test_as_f64() {
        let x = Fraction::new(1, 2);

        assert_eq!(x.as_f64(), 0.5);
    }

    #[test]
    #[should_panic]
    fn test_zero_denominator() {
        let _ = Fraction::new(1, 0);
    }

    #[test]
    fn test_from_str() {
        let x = Fraction::from("5/7");

        assert_eq!(x.numerator(), 5);
        assert_eq!(x.denominator(), 7);
    }
}

fn main() {
    let fraction_one = Fraction::new(1, 2);
    let fraction_two = Fraction::new(1, 3);

    println!("{:?}", fraction_one * fraction_two);
    println!("{:?}", Fraction::from("5/7"));
}

pub trait Luhn {
    fn luhn(self) -> bool;
}
pub trait Digits {
    fn digits(self) -> Result<Vec<u64>, String>;
}

fn remove_whitespace(input: &str) -> Result<String, String> {
    let trimmed = input.replace(" ", "");
    if trimmed.len() < 2 {
        Err("Invalid".to_string())
    } else {
        Ok(trimmed)
    }
}
fn dbl2nd(digits: Vec<u64>) -> Vec<u64> {
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(ix, &num)| if ix % 2 == 0 { num } else { num * 2 })
        .map(|num| if num > 9 { num - 9 } else { num })
        .collect::<Vec<u64>>()
}
fn sum_digits(digits: Vec<u64>) -> u64 {
    digits.iter().sum()
}
fn is_div_by_10(num: u64) -> bool {
    println!("{} {}", num, num % 10);
    num % 10 == 0
}

impl Digits for String {
    fn digits(self) -> Result<Vec<u64>, String> {
        self.chars()
            .map(|c| c.to_digit(10).map(u64::from))
            .collect::<Option<Vec<u64>>>()
            .ok_or("Invalid Digits".to_string())
    }
}

fn x_base_n_recurse(n: u64, base: u64, xs: &mut Vec<u64>) {
    if n >= base {
        x_base_n_recurse(n / base, base, xs);
    }
    xs.push(n % base);
}

impl Digits for u64 {
    fn digits(self) -> Result<Vec<u64>, String> {
        let mut xs = Vec::new();
        x_base_n_recurse(self, 10, &mut xs);
        println!(" Digits for u64 vec: {:?}", xs);
        if xs.len() <= 1 {
            Err("Bad".to_string())
        } else {
            Ok(xs)
        }
    }
}
impl Luhn for String {
    fn luhn(self) -> bool {
        remove_whitespace(&self)
            .and_then(Digits::digits)
            .map(dbl2nd)
            .map(sum_digits)
            .map(is_div_by_10)
            .unwrap_or(false)
    }
}
impl Luhn for u64 {
    fn luhn(self) -> bool {
        Digits::digits(self)
            .map(dbl2nd)
            .map(sum_digits)
            .map(is_div_by_10)
            .unwrap_or(false)
    }
}

fn main() {
    let ex1 = "4539 1488 0343 6467";
    let ex2 = 4539148803436467;
    /*
    let mut ttt = ex1.chars().map(|c| c.to_digit(10).map(u64::from));

    println!("ex1 digits: {:?} ", &ttt.next());
    println!("ex1 digits: {:?} ", &ttt.next());
    println!("ex1 digits: {:?} ", &ttt.next());
    println!("ex1 digits: {:?} ", &ttt.next());
    println!("ex1 digits: {:?} ", &ttt.next());
    println!("ex1 digits: {:?} ", &ttt.next());

    let res = Digits::digits(ex1.to_string());
    println!("ex1 digits: {:?} for '{}'", res, ex1);
    */

    //debug digits
    //let res = Digits::digits(ex1.to_string());
    //println!("ex1 digits: {:?} for '{}'", res, ex1);

    //println!("ex2 digits: {:?} for '{}'", Digits::digits(ex2), ex2);

    //true
    println!("ex1(true): {} for '{}'", Luhn::luhn(ex1.to_string()), ex1);
    println!("ex2(true): {} for '{}'", Luhn::luhn(ex2), ex2);

    /*
    //false
    let ex3 = "8273 1232 7352 0569";
    println!("ex3(false): {} for '{}'", Luhn::luhn(ex3.to_string()), ex3);
    let ex4 = 8273123273520569;
    println!("ex4(false): {} for '{}'", Luhn::luhn(ex4), ex4);

    let ex5 = "&% 1232 7352 0569";
    println!("ex5(false): {} for '{}'", Luhn::luhn(ex5.to_string()), ex5);
    */
}

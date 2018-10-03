/*
//Luhn trait
pub trait Luhn {
    fn luhn(self) -> bool;
}
impl Luhn for String {
    fn luhn(self) -> bool {
        let s = remove_whitespace(&self);
        let d = Digits::digits(s);
        if d.len() < 16 {
            return false;
        }
        let d2 = dbl2nd(d);
        let d3 = sum_digits(d2);
        return is_div_by_10(d3);
    }
}
impl Luhn for u64 {
    fn luhn(self) -> bool {
        let d = Digits::digits(self);
        if d.len() < 16 {
            return false;
        }
        let d2 = dbl2nd(d);
        let d3 = sum_digits(d2);
        return is_div_by_10(d3);
    }
}

//Digits trait
pub trait Digits {
    fn digits(self) -> Vec<u64>;
}
impl Digits for String {
    fn digits(self) -> Vec<u64> {
        let mut num = vec![];
        for c in self.chars() {
            let val = c.to_string().parse::<u64>();
            match val {
                Ok(v) => num.push(v),
                //Err(e) => println!("{}", e),
                Err(_) => (),
            }
        }
        return num;
    }
}
impl Digits for u64 {
    fn digits(self) -> Vec<u64> {
        let mut xs = Vec::new();
        x_base_n_recurse(self, 10, &mut xs);
        return xs;
    }
}

fn remove_whitespace(input: &str) -> String {
    return input.replace(" ", "");
}
fn x_base_n_recurse(n: u64, base: u64, xs: &mut Vec<u64>) {
    if n >= base {
        x_base_n_recurse(n / base, base, xs);
    }
    return xs.push(n % base);
}
fn dbl2nd(digits: Vec<u64>) -> Vec<u64> {
    return digits
        .iter()
        .rev()
        .enumerate()
        .map(|(ix, &num)| if ix % 2 == 0 { num } else { num * 2 })
        .map(|num| if num > 9 { num - 9 } else { num })
        .collect::<Vec<u64>>();
}
fn sum_digits(digits: Vec<u64>) -> u64 {
    return digits.iter().sum();
}
fn is_div_by_10(num: u64) -> bool {
    return num % 10 == 0;
}

fn main() {
    let ex1 = "4539 1488 0343 6467";
    let ex2 = 4539148803436467;
    //true
    println!("ex1(true): {} for '{}'", Luhn::luhn(ex1.to_string()), ex1);
    println!("ex2(true): {} for '{}'", Luhn::luhn(ex2), ex2);

    //false
    let ex3 = "8273 1232 7352 0569";
    let ex4 = 8273123273520569;
    println!("ex3(false): {} for '{}'", Luhn::luhn(ex3.to_string()), ex3);
    println!("ex4(false): {} for '{}'", Luhn::luhn(ex4), ex4);

    let ex5 = "^*( 1232 7352 0569";
    println!("ex5(false): {} for '{}'", Luhn::luhn(ex5.to_string()), ex5);
}
*/

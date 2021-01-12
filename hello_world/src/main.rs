use regex::Regex;
fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Matched ? {}", re.is_match("2012-1-1"));
    println!("Matched ? {}", re.is_match("2021-01-11"));
    /*
     *
     *     let mut v = Vec::new();
     *     let t: Vec<i32> = Vec::new();
     *     v.push(&t);
     */
}

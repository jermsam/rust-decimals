use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
fn main() {
    let number = dec!(-1.23);
    println!("Hello, world! {}",number);

    let number_two = format!("{:.2}", 1.2399);
    println!("Hello, world! {}", number_two);
    println!("{:.2}", 1.2399);

    let pi = Decimal::from_str("3.1415926535897932384626433832").unwrap();
    let pi = pi.round_dp(2);
    let k = Decimal::new(20,1);
    let k = k.round();
    let radius = Decimal::new(43,2);
    let circumference = k * pi * radius;
    println!("circumference = 2piR = {} x {} x {} = {}",k,pi,radius, circumference.round_dp(2))
}

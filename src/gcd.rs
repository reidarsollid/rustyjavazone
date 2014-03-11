/******
 * This file is generated with RustyCage
 */
fn gcd(a: int,b: int) -> int {
  match b {
	  0 =>  return a,
	  _ => return gcd(b,(a%b)),
  }
}

#[main]
fn main() {
  println!("GCD number {}", gcd(259,111));
}

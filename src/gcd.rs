fn gcd(a: int,b: int) -> int {
  match b {
	0 =>  return a,
	_ => return gcd(b,(a%b)),
  }
}

fn main() {
  io::println(#fmt("GCD number %i", gcd(259,111)));
}

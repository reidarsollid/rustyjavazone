/*This file is generated with RustyCage*/
use std;
import print = io:: println;

fn main() {
  let port = comm:: port::<str>();
  let chan = comm:: chan::<str>(port);
  do task:: spawn || { 
    let result = compute_some();
    comm:: send(chan, result);
  }
  let first_result = compute_another();
  let second_result = comm:: recv(port);
  print(#fmt("First, %s", first_result));
  print(#fmt("Second, %s", second_result));
}

fn compute_some() -> str {
  ret "Some computation";
}

fn compute_another() -> str {
  ret "Another computation";
}

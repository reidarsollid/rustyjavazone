/******
 * This file is generated with RustyCage
 */
fn run() {
	//Spawn a new task
  spawn(hello);
}

#[main]
fn hello() {
  println("Hello world");
}

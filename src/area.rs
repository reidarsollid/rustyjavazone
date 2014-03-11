/******
 * This file is generated with RustyCage
 */
use std::f64;

enum message {
    rectangle(int, int),
    circle(f64),
    end
}

fn area(from: Port<message>) {
    
    loop{
		let  msg:message  = from.recv();
    match msg {
      rectangle(width, height) => {
      let result = width * height;						
      println!("Result {}", result);
				
    }
      circle(radius) => {
      let result = f64::consts::PI * radius;
      println!("Result {}",result);
			
    }			
     end => {
      println("Bye");
			break;
    }			
  }
}
}

#[main]
fn run(){
	let (port, chan): (Port<message>, Chan<message>) = Chan::new();
	spawn(proc() area(port));
	chan.send(rectangle(22,22));
	chan.send(circle(2.45));
	chan.send(end);
}
/*This file is generated with RustyCage*/
use std;

import comm::port;
import comm::recv;
import io::println;

const PI: float = 3.14159265;

enum message {
    rectangle(int, int),
    circle(float),
    end
}

fn area(from: port<message>) {
    let msg = recv(from);
    loop {	
        alt msg {
          rectangle(width, height) {
            let result = width * height;
            println(#fmt("%i",result));		
          }
          circle(radius) {
            let result = PI * radius;
            println(#fmt("%f",result));
          }			
          end {
            println("Bye");
          }			
        }
    }
}
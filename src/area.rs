/*This file is generated with RustyCage*/
use std;

import pipes::{stream, chan, port};
import io::println;

const PI: float = 3.14159265;

enum message {
    rectangle(int, int),
    circle(float),
    end
}

fn area(from: port<message>) {
    let msg = from.recv();
    loop {	
        match msg {
          rectangle(width, height) => {
            let result = width * height;
            println(#fmt("%i",result));		
          }
          circle(radius) => {
            let result = PI * radius;
            println(#fmt("%f",result));
          }			
          end => {
            println("Bye");
          }			
        }
    }
}
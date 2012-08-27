/*This file is generated with RustyCage*/
fn main() {
    io::println("Hello world");
    receive(text(~"Hello match"));
    receive(number(22));
    receive(end);
}

enum message {
    end,
    text(~str),
    number(int)
}

fn receive(msg: message) {
    match msg {
      end => {
        io::println("The end");
      }
      text(aText) => {
        io::println(aText);
      }
      number(num) => {
        io::println(#fmt("%d", num));
      }
    }
}

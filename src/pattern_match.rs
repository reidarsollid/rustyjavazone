/*This file is generated with RustyCage*/
fn main() {
    io::println("Hello world");
    receive(Text(~"Hello match"));
    receive(Number(22));
    receive(End);
}

enum Message {
    End,
    Text(~str),
    Number(int)
}

fn receive(msg: Message) {
    match msg {
      End => {
        io::println("The end");
      }
      Text(aText) => {
        io::println(aText);
      }
      Number(num) => {
        io::println(#fmt("%d", num));
      }
    }
}

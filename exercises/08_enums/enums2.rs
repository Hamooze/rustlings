#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u64, u64, u64),
    Quit,
}

impl Message {
    fn call(&self) {
        match self {
            Message::Resize { width, height } => {
                println!("Resizing to width: {}, height: {}", width, height);
            }
            Message::Move(Point { x, y }) => {
                println!("Moving to x: {}, y: {}", x, y);
            }
            Message::Echo(text) => {
                println!("Echoing: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to red: {}, green: {}, blue: {}", r, g, b);
            }
            Message::Quit => {
                println!("Quitting");
            }
        }
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

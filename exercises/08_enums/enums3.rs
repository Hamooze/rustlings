#[derive(Debug, Clone)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

#[derive(Debug)]
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
        println!("Moved to position: ({}, {})", self.position.x, self.position.y);
    }

    fn echo(&mut self, message: String) {
        self.message = message;
    }

    fn change_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = (r, g, b);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process_message(&mut self, message: Message) {
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(text) => self.echo(text),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    let mut state = State {
        width: 800,
        height: 600,
        position: Point { x: 0, y: 0 },
        message: String::new(),
        color: (0, 0, 0),
        quit: false,
    };

    let messages = [
        Message::Resize { width: 1024, height: 768 },
        Message::Move(Point { x: 100, y: 200 }),
        Message::Echo(String::from("Hello, world!")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for message in &messages {
        state.process_message(message.clone());
    }

    println!("State: {:?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::new(),
            color: (0, 0, 0),
            quit: false,
        };

        state.process_message(Message::Resize { width: 10, height: 30 });
        state.process_message(Message::Move(Point { x: 10, y: 15 }));
        state.process_message(Message::Echo(String::from("Hello world!")));
        state.process_message(Message::ChangeColor(255, 0, 255));
        state.process_message(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}

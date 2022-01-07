use std::fmt;

enum MouseButton {
    Left,
    Right,
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MouseButton::Left => write!(f, "Left"),
            MouseButton::Right => write!(f, "Right"),
        }
    }
}

enum WindowEvent {
    Load,
    Show,
    KeyPress(char),
    MouseDown { x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    MouseUp { x: i32, y: i32 },
    Click { button: MouseButton, times: u32 },
    TextEntered(String),
}

fn log(event: WindowEvent) {
    match event {
        WindowEvent::Load => println!("Window loaded"),
        WindowEvent::Show => println!("Window shown"),
        WindowEvent::KeyPress(key) => println!("Key '{}' pressed", key),
        WindowEvent::MouseDown { x, y } => println!("Mouse button down at ({}, {})", x, y),
        WindowEvent::MouseMove { x, y } => println!("Mouse moved to ({}, {})", x, y),
        WindowEvent::MouseUp { x, y } => println!("Mouse button up at ({}, {})", x, y),
        WindowEvent::Click { button, times, .. } => {
            println!("{} mouse button clicked {} time(s)", button, times)
        }
        WindowEvent::TextEntered(s) => println!("'{}' entered", s),
    }
}

fn main() {
    let events = [
        WindowEvent::Load,
        WindowEvent::Show,
        WindowEvent::KeyPress('q'),
        WindowEvent::MouseDown { x: 100, y: 200 },
        WindowEvent::MouseMove { x: 100, y: 200 },
        WindowEvent::MouseUp { x: 100, y: 200 },
        WindowEvent::Click {
            button: MouseButton::Left,
            times: 1,
        },
        WindowEvent::Click {
            button: MouseButton::Right,
            times: 2,
        },
        WindowEvent::TextEntered(String::from("Hi")),
    ];

    for event in events {
        log(event);
    }
}

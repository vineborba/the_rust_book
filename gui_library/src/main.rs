use gui_library::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 320,
                height: 40,
                options: vec![
                    String::from("option 1"),
                    String::from("option 2"),
                    String::from("option 3"),
                    String::from("option 4"),
                ],
            }),
            Box::new(Button {
                height: 40,
                width: 240,
                label: String::from("Confirm"),
            }),
        ],
    };

    screen.run();
}

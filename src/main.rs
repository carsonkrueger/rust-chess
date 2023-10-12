use druid::{
    widget::{Button, Flex, Label},
    AppLauncher, Data, Widget, WindowDesc,
};

fn main() {
    let window = WindowDesc::new(build_root());

    let initial_state = ScreenState {
        title: String::from("Main Menu"),
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Could not launch application");
}

#[derive(Clone, Data)]
struct ScreenState {
    title: String,
}

fn build_root() -> impl Widget<ScreenState> {
    let label = Label::new("Rust Chess");

    let play_btn = Button::new("Play").on_click(|_, _, _| println!("You clicked play"));

    let layout = Flex::column().with_child(label).with_child(play_btn);

    layout
}

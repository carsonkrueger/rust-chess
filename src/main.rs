use druid::{
    widget::{Align, Button, Flex, Image, Label},
    AppLauncher, Data, ImageBuf, Widget, WindowDesc,
};
use image::open;

fn main() {
    let window = WindowDesc::new(build_board());

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

fn build_board() -> impl Widget<ScreenState> {
    let board_file_path: String = String::from("assets/chessboard.jpg");

    let img = open(board_file_path).unwrap();
    let w = img.width();
    let h = img.height();

    let img_slc = img.as_bytes();
    let arc_slc: std::sync::Arc<[u8]> = std::sync::Arc::from(img_slc);

    let img_buf = ImageBuf::from_raw(
        arc_slc,
        druid::piet::ImageFormat::Rgb,
        w as usize,
        h as usize,
    );

    Image::new(img_buf)
}

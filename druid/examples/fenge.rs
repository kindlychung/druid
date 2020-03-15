use druid::{
    widget::{FillStrat, Flex, Image, ImageData, WidgetExt, Stack, Zoom},
    AppLauncher, Widget, WindowDesc,
};
use druid::widget::Align;

fn ui_builder() -> impl Widget<u8> {
    let png_data = ImageData::from_file("examples/dog.jpg").unwrap();
    let img0 = Image::new(png_data.clone()).fill_mode(FillStrat::Fill).fix_width(500.).center();
    let img1= Image::new(png_data.clone()).fill_mode(FillStrat::ScaleDown).fix_width(200.).center();
    let root = Zoom::new(Stack::new()
        .with_child(img0)
        .with_child(img1)
    );
    root
}

#[cfg(feature = "image")]
fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u8;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}



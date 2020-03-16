use druid::widget::{Align, Button, ExternalImage, ImageDataProvider, Painter, WithImageData, TwoLayerRgba};
use druid::{
    widget::{FillStrat, Flex, Image, ImageData, Stack, WidgetExt, Zoom},
    AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WindowDesc,
};
use image::{ColorType, DynamicImage, GenericImage, Rgba, RgbaImage, };
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    img: TwoLayerRgba,
}


#[cfg(feature = "image")]
fn main() {
    let main_window = WindowDesc::new(ui_builder).window_size((500., 500.));
    let dog_data = image::open("examples/dog.jpg").unwrap().to_rgba();
    let img = TwoLayerRgba::from_bottom(dog_data);
    fn ui_builder() -> impl Widget<AppData> {
        Flex::column()
            .with_child(
                Painter::new(Rgba([33, 55, 55, 32])).lens(AppData::img),
                1.,
            )
    }
    AppLauncher::with_window(main_window)
        .launch(AppData {
            img
        })
        .expect("launch failed");
}

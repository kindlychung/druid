#![feature(in_band_lifetimes)]

use druid::{widget::{FillStrat, Flex, Image, ImageData, WidgetExt, Stack, Zoom}, Data, Lens, AppLauncher, Widget, WindowDesc, EventCtx, LifeCycle, PaintCtx, LifeCycleCtx, BoxConstraints, Size, LayoutCtx, Event, Env, UpdateCtx};
use druid::widget::{Align, ExternalImage, Button, ImageDataProvider, Painter};
use image::{DynamicImage, GenericImage, ColorType, Rgba};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    data: Arc<ImageData>,
}


#[cfg(feature = "image")]
fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let img_data = ImageData::from_dynamic_image(DynamicImage::new_rgba8(500, 500));
    let data = Arc::new(img_data);
    fn ui_builder() -> impl Widget<AppData> {
        Flex::column()
            .with_child(
                Painter::new().lens(AppData::data), 1.,
            )
    }
    AppLauncher::with_window(main_window)
        .launch(AppData {data})
        .expect("launch failed");
}

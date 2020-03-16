use druid::{widget::{FillStrat, Flex, Image, ImageData, WidgetExt, Stack, Zoom}, Data, Lens, AppLauncher, Widget, WindowDesc, EventCtx, LifeCycle, PaintCtx, LifeCycleCtx, BoxConstraints, Size, LayoutCtx, Event, Env, UpdateCtx};
use druid::widget::{Align, ExternalImage, Button, ImageDataProvider};
use image::{DynamicImage, GenericImage, ColorType, Rgba};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    toggle: Toggle,
}

#[derive(Clone, Data, Lens)]
struct Toggle {
    dog: bool,
    data: Arc<ImageData>,
}

impl Toggle{
    pub fn set_img(&mut self, img: ImageData) {
        *Arc::make_mut(&mut self.data) = img
    }
}

impl ImageDataProvider for Toggle {
    fn img(&self) -> &ImageData {
        self.data.as_ref()
    }

    fn img_mut(&mut self) -> &mut ImageData {
        Arc::make_mut(&mut self.data)
    }
}

#[cfg(feature = "image")]
fn main() {
    let main_window = WindowDesc::new(ui_builder);
    let img_data = ImageData::from_file("examples/PicWithAlpha.png").unwrap();
    let data = Arc::new(img_data);
    fn ui_builder() -> impl Widget<AppData> {
        Flex::column()
            .with_child(
                ExternalImage::new().lens(AppData::toggle), 1.,
            )
            .with_child(
                Button::new("Change image", |ctx, data: &mut Toggle, _env| {
                    // Arc::make_mut(data).from(dog_data);
                    // *(Arc::make_mut(&mut data.data)) = ImageData::from_file("examples/dog.jpg").unwrap();
                    // *(Arc::make_mut(data)) = ImageData::from_file("examples/dog.jpg").unwrap();
                    if data.dog {
                        data.set_img(ImageData::from_file("examples/PicWithAlpha.png").unwrap());
                        data.dog = false;
                    } else {
                        data.set_img(ImageData::from_file("examples/dog.jpg").unwrap());
                        data.dog = true;
                    }
                    ctx.request_layout();
                    ctx.request_paint();
                }).lens(AppData::toggle),
                0.,
            )
    }
    AppLauncher::with_window(main_window)
        .launch(AppData { toggle: Toggle{dog: false, data} })
        .expect("launch failed");
}

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
    show_img1: bool,
    data: Arc<ImageData>,
    img0: Arc<ImageData>,
    img1: Arc<ImageData>,
}

impl Toggle {
    pub fn set_img0(&mut self) {
        let new_img;
        {
            new_img = self.img0.as_ref().clone();
        }
        *Arc::make_mut(&mut self.data) = new_img
    }
    pub fn set_img1(&mut self) {
        let new_img;
        {
            new_img = self.img1.as_ref().clone();
        }
        *Arc::make_mut(&mut self.data) = new_img
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
    let dog_data = ImageData::from_file("examples/dog.jpg").unwrap();
    let data = Arc::new(img_data);
    fn ui_builder() -> impl Widget<AppData> {
        Flex::column()
            .with_child(
                ExternalImage::new().lens(AppData::toggle), 1.,
            )
            .with_child(
                Button::new("Change image", |ctx, data: &mut Toggle, _env| {
                    if data.show_img1 {
                        data.set_img0();
                        data.show_img1 = false;
                    } else {
                        data.set_img1();
                        data.show_img1 = true;
                    }
                    ctx.request_layout();
                    ctx.request_paint();
                }).lens(AppData::toggle),
                0.,
            )
    }
    AppLauncher::with_window(main_window)
        .launch(AppData { toggle: Toggle { show_img1: false, data: data.clone(), img0: data.clone(), img1: Arc::new(dog_data) } })
        .expect("launch failed");
}

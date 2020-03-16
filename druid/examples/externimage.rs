use druid::{widget::{FillStrat, Flex, Image, ImageData, WidgetExt, Stack, Zoom}, Data, Lens, AppLauncher, Widget, WindowDesc, EventCtx, LifeCycle, PaintCtx, LifeCycleCtx, BoxConstraints, Size, LayoutCtx, Event, Env, UpdateCtx};
use druid::widget::{Align, ExternalImage, Button, ImageDataProvider, WithImageData, Painter};
use image::{DynamicImage, GenericImage, ColorType, Rgba, RgbaImage};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    toggle: Toggle,
}

#[derive(Clone, Data, Lens)]
struct Toggle {
    show_img1: bool,
    data: Arc<RgbaImage>,
    img0: Arc<RgbaImage>,
    img1: Arc<RgbaImage>,
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

impl WithImageData for Toggle {
    fn width(&self) -> u32 {
        WithImageData::width(self.data.as_ref())
    }

    fn height(&self) -> u32 {
        WithImageData::height(self.data.as_ref())
    }

    fn pixels(&self) -> Vec<u8> {
        WithImageData::pixels(self.data.as_ref())
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        Arc::make_mut(&mut self.data).put_pixel(x, y, pixel)
    }
}

#[cfg(feature = "image")]
fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((500., 500.));
    let img_data = image::open("examples/PicWithAlpha.png").unwrap().to_rgba();
    let dog_data = image::open("examples/dog.jpg").unwrap().to_rgba();
    let data = Arc::new(img_data);
    fn ui_builder() -> impl Widget<AppData> {
        Flex::column()
            .with_child(
                Painter::new(Rgba([33, 55, 55, 5])).lens(AppData::toggle), 1.,
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

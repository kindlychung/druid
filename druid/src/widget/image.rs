// Copyright 2020 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An Image widget.
//! Please consider using SVG and the SVG wideget as it scales much better.

use std::convert::AsRef;
use std::error::Error;
use std::path::Path;

use image;

use crate::{
    piet::{ImageFormat, InterpolationMode},
    widget::common::FillStrat,
    Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};
use crate::{Lens, LensExt};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Primitive, Rgba, RgbaImage};
use std::marker::PhantomData;
use std::sync::Arc;
use std::borrow::BorrowMut;

/// A widget that renders an Image
pub struct Image {
    image_data: ImageData,
    fill: FillStrat,
}

impl Image {
    /// Create an image drawing widget from `ImageData`.
    ///
    /// The Image will scale to fit its box constraints.
    pub fn new(image_data: ImageData) -> Self {
        Image {
            image_data,
            fill: FillStrat::default(),
        }
    }

    /// A builder-style method for specifying the fill strategy.
    pub fn fill_mode(mut self, mode: FillStrat) -> Self {
        self.fill = mode;
        self
    }

    /// Modify the widget's `FillStrat`.
    pub fn set_fill(&mut self, newfil: FillStrat) {
        self.fill = newfil;
    }
}

impl Widget<u8> for Image {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut u8, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &u8, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u8, _data: &u8, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u8,
        _env: &Env,
    ) -> Size {
        bc.debug_check("Image");

        if bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(self.image_data.get_size())
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &u8, _env: &Env) {
        // dbg!(paint_ctx.size());
        // dbg!(self.image_data.get_size());
        let offset_matrix = self
            .fill
            .affine_to_fill(paint_ctx.size(), self.image_data.get_size());
        dbg!(offset_matrix);

        // The ImageData's to_piet function does not clip to the image's size
        // CairoRenderContext is very like druids but with some extra goodies like clip
        if self.fill != FillStrat::Contain {
            let clip_rect = Rect::ZERO.with_size(paint_ctx.size());
            paint_ctx.clip(clip_rect);
        }
        self.image_data.to_piet(offset_matrix, paint_ctx);
    }
}

/// Stored Image data.
#[derive(Clone)]
pub struct ImageData {
    pub pixels: Vec<u8>,
    pub x_pixels: u32,
    pub y_pixels: u32,
}

impl ImageData {
    /// Create an empty Image
    pub fn empty() -> Self {
        ImageData {
            pixels: [].to_vec(),
            x_pixels: 0,
            y_pixels: 0,
        }
    }

    /// Load an image from a DynamicImage from the image crate
    pub fn from_dynamic_image(image_data: image::DynamicImage) -> ImageData {
        let rgb_image = image_data.to_rgba();
        let sizeofimage = rgb_image.dimensions();
        ImageData {
            pixels: rgb_image.to_vec(),
            x_pixels: sizeofimage.0,
            y_pixels: sizeofimage.1,
        }
    }

    /// Attempt to load an image from raw bytes.
    ///
    /// If the image crate can't decode an image from the data an error will be returned.
    pub fn from_data(raw_image: &[u8]) -> Result<Self, Box<dyn Error>> {
        let image_data = image::load_from_memory(raw_image).map_err(|e| e)?;
        Ok(ImageData::from_dynamic_image(image_data))
    }

    /// Attempt to load an image from the file at the provided path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let image_data = image::open(path).map_err(|e| e)?;
        Ok(ImageData::from_dynamic_image(image_data))
    }

    /// Get the size in pixels of the contained image.
    fn get_size(&self) -> Size {
        Size::new(self.x_pixels as f64, self.y_pixels as f64)
    }

    /// Convert ImageData into Piet draw instructions
    fn to_piet(&self, offset_matrix: Affine, paint_ctx: &mut PaintCtx) {
        paint_ctx
            .with_save(|ctx| {
                ctx.transform(offset_matrix);

                let im = ctx
                    .make_image(
                        self.x_pixels as usize,
                        self.y_pixels as usize,
                        &self.pixels,
                        ImageFormat::RgbaSeparate,
                    )
                    .unwrap();
                let rec = Rect::from_origin_size(
                    (0.0, 0.0),
                    (self.x_pixels as f64, self.y_pixels as f64),
                );
                ctx.draw_image(&im, rec, InterpolationMode::Bilinear);

                Ok(())
            })
            .unwrap();
    }
}

impl Default for ImageData {
    fn default() -> Self {
        ImageData::empty()
    }
}

pub trait ImageDataProvider: Data {
    fn img(&self) -> &ImageData;
    fn img_mut(&mut self) -> &mut ImageData;
}

impl Data for ImageData {
    fn same(&self, other: &Self) -> bool {
        self.pixels == other.pixels
            && self.x_pixels == other.x_pixels
            && self.y_pixels == other.y_pixels
    }
}

impl ImageDataProvider for ImageData {
    fn img(&self) -> &ImageData {
        self
    }
    fn img_mut(&mut self) -> &mut ImageData {
        self
    }
}

impl ImageDataProvider for Arc<ImageData> {
    fn img(&self) -> &ImageData {
        self.as_ref()
    }

    fn img_mut(&mut self) -> &mut ImageData {
        Arc::make_mut(self)
    }
}

pub trait WithImageData: Data {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn pixels(&self) -> Vec<u8>;
    fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>);
    fn set_pixel_with_transform(&mut self, x: f64, y: f64, pixel: Rgba<u8>, transform: &Affine) {
        let point = Point { x, y };
        let point = *transform * point;
        self.set_pixel(point.x as u32, point.y as u32, pixel);
    }
    fn fill_square_with_transform(
        &mut self,
        x: f64,
        y: f64,
        size: u32,
        pixel: Rgba<u8>,
        transform: &Affine,
    ) {
        let point = Point { x, y };
        let point = *transform * point;
        self.fill_square(point.x as u32, point.y as u32, size, pixel)
    }
    fn fill_square(&mut self, x: u32, y: u32, size: u32, pixel: Rgba<u8>) {
        let w = self.width();
        let h = self.height();
        if size == 1 {
            self.set_pixel(x, y, pixel)
        }
        let xmax = (x + size).min(w);
        let ymax = (y + size).min(h);
        for i in x..xmax {
            for j in y..ymax {
                self.set_pixel(i, j, pixel);
            }
        }
    }
    fn size(&self) -> Size {
        Size {
            width: self.width() as f64,
            height: self.height() as f64,
        }
    }
    fn to_piet(&self, offset_matrix: Affine, paint_ctx: &mut PaintCtx) {
        paint_ctx
            .with_save(|ctx| {
                ctx.transform(offset_matrix);
                let im = ctx
                    .make_image(
                        self.width() as usize,
                        self.height() as usize,
                        &self.pixels(),
                        ImageFormat::RgbaSeparate,
                    )
                    .unwrap();
                let rec =
                    Rect::from_origin_size((0.0, 0.0), (self.width() as f64, self.height() as f64));
                ctx.draw_image(&im, rec, InterpolationMode::Bilinear);
                Ok(())
            })
            .unwrap();
    }
}

impl WithImageData for RgbaImage {
    fn width(&self) -> u32 {
        GenericImageView::width(self)
    }

    fn height(&self) -> u32 {
        GenericImageView::height(self)
    }

    fn pixels(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        self.put_pixel(x, y, pixel)
    }
}

#[derive(Clone)]
pub struct TwoLayerRgba {
    top: Arc<RgbaImage>,
    bottom: Arc<RgbaImage>,
}

impl Data for TwoLayerRgba {
    fn same(&self, other: &Self) -> bool {
        self.top.same(&other.top) && self.bottom.same(&other.bottom)
    }
}

impl TwoLayerRgba {
    pub fn new(top: RgbaImage, bottom: RgbaImage) -> TwoLayerRgba {
        assert!(top.width() == bottom.width() && top.height() == bottom.height());
        TwoLayerRgba { top: Arc::new(top), bottom: Arc::new(bottom) }
    }
    pub fn from_bottom(bottom: RgbaImage) -> TwoLayerRgba {
        let top = DynamicImage::new_rgba8(bottom.width(), bottom.height()).to_rgba();
        TwoLayerRgba { top: Arc::new(top), bottom: Arc::new(bottom) }
    }
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        Arc::make_mut(&mut self.top).put_pixel(x, y, pixel);
    }
}

impl WithImageData for TwoLayerRgba {
    fn width(&self) -> u32 {
        self.top.width()
    }

    fn height(&self) -> u32 {
        self.top.height()
    }

    fn pixels(&self) -> Vec<u8> {
        self.bottom.to_vec()
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        self.put_pixel(x, y, pixel)
    }

    fn to_piet(&self, offset_matrix: Affine, paint_ctx: &mut PaintCtx) {
        paint_ctx
            .with_save(|ctx| {
                ctx.transform(offset_matrix);
                let im_top = ctx
                    .make_image(
                        self.width() as usize,
                        self.height() as usize,
                        &self.top.to_vec(),
                        ImageFormat::RgbaSeparate,
                    )
                    .unwrap();
                let im_bottom = ctx
                    .make_image(
                        self.width() as usize,
                        self.height() as usize,
                        &self.bottom.to_vec(),
                        ImageFormat::RgbaSeparate,
                    )
                    .unwrap();
                let rec =
                    Rect::from_origin_size((0.0, 0.0), (self.width() as f64, self.height() as f64));
                ctx.draw_image(&im_bottom, rec, InterpolationMode::Bilinear);
                ctx.draw_image(&im_top, rec, InterpolationMode::Bilinear);
                Ok(())
            })
            .unwrap();
    }
}



impl Data for RgbaImage {
    fn same(&self, other: &Self) -> bool {
        self.width() == other.width()
            && self.height() == other.height()
            && self.to_vec() == other.to_vec()
    }
}

pub struct ExternalImage<T>
where
    T: WithImageData,
{
    phantom: PhantomData<T>,
    fill: FillStrat,
}

impl<T: WithImageData> ExternalImage<T> {
    pub fn new() -> Self {
        ExternalImage {
            fill: FillStrat::default(),
            phantom: PhantomData,
        }
    }

    pub fn with_fill_mode(mut self, mode: FillStrat) -> Self {
        self.fill = mode;
        self
    }

    pub fn set_fill(&mut self, newfil: FillStrat) {
        self.fill = newfil;
    }
}

impl<T: WithImageData> Widget<T> for ExternalImage<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        _env: &Env,
    ) -> Size {
        bc.debug_check("ExternalImage");
        if bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(data.size())
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, _env: &Env) {
        let offset_matrix = self.fill.affine_to_fill(paint_ctx.size(), data.size());
        // The ImageData's to_piet function does not clip to the image's size
        // CairoRenderContext is very like druids but with some extra goodies like clip
        if self.fill != FillStrat::Contain {
            let clip_rect = Rect::ZERO.with_size(paint_ctx.size());
            paint_ctx.clip(clip_rect);
        }
        data.to_piet(offset_matrix, paint_ctx);
    }
}

pub struct Painter<T> {
    phantom: PhantomData<T>,
    fill: FillStrat,
    painting: bool,
    paint_color: Rgba<u8>,
    transform: Affine,
}

impl<T> Painter<T> {
    pub fn new(color: Rgba<u8>) -> Self {
        Painter {
            phantom: PhantomData,
            fill: Default::default(),
            painting: false,
            paint_color: color,
            transform: Affine::default(),
        }
    }

    /// A builder-style method for specifying the fill strategy.
    pub fn with_fill_mode(mut self, mode: FillStrat) -> Self {
        self.fill = mode;
        self
    }

    /// Modify the widget's `FillStrat`.
    pub fn set_fill(&mut self, newfil: FillStrat) {
        self.fill = newfil;
    }
}

impl<T: WithImageData> Widget<T> for Painter<T> {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(e) => {
                data.fill_square_with_transform(
                    e.pos.x,
                    e.pos.y,
                    10,
                    self.paint_color,
                    &self.transform,
                );
                self.painting = true;
            }
            Event::MouseUp(e) => {
                self.painting = false;
            }
            Event::MouseMoved(e) => {
                if self.painting {
                    data.fill_square_with_transform(
                        e.pos.x,
                        e.pos.y,
                        10,
                        self.paint_color,
                        &self.transform,
                    );
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, _env: &Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        _env: &Env,
    ) -> Size {
        bc.debug_check("ExternalImage");
        if bc.is_width_bounded() {
            bc.max()
        } else {
            bc.constrain(data.size())
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, _env: &Env) {
        // let paint_ctx_size: Size = paint_ctx.size();
        // let data_size = data.size();
        // dbg!(paint_ctx_size);
        // dbg!(data_size);
        let offset_matrix = self.fill.affine_to_fill(paint_ctx.size(), data.size());
        self.transform = offset_matrix.inverse();
        // The ImageData's to_piet function does not clip to the image's size
        // CairoRenderContext is very like druids but with some extra goodies like clip
        if self.fill != FillStrat::Contain {
            let clip_rect = Rect::ZERO.with_size(paint_ctx.size());
            paint_ctx.clip(clip_rect);
        }
        data.to_piet(offset_matrix, paint_ctx);
    }
}

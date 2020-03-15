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

use crate::{Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget, BoxedWidget, WidgetPod, Point};

pub struct Zoom<T> {
    child: BoxedWidget<T>,
    scale: f64,
}

impl<T: Data> Zoom<T> {
    pub fn new(child: impl Widget<T> + 'static) -> Self {
        Zoom {
            child: WidgetPod::new(child).boxed(),
            scale: 1.,
        }
    }
}

impl<T: Data> Widget<T> for Zoom<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::Wheel(e) => {
                if e.mods.ctrl == true && e.mods.shift == false && e.mods.alt == false && e.mods.meta == false && e.delta.y != 0.0 {
                    if e.delta.y > 0. {
                        self.scale *= 1.2;
                    } else {
                        self.scale *= 0.8;
                    }
                    ctx.request_paint();
                }
            }
            _ => ()
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("Zoom");
        let loosened_bc = bc.loosen();
        let mut max_width: f64 = 0.;
        let mut max_height: f64 = 0.;
        let child_size: Size = self.child.layout(layout_ctx, &loosened_bc, data, env);
        max_width = max_width.max(child_size.width);
        max_height = max_height.max(child_size.height);
        let rect = Rect::from_origin_size(Point::ORIGIN, child_size);
        self.child.set_layout_rect(rect);
        let zoom_size = Size {
            width: max_width,
            height: max_height,
        };
        dbg!(zoom_size);
        zoom_size
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        paint_ctx.transform(Affine::scale(self.scale));
        self.child.paint_with_offset(paint_ctx, data, env)
    }
}




// Copyright 2019 The xi-editor Authors.
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

//! This example shows how to construct a basic layout.

use druid::widget::{Button, Flex, Label, SizedBox, WidgetExt, CrossAxisAlignment, MainAxisAlignment};
use druid::{AppLauncher, Color, LocalizedString, Widget, WindowDesc};

fn build_app() -> impl Widget<u32> {
    // Begin construction of vertical layout
    let mut col = Flex::column();
    for i in 0..5 {
        // Give a larger weight to one of the buttons for it to
        // occupy more space.
        // let weight = if i == 2 { 3.0 } else { 1.0 };
        let weight = 0.;
        let mut btn = Button::new(format!("Button #{}", i), Button::noop).fix_height(40.);
        if i == 2 { btn = btn.fix_width(1200.); }
        col.add_child(btn, weight);
    }

    col.main_axis_alignment(MainAxisAlignment::SpaceEvenly).cross_axis_alignment(CrossAxisAlignment::Center).debug_paint_layout()
}

fn main() {
    let window = WindowDesc::new(build_app)
        .title(LocalizedString::new("layout-demo-window-title").with_placeholder("Very flexible"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(0u32)
        .expect("launch failed");
}

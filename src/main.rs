// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example of how to setup a masonry application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
#![expect(elided_lifetimes_in_paths, reason = "Deferred: Noisy")]

use masonry::dpi::LogicalSize;
use masonry::widget::{Button, Flex, Portal, RootWidget, Textbox};
use winit::window::Window;

use app::Driver;
mod app;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

fn main() {
    let main_widget = Portal::new(
        Flex::column()
            .with_child(
                Flex::row()
                    .with_flex_child(Textbox::new(""), 1.0)
                    .with_child(Button::new("Add task")),
            )
            .with_spacer(VERTICAL_WIDGET_SPACING),
    );

    let window_size = LogicalSize::new(400.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("To-do list")
        .with_resizable(true)
        .with_min_inner_size(window_size);

    masonry::event_loop_runner::run(
        masonry::event_loop_runner::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_widget),
        Driver {
            next_task: String::new(),
        },
    )
    .unwrap();
}

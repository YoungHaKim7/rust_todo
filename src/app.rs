use masonry::{
    widget::{Flex, Label, Portal, RootWidget, TextArea, Textbox, WidgetMut},
    Action, AppDriver, DriverCtx, WidgetId,
};

pub struct Driver {
    pub next_task: String,
}

impl AppDriver for Driver {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) | Action::TextEntered() => {
                let mut root: WidgetMut<RootWidget<Portal<Flex>>> = ctx.get_root();
                let mut portal = RootWidget::child_mut(&mut root);
                let mut flex = Portal::child_mut(&mut portal);
                Flex::add_child(&mut flex, Label::new(self.next_task.clone()));

                let mut first_row = Flex::child_mut(&mut flex, 0).unwrap();
                let mut first_row = first_row.downcast::<Flex>();
                let mut textbox = Flex::child_mut(&mut first_row, 0).unwrap();
                let mut textbox = textbox.downcast::<Textbox>();
                let mut text_area = Textbox::text_mut(&mut textbox);
                TextArea::reset_text(&mut text_area, "");
            }
            Action::TextChanged(new_text) => {
                self.next_task = new_text.clone();
            }
            _ => {}
        }
    }
}

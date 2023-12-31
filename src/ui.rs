use druid::widget::prelude::*;
use druid::widget::{Flex, Label};
use druid::{Data, Lens, Widget, WidgetExt};
use std::sync::{Arc, Mutex};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub data_outside: Arc<Mutex<f64>>,
}

struct UpdatedLabelWidget {}

impl Widget<AppState> for UpdatedLabelWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, data: &mut AppState, _env: &Env) {     
        let data = data.data_outside.lock().unwrap();
        println!("event {}", *data)

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        println!("lifecycle")
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        println!("update")
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _data: &AppState, _env: &Env) {
    }
}

pub fn ui_builder() -> impl Widget<AppState> {
    let label = Label::new(|data: &Arc<Mutex<f64>>, _env: &_| {
            let v = data.lock().unwrap();
            println!("Data outside: {}", v);
            format!("Data outside: {}", v)
        })
        .with_text_size(32.0)
        .lens(AppState::data_outside)
        .padding(5.0);

    Flex::column().with_child(label).with_default_spacer().with_child(UpdatedLabelWidget {})
}

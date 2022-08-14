pub mod binder;
mod draw_wrap;
pub mod drawpanel;
pub mod elem;
pub mod panel;

// #[cfg(test)]
// mod tests {
//     use std::rc::Rc;

//     use geo::coord;
//     use mockall::predicate::*;
//     use mockall::*;

//     use crate::{
//         drawpanel::{self, Drawpanel},
//         elem::line::Line,
//         render::{Draw, EventType, Render},
//     };

//     static mut is_run: bool = false;

//     #[derive(Default)]
//     struct TestRender;

//     impl Render for TestRender {
//         fn bind_draw<F: FnMut()>(&mut self, mut trigger: F) {
//             trigger();
//             unsafe {
//                 is_run = true;
//             }
//         }

//         fn bind_event<F: FnMut(EventType, geo::Coordinate)>(&self, mut trigger: F) {
//             trigger(EventType::Push, coord! {x: 0., y:0.});
//         }

//         fn draw(&self) -> Box<dyn Draw> {
//             Box::new(TestDraw::default())
//         }
//     }

//     #[derive(Default)]
//     struct TestDraw;

//     impl Draw for TestDraw {
//         fn draw_line(&self, opts: crate::render::DrawLineOpts) {
//             todo!()
//         }

//         fn draw_rect(&self, opts: crate::render::DrawRectOpts) {
//             todo!()
//         }

//         fn draw_circle(&self, opts: crate::render::DrawCircleOpts) {
//             todo!()
//         }
//     }

//     #[test]
//     fn it_new_drawpanel_append_line() {
//         let drawpanel = Drawpanel::new(TestRender::default());
//         assert_eq!(drawpanel.elems().len(), 0);
//         let mut drawpanel = drawpanel;
//         drawpanel.append(Line::default());
//         assert_eq!(drawpanel.elems().len(), 1);
//         unsafe {
//             assert_eq!(is_run, true);
//         }
//     }
// }

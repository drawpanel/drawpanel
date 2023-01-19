use std::{borrow::Borrow, cell::RefCell, io, rc::Rc};

use drawpanel_bind_fltk::FltkBinder;
use drawpanel_core::{
    drawpanel::{Drawpanel, Mode},
    elem::{line::Line, pen::Pen, rect::Rect, text::Text},
};
use fltk::{
    app::Scheme,
    enums::{CallbackTrigger, Color, FrameType},
    prelude::*,
    *,
};

struct AppView {
    app: app::App,
    win: window::Window,
    drawpanel: Rc<RefCell<Drawpanel>>,
}

impl AppView {
    fn new() -> Self {
        let app = app::App::default().with_scheme(Scheme::Base);
        let mut win = window::Window::default().with_size(700, 600);

        let mut root_panel = group::Flex::new(0, 0, 700, 600, None).row();

        let mut left_panel = group::Flex::default().column();
        let mut select_btn = button::Button::default().with_label("Select");
        let mut pen_btn = button::Button::default().with_label("Pen");
        let mut line_btn = button::Button::default().with_label("Line");
        let mut rect_btn = button::Button::default().with_label("Rect");
        let mut text_btn = button::Button::default().with_label("Text");
        let mut remove_btn = button::Button::default().with_label("Remove");
        let mut up_scale_btn = button::Button::default().with_label("UP");
        let mut down_scale_btn = button::Button::default().with_label("Down");
        let mut export_btn = button::Button::default().with_label("Export");
        let mut import_btn = button::Button::default().with_label("Import");
        let mut status_frm = frame::Frame::default();

        left_panel.set_size(&status_frm, 200);
        left_panel.set_pad(0);
        left_panel.end();

        let mut mid_panel = group::Flex::default().column();
        let mut elem_change_panel = group::Flex::default().row();

        elem_change_panel.set_pad(0);
        elem_change_panel.end();
        let mut draw_frm = frame::Frame::default();
        draw_frm.set_frame(FrameType::FlatBox);
        draw_frm.set_color(Color::White);

        mid_panel.set_pad(0);
        mid_panel.set_size(&elem_change_panel, 50);
        mid_panel.set_size(&draw_frm, 500);
        mid_panel.end();

        let mut right_panel = group::Flex::default().column();
        let mut tree_frm = frame::Frame::default();
        right_panel.set_size(&tree_frm, 400);
        right_panel.set_pad(0);
        right_panel.end();

        root_panel.set_pad(0);
        root_panel.set_size(&left_panel, 100);
        root_panel.set_size(&mid_panel, 500);
        root_panel.set_size(&right_panel, 100);
        root_panel.end();

        win.end();
        win.show();

        let dp_x = *&draw_frm.x() as f64;
        let dp_y = *&draw_frm.y() as f64;
        let dp_w = *&draw_frm.w() as f64;
        let dp_h = *&draw_frm.h() as f64;
        let drawpanel = Rc::new(RefCell::new(Drawpanel::new(
            FltkBinder::new(draw_frm, win.clone()),
            dp_x,
            dp_y,
            dp_w,
            dp_h,
        )));

        select_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            move |btn| {
                (*drawpanel).borrow_mut().set_mode(Mode::Select);
            }
        });

        pen_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            move |btn| {
                (*drawpanel)
                    .borrow_mut()
                    .set_mode(Mode::Creating(Some(Box::new(Pen::default()))));
            }
        });

        line_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            move |btn| {
                (*drawpanel)
                    .borrow_mut()
                    .set_mode(Mode::Creating(Some(Box::new(Line::default()))));
            }
        });

        rect_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            move |btn| {
                (*drawpanel)
                    .borrow_mut()
                    .set_mode(Mode::Creating(Some(Box::new(Rect::default()))));
            }
        });

        text_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            move |btn| {
                (*drawpanel)
                    .borrow_mut()
                    .set_mode(Mode::Creating(Some(Box::new(Text::default()))));
            }
        });

        remove_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            let mut win = win.clone();
            move |btn| {
                (*drawpanel).borrow_mut().set_mode(Mode::Deleting);
            }
        });

        up_scale_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            let mut win = win.clone();
            move |btn| {
                let scale = drawpanel.borrow_mut().scale();
                (*drawpanel).borrow_mut().set_scale(scale + 0.1, 350., 300.);
            }
        });

        down_scale_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            let mut win = win.clone();
            move |btn| {
                let scale = drawpanel.borrow_mut().scale();
                (*drawpanel).borrow_mut().set_scale(scale - 0.1, 350., 300.);
            }
        });
        let data = Rc::new(RefCell::new(String::from("")));
        export_btn.set_callback({
            let data = Rc::clone(&data);
            let drawpanel = Rc::clone(&drawpanel);
            let mut win = win.clone();
            move |btn| {
                *data.borrow_mut() = (*drawpanel).borrow().export();
                println!("export:{}", *data.borrow_mut());
            }
        });

        import_btn.set_callback({
            let drawpanel = Rc::clone(&drawpanel);
            let data = Rc::clone(&data);
            let mut win = win.clone();
            move |btn| {
                println!("import:{}", *data.borrow_mut());
                (*drawpanel)
                    .borrow_mut()
                    .import((*data.borrow_mut()).as_str());
            }
        });

        AppView {
            app,
            win,
            drawpanel,
        }
    }

    fn run(&mut self) {
        while self.app.wait() {}
    }
}

fn main() {
    AppView::new().run();
}

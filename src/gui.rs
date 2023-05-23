use crate::data::service::HouseService;

use fltk::{
    app::{self, channel, App, Receiver, Scheme, Sender},
    browser::HoldBrowser,
    button::Button,
    enums::{Color, Event, Font, FrameType},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
    window::Window,
};
use std::any::Any;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Action {
    Select,
    Filter,
    Create,
    Update,
    Delete,
    Change,
}

pub struct Gui {
    app: App,
    dao: HouseService,
    inputs: HashMap<String, Box<dyn Any>>,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    changed: bool,
}

impl Gui {
    pub fn new() -> Self {
        let app = App::default();
        let dao = HouseService::new();
        let inputs = HashMap::new();
        let (sender, receiver) = channel::<Action>();
        let changed = false;
        Gui {
            app,
            dao,

            inputs,
            sender,
            receiver,
            changed,
        }
    }

    fn build(&mut self) {
        app::set_scheme(Scheme::Gtk);
        //app::background(255, 255, 255);
        app::set_background_color(170, 189, 206);
        app::set_background2_color(200, 255, 200);
        //app::foreground(20, 20, 20);
        app::set_foreground_color(0, 0, 0);
        app::set_selection_color(255, 160, 63);
        app::set_inactive_color(130, 149, 166);
        app::set_font_size(16);
        app::set_visible_focus(false);

        let mut win = Window::default()
            .with_label("Wohnen - Schcriher")
            .with_size(900, 450)
            .center_screen();

        let margin_size = 10;
        let button_height = 36;

        let mut main = Flex::default_fill().row();
        main.set_margin(margin_size);

        // --- LEFT ---------------------------------------------

        let mut left = Flex::default().column();
        left.set_margin(margin_size);

        {
            let row = Flex::default().row();
            self.create_button("Nuevo", Action::Create);
            self.create_button("Filtrar", Action::Filter);
            row.end();
            left.set_size(&row, button_height);
        }

        {
            let mut selector = HoldBrowser::default();
            selector.set_selection_color(Color::from_hex(0x1b1b1b));
            selector.emit(self.sender, Action::Select);
            self.inputs.insert("list".to_string(), Box::new(selector));
        }

        left.end();

        // --- RIGHT --------------------------------------------

        let mut right = Flex::default().column();
        right.set_margin(margin_size);

        // https://docs.rs/fltk/latest/fltk/enums/struct.Font.html#method.set_font

        let mut title = Frame::default().with_label("Vivienda Seleccionada");
        title.set_label_font(Font::HelveticaBold);
        right.set_size(&title, button_height);

        let sep = Frame::default();
        right.set_size(&sep, 5);

        self.create_input("id", "Número de registro");
        self.create_input("kind", "Tipo de vivienda");
        self.create_input("street", "Calle");
        self.create_input("number", "Número");
        self.create_input("floor", "Piso");
        self.create_input("postcode", "Código postal");
        self.create_input("rooms", "Número de habitaciones");
        self.create_input("baths", "Número de baños");
        self.create_input("area", "Superficie total (m²)");

        let sep = Frame::default();
        right.set_size(&sep, 10);

        {
            let row = Flex::default().row();
            self.create_button("Borrar", Action::Delete);
            self.create_button("Guardar", Action::Update);
            row.end();
            right.set_size(&row, button_height);
        }

        right.end();

        // ------------------------------------------------------

        main.end();

        win.end();
        win.show();

        let icon = SvgImage::load("assets/icon.svg").unwrap();
        win.set_icon(Some(icon));
    }

    fn create_button(&self, caption: &str, action: Action) -> Button {
        let mut button = Button::default().with_label(caption);
        button.set_color(Color::from_rgb(225, 225, 225));
        button.emit(self.sender, action);
        button.handle(move |b, ev| match ev {
            Event::Enter => {
                b.set_color(Color::from_rgb(150, 150, 150));
                b.redraw();
                true
            }
            Event::Leave => {
                b.set_color(Color::from_rgb(225, 225, 225));
                b.redraw();
                true
            }
            _ => false,
        });
        button
    }

    fn create_input(&mut self, key: &str, text: &str) {
        let row = Flex::default().row();
        Frame::default().with_label(text);
        let obj: Box<dyn Any>;
        match key {
            "id" => {
                let mut input = Input::default();
                input.set_frame(FrameType::FlatBox);
                input.set_readonly(true);
                input.deactivate();
                obj = Box::new(input);
            }
            "kind" => {
                let mut choice = Choice::default();
                choice.emit(self.sender, Action::Change);
                choice.set_selection_color(Color::from_hex(0x1b1b1b));
                obj = Box::new(choice);
            }
            "street" => {
                let mut input = Input::default();
                input.emit(self.sender, Action::Change);
                obj = Box::new(input);
            }
            "area" => {
                let mut input = FloatInput::default();
                input.emit(self.sender, Action::Change);
                input.set_tooltip("Ingrese un número con o sin coma");
                obj = Box::new(input);
            }
            _ => {
                let mut input = IntInput::default();
                input.emit(self.sender, Action::Change);
                input.set_tooltip("Ingrese un número entero");
                obj = Box::new(input);
            }
        }
        row.end();
        self.inputs.insert(key.to_string(), obj);
    }

    pub fn run(&mut self) {
        self.build();

        // https://docs.rs/fltk/latest/fltk/prelude/trait.WidgetBase.html#method.from_dyn_widget

        if let Some(widget_boxed) = self.inputs.get_mut("list") {
            if let Some(widget) = widget_boxed.downcast_mut::<HoldBrowser>() {
                for n in 1..5 {
                    widget.add(&format!("Vivienda {n}"));
                }
            }
        }

        if let Some(widget_boxed) = self.inputs.get_mut("kind") {
            if let Some(widget) = widget_boxed.downcast_mut::<Choice>() {
                for n in 1..5 {
                    widget.add_choice(&format!("Vivienda Tipo {n}"));
                }
            }
        }

        while self.app.wait() {
            match self.receiver.recv() {
                Some(Action::Create) => {
                    println!("Create");
                }
                Some(Action::Update) => {
                    println!("Update");
                }
                Some(Action::Delete) => {
                    println!("Delete");
                }
                Some(Action::Select) => {
                    println!("Select");
                }
                Some(Action::Filter) => {
                    println!("Filter");
                }
                Some(Action::Change) => {
                    println!("Change");
                    self.changed = true;
                }
                None => {}
            }
        }
    }
}

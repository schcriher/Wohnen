use fltk::{
    app::{self, channel, App, Receiver, Scheme, Sender},
    browser::HoldBrowser,
    button::Button,
    enums::{Color, Event, FrameType},
    frame::Frame,
    group::Flex,
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
    dirty: bool,
    inputs: HashMap<String, Box<dyn Any>>,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
}

impl Gui {
    pub fn new() -> Self {
        let app = App::default();
        let dirty = false;
        let inputs = HashMap::new();
        let (sender, receiver) = channel::<Action>();
        Gui {
            app,
            dirty,
            inputs,
            sender,
            receiver,
        }
    }

    fn init_styles(&self) {
        app::set_scheme(Scheme::Gtk);
        app::set_background_color(170, 189, 206);
        app::set_background2_color(255, 255, 255);
        app::set_foreground_color(0, 0, 0);
        app::set_selection_color(255, 160, 63);
        app::set_inactive_color(130, 149, 166);
        app::set_font_size(16);
        //app::background(255, 255, 255);
        //app::foreground(20, 20, 20);
        //app::set_font_size(16);
        app::set_visible_focus(false);
    }

    fn build(&mut self) {
        self.init_styles();

        let mut win = Window::default()
            .with_label("Wohnen - Schcriher")
            .with_size(900, 500)
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

        let title = Frame::default().with_label("Vivienda Seleccionada");
        right.set_size(&title, button_height);

        Frame::default();

        self.create_input("id", "Número de registro");
        self.create_input("type", "Tipo de vivienda");
        self.create_input("street", "Calle");
        self.create_input("number", "Número");
        self.create_input("floor", "Piso");
        self.create_input("postcode", "Código postal");
        self.create_input("rooms", "Número de habitaciones");
        self.create_input("baths", "Número de baños");
        self.create_input("area", "Superficie total");

        Frame::default();

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

        //win.resizable(&main);
        win.end();
        win.show();
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
            "type" => {
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

        if let Some(widget_boxed) = self.inputs.get_mut("list") {
            if let Some(widget) = widget_boxed.downcast_mut::<HoldBrowser>() {
                for n in 1..5 {
                    widget.add(&format!("Vivienda {n}"));
                }
            }
        }

        if let Some(widget_boxed) = self.inputs.get_mut("type") {
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
                    self.dirty = true;
                }
                None => {}
            }
        }
    }
}

use super::{wrapper::Widget, BUTTON_HEIGHT, BUTTON_WIDTH, MARGIN_SIZE};
use crate::base::{Filter, Range, HOUSE_TYPES};

use std::collections::HashMap;

use fltk::{
    app::{self, channel, Receiver, Sender},
    button::Button,
    enums::{Color, Event, Font, FrameType, Key},
    frame::Frame,
    group::Flex,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
    window::DoubleWindow,
};

pub const ALL_TYPE: &str = "«Todos los tipos»";

pub struct FilterDialog {
    window: DoubleWindow,
    inputs: HashMap<String, Vec<Widget>>,
    sender: Sender<bool>,
    receiver: Receiver<bool>,
}

impl FilterDialog {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let w_min = 4 * MARGIN_SIZE + 2 * BUTTON_WIDTH;
        debug_assert!(w > w_min, "width({w}) > {w_min}");

        let (sender, receiver) = channel::<bool>();
        Self {
            window: DoubleWindow::new(x, y, w, h, None),
            inputs: HashMap::new(),
            sender,
            receiver,
        }
    }

    fn build(&mut self) {
        self.window.set_frame(FrameType::BorderBox);
        self.window.set_border(false);
        self.window.make_modal(true);
        self.window.begin();

        let mut main = Flex::default_fill().column();
        main.set_margin(2 * MARGIN_SIZE);

        let mut title = Frame::default().with_label("Seleccione los parámetros para filtrar las viviendas");
        title.set_label_font(Font::HelveticaBold);
        title.set_label_size(24);
        main.set_size(&title, BUTTON_HEIGHT);

        let mut explanation = Frame::default().with_label(&format!(
            "El texto se buscar por similitud y los números dentro del rango indicado\n\
             Si no se especifica un mínimo se asume 0 y si no se especifica un máximo se asumen todos"
        ));
        explanation.set_label_size(12);
        main.set_size(&explanation, BUTTON_HEIGHT);

        let sep = Frame::default();
        main.set_size(&sep, 16);

        let filters = Flex::default_fill().row();
        {
            let mut left = Flex::default().column();

            self.create_input("kind", "Tipo de vivienda");
            self.create_input("number", "Número");
            self.create_input("floor", "Piso");
            self.create_input("postcode", "Código postal");

            let text = self.set_text_min_max();
            left.set_size(&text, 12);

            left.end();
        }
        {
            let mut right = Flex::default().column();

            self.create_input("street", "Calle");
            self.create_input("rooms", "Habitaciones");
            self.create_input("baths", "Baños");
            self.create_input("area", "Superficie (m²)");

            let text = self.set_text_min_max();
            right.set_size(&text, 12);

            right.end();
        }
        filters.end();

        let sep = Frame::default();
        main.set_size(&sep, 16);

        {
            let buttons = Flex::default().row();

            Frame::default();
            self.create_button("Cancelar");
            self.create_button("Aceptar");
            Frame::default();

            buttons.end();
            main.set_size(&buttons, BUTTON_HEIGHT);
        }

        main.end();

        self.window.end();

        self.window.handle({
            let sender = self.sender.clone();
            move |w, ev| match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Enter | Key::KPEnter => {
                        sender.send(true);
                        w.hide();
                        true
                    }
                    Key::Escape => {
                        sender.send(false);
                        w.hide();
                        true
                    }
                    _ => false,
                },
                _ => false,
            }
        });

        self.window.set_callback({
            let sender = self.sender.clone();
            move |w| {
                if app::event() == Event::Close {
                    sender.send(false);
                    w.hide();
                }
            }
        });
    }

    fn set_text_min_max(&self) -> Flex {
        let row = Flex::default().row();
        Frame::default();
        Frame::default();
        Frame::default().with_label("Mínimo").set_label_size(10);
        Frame::default().with_label("Máximo").set_label_size(10);
        row.end();
        row
    }

    fn create_input(&mut self, key: &str, text: &str) {
        let row = Flex::default().row();
        Frame::default().with_label(text);
        let mut vec = Vec::new();
        match key {
            "kind" => {
                let mut kind = Choice::default();
                kind.set_selection_color(Color::from_hex(0x1b1b1b));
                vec.push(Widget::Choice(kind));
            }
            "street" => {
                let mut input = Input::default();
                input.set_tooltip("Ingrese la dirección exacta o aproximada");
                vec.push(Widget::TInput(input));
            }
            "area" => {
                let row = Flex::default().row();

                let mut input = FloatInput::default();
                input.set_tooltip("Desde (valor mínimo), valores decimales");
                vec.push(Widget::FInput(input));

                let mut input = FloatInput::default();
                input.set_tooltip("Hasta (valor máximo), valores decimales");
                vec.push(Widget::FInput(input));

                row.end();
            }
            _ => {
                let row = Flex::default().row();

                let mut input = IntInput::default();
                input.set_tooltip("Desde (valor mínimo), solo valores enteros");
                vec.push(Widget::IInput(input));

                let mut input = IntInput::default();
                input.set_tooltip("Hasta (valor máximo), solo valores enteros");
                vec.push(Widget::IInput(input));

                row.end();
            }
        }
        row.end();
        self.inputs.insert(key.to_string(), vec);
    }

    fn create_button(&self, caption: &str) {
        let mut button = Button::default().with_label(caption);
        button.set_color(Color::from_rgb(225, 225, 225));
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
        let signal = caption == "Aceptar";
        button.set_callback({
            let mut win = self.window.clone();
            let sender = self.sender.clone();
            move |_| {
                win.hide();
                sender.send(signal);
            }
        });
    }

    fn fill_kind(&mut self) {
        let widgets = self.inputs.get_mut("kind").unwrap();
        widgets[0].add(ALL_TYPE);
        for value in HOUSE_TYPES {
            widgets[0].add(value);
        }
        widgets[0].set("0");
    }

    fn get_value_i32(&self, key: &str) -> Range<i32> {
        let widgets = self.inputs.get(key).unwrap();
        Range {
            min: widgets[0].get().parse().unwrap_or(0),
            max: widgets[1].get().parse().unwrap_or(i32::MAX),
        }
    }

    fn get_value_f32(&self, key: &str) -> Range<f32> {
        let widgets = self.inputs.get(key).unwrap();
        Range {
            min: widgets[0].get().parse().unwrap_or(0.0),
            max: widgets[1].get().parse().unwrap_or(f32::MAX),
        }
    }

    fn get_filter(&self) -> Filter {
        let mut filter = Filter::default();

        let widgets = self.inputs.get("kind").unwrap();
        let index = widgets[0].get().parse::<usize>().unwrap();
        filter.kind.text = if index == 0 {
            "".to_string()
        } else {
            HOUSE_TYPES[index - 1].to_string()
        };

        let widgets = self.inputs.get("street").unwrap();
        filter.street.text = widgets[0].get();

        filter.number = self.get_value_i32("number");
        filter.floor = self.get_value_i32("floor");
        filter.postcode = self.get_value_i32("postcode");
        filter.rooms = self.get_value_i32("rooms");
        filter.baths = self.get_value_i32("baths");
        filter.area = self.get_value_f32("area");

        filter
    }

    pub fn run(&mut self) -> Option<Filter> {
        self.build();
        self.fill_kind();
        self.window.show();
        while self.window.shown() {
            app::wait();
        }
        if self.receiver.recv().unwrap_or(false) {
            Some(self.get_filter())
        } else {
            None
        }
    }
}

pub struct MDButton {
    text: String,
    value: i32,
}

impl MDButton {
    pub fn new(text: &str, value: i32) -> Self {
        Self {
            text: text.to_string(),
            value,
        }
    }
}

pub struct MessageDialog {
    window: DoubleWindow,
    sender: Sender<i32>,
    receiver: Receiver<i32>,
    title: String,
    message: String,
    buttons: Vec<MDButton>,
}

impl MessageDialog {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        title: String,
        message: String,
        buttons: Vec<MDButton>,
    ) -> Self {
        let w_min = 4 * MARGIN_SIZE + buttons.len() as i32 * BUTTON_WIDTH;
        debug_assert!(w > w_min, "width({w}) > {w_min}");

        let (sender, receiver) = channel::<i32>();
        Self {
            window: DoubleWindow::new(x, y, w, h, None),
            sender,
            receiver,
            title,
            message,
            buttons,
        }
    }

    fn build(&mut self) {
        self.window.set_frame(FrameType::BorderBox);
        self.window.set_border(false);
        self.window.make_modal(true);
        self.window.begin();

        let mut main = Flex::default_fill().column();
        main.set_margin(2 * MARGIN_SIZE);

        let mut title = Frame::default().with_label(&self.title);
        title.set_label_font(Font::HelveticaBold);
        title.set_label_size(22);
        let (_, h) = title.measure_label();

        main.set_size(&title, h);

        Frame::default().with_label(&self.message);

        let sep = Frame::default();
        main.set_size(&sep, 1);

        {
            let mut buttons = Flex::default().row();

            Frame::default();

            for button in &self.buttons {
                let b = self.create_button(button);
                buttons.set_size(&b, BUTTON_WIDTH);
            }

            Frame::default();

            buttons.end();
            main.set_size(&buttons, BUTTON_HEIGHT);
        }

        main.end();

        self.window.end();

        self.window.handle({
            let sender = self.sender.clone();
            move |w, ev| match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Enter | Key::KPEnter => {
                        w.hide();
                        true
                    }
                    Key::Escape => {
                        sender.send(0);
                        w.hide();
                        true
                    }
                    _ => false,
                },
                _ => false,
            }
        });

        self.window.set_callback({
            let sender = self.sender.clone();
            move |w| {
                if app::event() == Event::Close {
                    sender.send(0);
                    w.hide();
                }
            }
        });
    }

    fn create_button(&self, data: &MDButton) -> Button {
        let mut button = Button::default().with_label(&data.text);
        button.set_color(Color::from_rgb(225, 225, 225));
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
        button.set_callback({
            let mut win = self.window.clone();
            let sender = self.sender.clone();
            let value = data.value.clone();
            move |_| {
                win.hide();
                sender.send(value);
            }
        });
        button
    }

    pub fn run(&mut self) -> i32 {
        self.build();
        self.window.show();
        while self.window.shown() {
            app::wait();
        }
        self.receiver.recv().unwrap_or(0)
    }
}

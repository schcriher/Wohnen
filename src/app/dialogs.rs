use super::wrapper::Widget;
use crate::base::{Filter, Range, HOUSE_TYPES};

use std::collections::HashMap;

use fltk::{
    app,
    button::Button,
    enums::{Color, Event, Font, FrameType},
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
}

impl FilterDialog {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            window: DoubleWindow::new(x, y, w, h, "Filtrar"),
            inputs: HashMap::new(),
        }
    }

    fn build(&mut self) {
        self.window.set_border(false);
        self.window.make_modal(true);
        self.window.begin();

        let margin_size = 20;
        let button_height = 36;

        let mut main = Flex::default_fill().column();
        main.set_margin(margin_size);

        let mut title = Frame::default().with_label("Seleccione los parámetros para filtrar");
        title.set_label_font(Font::HelveticaBold);
        main.set_size(&title, button_height);

        let mut filters = Flex::default_fill().row();
        filters.set_margin(margin_size);

        {
            let left = Flex::default().column();

            self.create_input("kind", "Tipo de vivienda");
            self.create_input("number", "Número");
            self.create_input("floor", "Piso");
            self.create_input("postcode", "Código postal");
            self.set_text_min_max();

            left.end();
        }

        {
            let right = Flex::default().column();

            self.create_input("street", "Calle");
            self.create_input("rooms", "Habitaciones");
            self.create_input("baths", "Baños");
            self.create_input("area", "Superficie (m²)");
            self.set_text_min_max();

            right.end();
        }

        filters.end();

        {
            let buttons = Flex::default().row();

            Frame::default();

            self.create_button("Cancelar");
            self.create_button("Aceptar");

            Frame::default();

            buttons.end();
            main.set_size(&buttons, button_height);
        }

        main.end();

        self.window.set_frame(FrameType::BorderBox);

        self.window.end();
    }

    fn set_text_min_max(&self) {
        let row = Flex::default().row();
        Frame::default();
        Frame::default();
        Frame::default().with_label("Mínimo\n[ 0 ]").set_label_size(12);
        Frame::default().with_label("Máximo\n[ max ]").set_label_size(12);
        row.end();
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
        button.set_callback({
            let mut win = self.window.clone();
            move |_| {
                win.hide();
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

    pub fn run(&mut self) -> Filter {
        self.build();
        self.fill_kind();
        self.window.show();
        while self.window.shown() {
            app::wait();
        }
        self.get_filter()
    }
}

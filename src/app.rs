mod constants;
mod wrapper;

use super::base::{House, DAO, HOUSE_TYPES};
use constants::{Action, NEW_HOUSE};
use wrapper::Widget;

use fltk::{
    app::{self, channel, App, Receiver, Scheme, Sender},
    browser::HoldBrowser,
    button::Button,
    enums::{CallbackTrigger, Color, Event, Font, FrameType},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
    window::Window,
};

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt::Display,
    rc::Rc,
};

pub struct Gui<'a> {
    app: App,
    dao: &'a mut dyn DAO,
    inputs: HashMap<String, Widget>,
    idxhid: HashMap<String, i32>,
    houses: BTreeMap<i32, Rc<RefCell<House>>>, // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    changed: bool,
}

impl<'a> Gui<'a> {
    pub fn new(dao: &'a mut dyn DAO) -> Self {
        let app = App::default();
        let inputs = HashMap::new();
        let idxhid = HashMap::new();
        let houses = BTreeMap::new();
        let (sender, receiver) = channel::<Action>();
        let changed = false;
        Gui {
            app,
            dao,
            inputs,
            idxhid,
            houses,
            sender,
            receiver,
            changed,
        }
    }

    fn build(&mut self) {
        app::set_scheme(Scheme::Gtk);
        app::set_background_color(170, 189, 206);
        app::set_background2_color(200, 255, 200);
        app::set_foreground_color(0, 0, 0);
        app::set_selection_color(255, 160, 63);
        app::set_inactive_color(130, 149, 166);
        app::set_font_size(16);
        app::set_visible_focus(false);

        let mut win = Window::default()
            .with_label("Wohnen - Schcriher")
            .with_size(900, 460)
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
            let mut select = HoldBrowser::default();
            select.set_selection_color(Color::from_hex(0x1b1b1b));
            select.emit(self.sender, Action::Select);
            self.inputs.insert("select".to_string(), Widget::Browser(select));
        }

        left.end();

        // --- RIGHT --------------------------------------------

        let mut right = Flex::default().column();
        right.set_margin(margin_size);

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

        let icon = include_bytes!("../assets/icon.svg");
        let icon = SvgImage::from_data(std::str::from_utf8(icon).unwrap()).unwrap();
        win.set_icon(Some(icon));
    }

    fn create_button(&self, caption: &str, action: Action) {
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
    }

    fn create_input(&mut self, key: &str, text: &str) {
        let row = Flex::default().row();
        Frame::default().with_label(text);
        let sender = self.sender.clone();
        let widget = match key {
            "id" => {
                let mut input = Input::default();
                input.set_frame(FrameType::FlatBox);
                input.set_readonly(true);
                input.deactivate();
                Widget::TInput(input)
            }
            "kind" => {
                let mut kind = Choice::default();
                kind.set_selection_color(Color::from_hex(0x1b1b1b));
                kind.emit(sender, Action::Change);
                Widget::Choice(kind)
            }
            "street" | "postcode" => {
                let mut input = Input::default();
                input.set_tooltip("Puede ingresar texto y números");
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                Widget::TInput(input)
            }
            "area" => {
                let mut input = FloatInput::default();
                input.set_tooltip("Puede ingresar números decimales");
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                Widget::FInput(input)
            }
            _ => {
                let mut input = IntInput::default();
                input.set_tooltip("Puede ingresar solamente números enteros");
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                Widget::IInput(input)
            }
        };
        row.end();
        self.inputs.insert(key.to_string(), widget);
    }

    fn get_widget(&self, key: &str) -> &Widget {
        self.inputs.get(key).unwrap()
    }

    fn get_widget_mut(&mut self, key: &str) -> &mut Widget {
        self.inputs.get_mut(key).unwrap()
    }

    fn get_value(&self, key: &str) -> String {
        self.get_widget(key).get()
    }

    fn set_value<U: Display>(&mut self, key: &str, value: U) {
        self.get_widget_mut(key).set(value);
    }

    fn add_value(&mut self, key: &str, value: &str) {
        self.get_widget_mut(key).add(value);
    }

    fn fill_select(&mut self) {
        if let Ok(houses) = self.dao.get_houses() {
            for house in houses {
                self.houses.insert(house.id, Rc::new(RefCell::new(house)));
            }
        }
        let select = self.inputs.get_mut("select").unwrap();
        for (index, house) in self.houses.values().enumerate() {
            let house = house.borrow();
            select.add(&format!("Vivienda {:>02}: {} ({})", house.id, house.street, house.kind));
            self.idxhid.insert((index + 1).to_string(), house.id);
        }
    }

    fn fill_kind(&mut self) {
        let kind = self.get_widget_mut("kind");
        for value in HOUSE_TYPES {
            kind.add(value);
        }
    }

    fn set_new_house(&mut self) {
        self.set_value("id", NEW_HOUSE);
        self.add_value("select", NEW_HOUSE);
        self.set_value("select", i32::MAX);
    }

    fn clear(&mut self) {
        for widget in self.inputs.values_mut() {
            widget.set("");
        }
    }

    fn show(&mut self) {
        let idx = self.get_value("select");
        if let Some(hid) = self.idxhid.get(&idx) {
            if let Some(house) = self.houses.get(&hid) {
                let house = house.to_owned();
                let house = house.borrow();
                self.set_value("id", &house.id);
                self.set_value("kind", HOUSE_TYPES.iter().position(|&r| r == house.kind).unwrap());
                self.set_value("street", &house.street);
                self.set_value("number", &house.number);
                self.set_value("floor", &house.floor);
                self.set_value("postcode", &house.postcode);
                self.set_value("rooms", &house.rooms);
                self.set_value("baths", &house.baths);
                self.set_value("area", &house.area);
            }
        }
    }

    pub fn run(&mut self) {
        self.build();

        self.fill_select();
        self.fill_kind();

        while self.app.wait() {
            match self.receiver.recv() {
                Some(Action::Create) => {
                    println!("Create");
                    self.clear();
                    self.set_new_house();
                }
                Some(Action::Update) => {
                    println!("Update");
                    self.changed = false;
                }
                Some(Action::Delete) => {
                    println!("Delete");
                }
                Some(Action::Select) => {
                    println!("Select");
                    let input: &mut Widget = self.get_widget_mut("select");
                    let pos = input.get();
                    let last = input.get_size();
                    if pos != last && input.get_text(&last) == NEW_HOUSE {
                        input.del(&last);
                    }
                    if pos != "0" {
                        self.show();
                    } else {
                        self.clear();
                    }
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

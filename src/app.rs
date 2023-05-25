use super::base::{House, DAO, HOUSE_TYPES};

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
    any::Any,
    collections::{BTreeMap, HashMap},
};

#[derive(Clone, Copy)]
enum Action {
    Select,
    Filter,
    Create,
    Update,
    Delete,
    Change,
}

pub struct Gui<'a> {
    app: App,
    dao: &'a mut dyn DAO,
    inputs: HashMap<String, Box<dyn Any>>,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    changed: bool,
    idx_to_hid: HashMap<i32, i32>,
    houses: BTreeMap<i32, House>,
}

impl<'a> Gui<'a> {
    pub fn new(dao: &'a mut dyn DAO) -> Self {
        let app = App::default();
        let inputs = HashMap::new();
        let houses = BTreeMap::new();
        let idx_to_hid = HashMap::new();
        let (sender, receiver) = channel::<Action>();
        Gui {
            app,
            dao,
            inputs,
            sender,
            receiver,
            changed: false,
            idx_to_hid,
            houses,
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
        let sender = self.sender.clone();
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
                choice.set_selection_color(Color::from_hex(0x1b1b1b));
                choice.emit(self.sender, Action::Change);
                obj = Box::new(choice);
            }
            "street" | "postcode" => {
                let mut input = Input::default();
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                obj = Box::new(input);
            }
            "area" => {
                let mut input = FloatInput::default();
                input.set_tooltip("Ingrese un número con o sin coma");
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                obj = Box::new(input);
            }
            _ => {
                let mut input = IntInput::default();
                input.set_tooltip("Ingrese un número entero");
                input.set_trigger(CallbackTrigger::Changed);
                input.set_callback(move |_| sender.send(Action::Change));
                obj = Box::new(input);
            }
        }
        row.end();
        self.inputs.insert(key.to_string(), obj);
    }

    fn fill_browser(&mut self) {
        if let Ok(houses) = self.dao.get_houses() {
            for house in houses {
                self.houses.insert(house.id, house);
            }
        }

        let browser = self
            .inputs
            .get_mut("list")
            .unwrap()
            .downcast_mut::<HoldBrowser>()
            .unwrap();

        for (index, house) in self.houses.values().enumerate() {
            browser.add(&format!("Vivienda {:>02}: {} ({})", house.id, house.street, house.kind));
            self.idx_to_hid.insert((index + 1) as i32, house.id);
        }
    }

    fn fill_choice(&mut self) {
        let choice = self.inputs.get_mut("kind").unwrap().downcast_mut::<Choice>().unwrap();
        for value in HOUSE_TYPES {
            choice.add_choice(value);
        }
    }

    fn clear(&mut self) {
        {
            let widget = self.inputs.get_mut("id").unwrap().downcast_mut::<Input>().unwrap();
            widget.set_value("«nuevo»");
        }

        {
            let widget = self.inputs.get_mut("kind").unwrap().downcast_mut::<Choice>().unwrap();
            widget.set_value(-1);
        }

        {
            let widget = self.inputs.get_mut("street").unwrap().downcast_mut::<Input>().unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("number")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("floor")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("postcode")
                .unwrap()
                .downcast_mut::<Input>()
                .unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("rooms")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("baths")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value("");
        }

        {
            let widget = self
                .inputs
                .get_mut("area")
                .unwrap()
                .downcast_mut::<FloatInput>()
                .unwrap();
            widget.set_value("");
        }
    }

    fn show(&mut self) {
        let browser = self
            .inputs
            .get_mut("list")
            .unwrap()
            .downcast_mut::<HoldBrowser>()
            .unwrap();
        let idx = browser.value();
        if idx < 1 {
            return;
        }

        let hid = self.idx_to_hid.get(&idx).unwrap();
        let house = self.houses.get(&hid).unwrap().clone();

        {
            let widget = self.inputs.get_mut("id").unwrap().downcast_mut::<Input>().unwrap();
            widget.set_value(&format!("{}", house.id));
        }

        {
            let widget = self.inputs.get_mut("kind").unwrap().downcast_mut::<Choice>().unwrap();
            let index = HOUSE_TYPES.iter().position(|&r| r == house.kind).unwrap();
            println!("index: {index}");
            widget.set_value(index as i32);
        }

        {
            let widget = self.inputs.get_mut("street").unwrap().downcast_mut::<Input>().unwrap();
            widget.set_value(&format!("{}", house.street));
        }

        {
            let widget = self
                .inputs
                .get_mut("number")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value(&format!("{}", house.number));
        }

        {
            let widget = self
                .inputs
                .get_mut("floor")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value(&format!("{}", house.floor));
        }

        {
            let widget = self
                .inputs
                .get_mut("postcode")
                .unwrap()
                .downcast_mut::<Input>()
                .unwrap();
            widget.set_value(&format!("{}", house.postcode));
        }

        {
            let widget = self
                .inputs
                .get_mut("rooms")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value(&format!("{}", house.rooms));
        }

        {
            let widget = self
                .inputs
                .get_mut("baths")
                .unwrap()
                .downcast_mut::<IntInput>()
                .unwrap();
            widget.set_value(&format!("{}", house.baths));
        }

        {
            let widget = self
                .inputs
                .get_mut("area")
                .unwrap()
                .downcast_mut::<FloatInput>()
                .unwrap();
            widget.set_value(&format!("{}", house.area));
        }
    }

    pub fn run(&mut self) {
        self.build();

        self.fill_browser();
        self.fill_choice();

        while self.app.wait() {
            match self.receiver.recv() {
                Some(Action::Create) => {
                    println!("Create");
                    self.clear();
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
                    self.show();
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

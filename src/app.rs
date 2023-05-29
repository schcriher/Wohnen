mod dialogs;
mod wrapper;

use crate::base::{Filter, House, DAO, HOUSE_TYPES};

use dialogs::FilterDialog;
use wrapper::Widget;

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt::Display,
    rc::Rc,
};

use fltk::{
    app::{self, channel, App, Receiver, Scheme, Sender},
    browser::HoldBrowser,
    button::Button,
    dialog,
    enums::{CallbackTrigger, Color, Event, Font, FrameType},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
    window::DoubleWindow,
};

pub const NEW_HOUSE: &str = "«nuevo»";

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Action {
    Select,
    Filter,
    Unfilter,
    New,
    Save,
    Delete,
    Change,
    Close,
}

pub struct Gui<'a> {
    app: App,
    win: DoubleWindow,
    dao: &'a mut dyn DAO,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    inputs: HashMap<String, Widget>,
    idxhid: HashMap<String, i32>,
    houses: BTreeMap<i32, Rc<RefCell<House>>>, // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    buttons: HashMap<Action, Button>,
    hid_select: i32,
}

impl<'a> Gui<'a> {
    pub fn new(dao: &'a mut dyn DAO) -> Self {
        let (sender, receiver) = channel::<Action>();
        Gui {
            dao,
            sender,
            receiver,
            app: App::default(),
            win: DoubleWindow::default(),
            inputs: HashMap::new(),
            idxhid: HashMap::new(),
            houses: BTreeMap::new(),
            buttons: HashMap::new(),
            hid_select: -1,
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

        self.win.set_label("Wohnen - Schcriher");

        let (sx, sy, sw, sh) = app::screen_work_area(self.win.screen_num());
        let w = 900;
        let h = 460;

        if w > sw || h > sh {
            let x = sx + (sw - 420) / 2;
            let y = sy + (sh - 190) / 2;
            let line1 = format!("Esta aplicación fue diseñada para una");
            let line2 = format!("pantalla de {w}x{h} píxeles como mínimo");
            let line3 = format!("Detectado: {sw}x{sh} (área utilizable)");
            let error = format!("𝗘𝗥𝗥𝗢𝗥\n{line1}\n{line2}\n\n{line3}");
            dialog::beep(dialog::BeepType::Error);
            dialog::alert(x, y, &error);
            panic!("{error}");
        }

        let x = sx + (sw - w) / 2;
        let y = sy + (sh - h) / 2;
        self.win.set_size(w, h);
        self.win.set_pos(x, y);

        self.win.begin();

        let margin_size = 10;
        let button_height = 36;

        let mut main = Flex::default_fill().row();
        main.set_margin(margin_size);

        // --- LEFT ---------------------------------------------

        let mut left = Flex::default().column();
        left.set_margin(margin_size);

        {
            let row = Flex::default().row();
            self.create_button("Nuevo", Action::New);
            self.create_button("Filtrar", Action::Filter);
            self.create_button("Quitar Filtro", Action::Unfilter);
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
        title.set_label_size(22);
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
            self.create_button("Guardar", Action::Save);
            self.create_button("Salir", Action::Close);
            row.end();
            right.set_size(&row, button_height);
        }

        right.end();

        // ------------------------------------------------------

        main.end();

        self.win.end();

        let icon = include_bytes!("../assets/icon.svg");
        let icon = SvgImage::from_data(std::str::from_utf8(icon).unwrap()).unwrap();
        self.win.set_icon(Some(icon));
    }

    fn get_pos(&self, width: i32, height: i32) -> (i32, i32) {
        let w = self.win.w();
        let h = self.win.h();
        let x = self.win.x();
        let y = self.win.y();
        let offset_x = (w - width) / 2;
        let offset_y = (h - height) / 2;
        (x + offset_x, y + offset_y)
    }

    fn create_button(&mut self, caption: &str, action: Action) {
        let mut button = Button::default().with_label(caption);
        button.set_color(Color::from_rgb(225, 225, 225));
        button.emit(self.sender, action);
        button.handle(move |b, ev| match ev {
            Event::Enter => {
                if b.active() {
                    b.set_color(Color::from_rgb(150, 150, 150));
                } else {
                    b.set_color(Color::from_rgb(225, 225, 225));
                }
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
        self.buttons.insert(action, button);
    }

    fn create_input(&mut self, key: &str, text: &str) {
        let row = Flex::default().row();
        Frame::default().with_label(text);
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
                kind.emit(self.sender, Action::Change);
                Widget::Choice(kind)
            }
            "street" => {
                let mut input = Input::default();
                input.set_tooltip("Ingrese la dirección de la vivienda");
                input.set_trigger(CallbackTrigger::Changed);
                input.emit(self.sender, Action::Change);
                Widget::TInput(input)
            }
            "area" => {
                let mut input = FloatInput::default();
                input.set_tooltip("Ingresar los metros cuadrados, puede ser decimales");
                input.set_trigger(CallbackTrigger::Changed);
                input.emit(self.sender, Action::Change);
                Widget::FInput(input)
            }
            _ => {
                let mut input = IntInput::default();
                input.set_tooltip("Ingrese solamente números enteros");
                input.set_trigger(CallbackTrigger::Changed);
                input.emit(self.sender, Action::Change);
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

    fn fill_select(&mut self, filter: Filter) {
        self.clear_house();
        self.houses.clear();
        self.idxhid.clear();

        let select = self.inputs.get_mut("select").unwrap();
        select.clear();

        let mut max = 0;
        if let Ok(houses) = self.dao.get_houses() {
            max = houses.len();
            for house in houses {
                if filter.valid(&house) {
                    self.houses.insert(house.id, Rc::new(RefCell::new(house)));
                }
            }
        }

        for (index, house) in self.houses.values().enumerate() {
            let house = house.borrow();
            select.add(&format!("Vivienda {:>02}: {} ({})", house.id, house.street, house.kind));
            let idx = (index + 1).to_string();
            self.idxhid.insert(idx.clone(), house.id);

            if house.id == self.hid_select {
                select.set(idx);
            }
        }

        let selected = select.get() != "0";
        self.set_button_status(Action::Delete, selected);
        self.set_button_status(Action::Unfilter, self.houses.len() < max);
        self.show_house();
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

    fn clear_house(&mut self) {
        self.hid_select = -1;
        for widget in self.inputs.values_mut() {
            widget.set("");
        }
    }

    fn show_house(&mut self) {
        self.hid_select = -1;
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
                self.hid_select = house.id;
            }
        }
    }

    fn set_button_status(&mut self, key: Action, value: bool) {
        let button = self.buttons.get_mut(&key).unwrap();
        if value {
            button.activate();
        } else {
            button.deactivate();
        }
    }

    fn set_buttons_new_save_delete(&mut self, new: bool, save: bool, delete: bool) {
        self.set_button_status(Action::New, new);
        self.set_button_status(Action::Save, save);
        self.set_button_status(Action::Delete, delete);
    }

    fn unsaved_changes(&self) -> bool {
        let button = self.buttons.get(&Action::Save).unwrap();
        button.active()
    }

    fn current_is_new_house(&self) -> bool {
        let input = self.get_widget("select");
        let pos = input.get();
        pos != "0" && input.get_text(&pos) == NEW_HOUSE
    }

    fn is_data_completed(&self) -> bool {
        self.get_value("kind") != "-1"
            && self.get_value("street") != ""
            && self.get_value("number") != ""
            && self.get_value("floor") != ""
            && self.get_value("postcode") != ""
            && self.get_value("rooms") != ""
            && self.get_value("baths") != ""
            && self.get_value("area") != ""
    }

    fn update_house(&self, house: &mut House) {
        let kind = self.get_widget("kind");
        house.kind = kind.get_text(&kind.get());
        house.street = self.get_value("street");
        house.number = self.get_value("number").parse::<i32>().unwrap();
        house.floor = self.get_value("floor").parse::<i32>().unwrap();
        house.postcode = self.get_value("postcode").parse::<i32>().unwrap();
        house.rooms = self.get_value("rooms").parse::<i32>().unwrap();
        house.baths = self.get_value("baths").parse::<i32>().unwrap();
        house.area = self.get_value("area").parse::<f32>().unwrap();
    }

    pub fn run(&mut self) {
        self.build();
        self.set_buttons_new_save_delete(true, false, false);
        self.fill_select(Filter::default());
        self.fill_kind();
        self.win.show();

        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                match action {
                    Action::New => {
                        self.clear_house();
                        self.set_new_house();
                        self.set_buttons_new_save_delete(false, false, false);
                    }

                    Action::Save => {
                        if self.is_data_completed() {
                            if self.current_is_new_house() {
                                let mut house = House::default();
                                self.update_house(&mut house);
                                // TODO The user should be informed of the result
                                match self.dao.create_house(&house) {
                                    Ok(house) => self.hid_select = house.id,
                                    Err(_) => {}
                                }
                            } else {
                                let house = self.houses.get(&self.hid_select).unwrap();
                                let mut house = house.borrow_mut();
                                self.update_house(&mut house);
                                // TODO The user should be informed of the result
                                match self.dao.update_house(&house) {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                            }
                            self.set_buttons_new_save_delete(true, false, true);
                            // TODO A better option would be to update only Browser and BTreeMap
                            self.fill_select(Filter::default()); // update delete button
                        } else {
                            //
                            // FIXME: informar de fatos faltantes
                            //
                            println!("  FALTAN DATOS!!!");
                        }
                    }

                    Action::Delete => {
                        if let Some(house) = self.houses.get(&self.hid_select) {
                            let house = house.borrow();
                            // TODO The user should be informed of the result
                            match self.dao.delete_house(house.id) {
                                Ok(_) => {}
                                Err(_) => {}
                            }
                        }
                        self.clear_house();
                        self.set_buttons_new_save_delete(true, false, false);
                        // TODO A better option would be to update only Browser and BTreeMap
                        self.fill_select(Filter::default()); // update delete button
                    }

                    Action::Select => {
                        if self.unsaved_changes() {
                            //
                            // FIXME: ask for save the changes
                            //
                            println!("  Changed!!!");
                        }

                        let input = self.get_widget_mut("select");
                        let pos = input.get();
                        let last = input.get_size();
                        if pos != last && input.get_text(&last) == NEW_HOUSE {
                            input.del(&last);
                        }
                        let selected = pos != "0";
                        if selected {
                            self.show_house();
                        } else {
                            self.clear_house();
                        }
                        let not_new = !self.current_is_new_house();
                        self.set_buttons_new_save_delete(!selected || not_new, false, not_new);
                    }

                    Action::Filter => {
                        let button = self.buttons.get_mut(&Action::Filter).unwrap();
                        button.set_color(Color::from_rgb(225, 225, 225));
                        button.redraw();

                        self.win.deactivate();

                        let width = 800;
                        let height = 360;
                        let (x, y) = self.get_pos(width, height);

                        let mut dialog = FilterDialog::new(x, y, width, height);
                        let filter = dialog.run();

                        self.set_buttons_new_save_delete(true, false, false);
                        if let Some(filter) = filter {
                            self.fill_select(filter); // update delete button
                        }

                        self.win.activate();
                    }

                    Action::Unfilter => {
                        self.set_buttons_new_save_delete(true, false, false);
                        self.fill_select(Filter::default()); // update delete button
                    }

                    Action::Change => {
                        self.set_button_status(Action::Save, true);
                    }

                    Action::Close => {
                        if self.unsaved_changes() {
                            //
                            // FIXME: ask for save the changes
                            //
                            println!("  Changed!!!");
                        }
                        self.app.quit();
                    }
                }
            }
        }
    }
}

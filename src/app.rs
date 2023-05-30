mod dialogs;
mod utils;
mod wrapper;

use crate::base::{Filter, House, DAO, HOUSE_TYPES};

use dialogs::{FilterDialog, MDButton, MessageDialog};
use utils::test_positive_and_zero_number;
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
    enums::{CallbackTrigger, Color, Event, Font, FrameType, Key},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
    window::DoubleWindow,
};
use fltk_theme::{color_themes, ColorTheme};

pub const NEW_HOUSE: &str = "«nuevo»";
pub const MARGIN_SIZE: i32 = 16;
pub const BUTTON_WIDTH: i32 = 128;
pub const BUTTON_HEIGHT: i32 = 32;

pub const FOREGROUND_COLOR: Color = Color::from_rgb(190, 190, 190);
pub const SELECTION_COLOR: Color = Color::from_rgb(13, 13, 13);
pub const NORMAL_COLOR: Color = Color::from_rgb(23, 23, 23);
pub const HOVER_COLOR: Color = Color::from_rgb(16, 16, 16);
pub const ERROR_COLOR: Color = Color::from_rgb(86, 16, 16);

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

// Rc<RefCell<···>> https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

pub struct Gui<'a> {
    app: App,
    win: DoubleWindow,
    dao: &'a mut dyn DAO,
    sender: Sender<Action>,
    receiver: Receiver<Action>,
    inputs: HashMap<String, Widget>,
    idxhid: HashMap<String, i32>,
    houses: BTreeMap<i32, Rc<RefCell<House>>>,
    buttons: HashMap<Action, Button>,
    hid_select: i32,
    current_filter: Filter,
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
            current_filter: Filter::default(),
        }
    }

    fn build(&mut self) {
        app::set_font_size(16);
        app::set_visible_focus(false);
        app::set_scheme(Scheme::Gtk);
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();

        let (r, g, b) = FOREGROUND_COLOR.to_rgb();
        app::set_foreground_color(r, g, b);

        let (r, g, b) = SELECTION_COLOR.to_rgb();
        app::set_selection_color(r, g, b);

        self.win.set_label("Wohnen - Schcriher");

        let (sx, sy, sw, sh) = app::screen_work_area(self.win.screen_num());
        let w = 900;
        let h = 500;

        if w > sw || h > sh {
            let x = sx + (sw - 420) / 2;
            let y = sy + (sh - 190) / 2;
            let error = format!(
                "𝗘𝗥𝗥𝗢𝗥\n\
                Esta aplicación fue diseñada para una\n\
                pantalla de {w}x{h} píxeles como mínimo\n\
                \n\
                Detectado: {sw}x{sh} (área utilizable)"
            );
            dialog::beep(dialog::BeepType::Error);
            dialog::alert(x, y, &error);
            panic!("{error}");
        }

        let x = sx + (sw - w) / 2;
        let y = sy + (sh - h) / 2;
        self.win.set_size(w, h);
        self.win.set_pos(x, y);

        self.win.begin();

        let mut main = Flex::default_fill().row();
        main.set_margin(MARGIN_SIZE);

        // --- LEFT ---------------------------------------------

        let mut left = Flex::default().column();
        left.set_margin(MARGIN_SIZE);

        {
            let row = Flex::default().row();
            self.create_button("Nuevo", Action::New);
            self.create_button("Filtrar", Action::Filter);
            self.create_button("Quitar Filtro", Action::Unfilter);
            row.end();
            left.set_size(&row, BUTTON_HEIGHT);
        }

        {
            let mut select = HoldBrowser::default();
            select.emit(self.sender, Action::Select);
            self.inputs.insert("select".to_owned(), Widget::Browser(select));
        }

        left.end();

        // --- RIGHT --------------------------------------------

        let mut right = Flex::default().column();
        right.set_margin(MARGIN_SIZE);

        let mut title = Frame::default().with_label("Vivienda Seleccionada");
        title.set_label_font(Font::HelveticaBold);
        title.set_label_size(24);
        right.set_size(&title, BUTTON_HEIGHT);

        let sep = Frame::default();
        right.set_size(&sep, 8);

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
        right.set_size(&sep, 16);

        {
            let row = Flex::default().row();
            self.create_button("Borrar", Action::Delete);
            self.create_button("Guardar", Action::Save);
            self.create_button("Salir", Action::Close);
            row.end();
            right.set_size(&row, BUTTON_HEIGHT);
        }

        right.end();

        // ------------------------------------------------------

        main.end();

        self.win.end();

        let icon = include_bytes!("../assets/icon.svg");
        let icon = SvgImage::from_data(std::str::from_utf8(icon).unwrap()).unwrap();
        self.win.set_icon(Some(icon));

        self.win.handle({
            let sender = self.sender.clone();
            move |_, ev| match ev {
                Event::KeyDown => match app::event_key() {
                    Key::Escape => {
                        sender.send(Action::Close);
                        true
                    }
                    _ => false,
                },
                _ => false,
            }
        });

        self.win.set_callback({
            let sender = self.sender.clone();
            move |_| {
                if app::event() == Event::Close {
                    sender.send(Action::Close);
                }
            }
        });
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
        button.set_color(NORMAL_COLOR);
        button.emit(self.sender, action);
        button.handle(move |b, ev| match ev {
            Event::Enter => {
                if b.active() {
                    b.set_color(HOVER_COLOR);
                } else {
                    b.set_color(NORMAL_COLOR);
                }
                b.redraw();
                true
            }
            Event::Leave => {
                b.set_color(NORMAL_COLOR);
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
        let mut widget = match key {
            "id" => {
                let mut input = Input::default();
                input.set_tooltip("Este es el ID en la base de datos");
                input.set_frame(FrameType::FlatBox);
                input.set_readonly(true);
                input.deactivate();
                Widget::TInput(input)
            }
            "kind" => {
                let mut kind = Choice::default();
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
        widget.set_color(NORMAL_COLOR);
        row.end();
        self.inputs.insert(key.to_owned(), widget);
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

    fn set_color(&mut self, key: &str, color: Color) {
        self.get_widget_mut(key).set_color(color);
    }

    fn fill_select(&mut self) {
        self.clear_house();
        self.houses.clear();
        self.idxhid.clear();

        let select = self.inputs.get_mut("select").unwrap();
        select.clear();

        let mut max = 0;
        // TODO Pagination should be implemented
        if let Ok(houses) = self.dao.get_houses() {
            max = houses.len();
            for house in houses {
                if self.current_filter.valid(&house) {
                    self.houses.insert(house.id, Rc::new(RefCell::new(house)));
                }
            }
        }

        for (index, house) in self.houses.values().enumerate() {
            let house = house.borrow();
            select.add(&format!("{} al {}", house.street, house.number));
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

    fn reset_buttons_color(&mut self) {
        for button in self.buttons.values_mut() {
            button.set_color(NORMAL_COLOR);
        }
        self.win.redraw();
    }

    fn reset_inputs_color(&mut self) {
        for input in self.inputs.values_mut() {
            // HoldBrowser is ignored by wrapper::Widget::set_color()
            input.set_color(NORMAL_COLOR);
        }
        self.win.redraw();
    }

    fn set_buttons_new_save_delete(&mut self, new: bool, save: bool, delete: bool) {
        self.set_button_status(Action::New, new);
        self.set_button_status(Action::Save, save);
        self.set_button_status(Action::Delete, delete);
    }

    fn current_nothing_selected(&self) -> bool {
        let input = self.get_widget("select");
        let idx = input.get();
        idx == "0"
    }

    fn current_is_new_house(&self) -> bool {
        let input = self.get_widget("select");
        let idx = input.get();
        idx != "0" && input.get_text(&idx) == NEW_HOUSE
    }

    fn is_data_field_completed_and_correct(&mut self) -> bool {
        let keys = [
            "kind", "street", "number", "floor", "postcode", "rooms", "baths", "area",
        ];
        let mut count = 0;
        for key in keys {
            count += if self.is_data_field_correct(key) { 1 } else { 0 };
        }
        self.win.redraw();
        count == keys.len()
    }

    fn is_data_field_correct(&mut self, key: &str) -> bool {
        if self.is_data_value_correct(key) {
            self.set_color(key, NORMAL_COLOR);
            true
        } else {
            self.set_color(key, ERROR_COLOR);
            false
        }
    }

    fn is_data_value_correct(&self, key: &str) -> bool {
        match key {
            "kind" => self.get_value(key) != "-1",
            "street" => self.get_value(key) != "",
            "area" => test_positive_and_zero_number::<f32>(&self.get_value(key)),
            _ => test_positive_and_zero_number::<i32>(&self.get_value(key)),
        }
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

    fn open_message_dialog(&mut self, title: &str, message: &str, buttons: Vec<MDButton>) -> i32 {
        self.win.deactivate();

        let width = 360;
        let height = 200;
        let (x, y) = self.get_pos(width, height);
        let mut dialog =
            MessageDialog::new(x, y, width, height, title.to_owned(), message.to_owned(), buttons);
        let answer = dialog.run();

        self.win.activate();
        answer
    }

    pub fn run(&mut self) {
        self.build();
        self.set_buttons_new_save_delete(true, false, false);
        self.fill_select();
        self.fill_kind();
        self.win.show();

        while self.app.wait() {
            if let Some(action) = self.receiver.recv() {
                self.reset_buttons_color();

                match action {
                    Action::New => {
                        self.clear_house();
                        self.set_new_house();
                        self.set_buttons_new_save_delete(false, false, false);
                        self.reset_inputs_color();
                    }

                    Action::Save => {
                        if self.is_data_field_completed_and_correct() {
                            if self.current_nothing_selected() || self.current_is_new_house() {
                                let mut house = House::default();
                                self.update_house(&mut house);
                                match self.dao.create_house(&house) {
                                    Ok(house) => self.hid_select = house.id,
                                    Err(_) => {
                                        self.open_message_dialog(
                                            "Error",
                                            "No se pudo guardar la vivienda",
                                            vec![MDButton::new("Aceptar", 0)],
                                        );
                                    }
                                }
                            } else {
                                let houses = self.houses.clone();
                                let house = houses.get(&self.hid_select).unwrap();
                                let mut house = house.borrow_mut();
                                self.update_house(&mut house);
                                match self.dao.update_house(&house) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        self.open_message_dialog(
                                            "Error",
                                            "No se pudo guardar la vivienda",
                                            vec![MDButton::new("Aceptar", 0)],
                                        );
                                    }
                                }
                            }
                            self.set_button_status(Action::Save, false);
                            // TODO A better option would be to update only Browser and BTreeMap
                            self.fill_select(); // update delete button
                        } else {
                            self.open_message_dialog(
                                "Error",
                                "Los datos cargados contienen errores\nverifíquelos para continuar",
                                vec![MDButton::new("Aceptar", 0)],
                            );
                        }
                    }

                    Action::Delete => {
                        // TODO Should be asked if delete is desired
                        let key = self.hid_select;
                        match self.dao.delete_house(key) {
                            Ok(_) => {
                                self.hid_select = -1;
                            }
                            Err(_) => {
                                self.open_message_dialog(
                                    "Error",
                                    "No se pudo borrar la vivienda",
                                    vec![MDButton::new("Aceptar", 0)],
                                );
                            }
                        }
                        self.set_buttons_new_save_delete(true, false, false);
                        // TODO A better option would be to update only Browser and BTreeMap
                        self.fill_select(); // update delete button
                        self.reset_inputs_color();
                    }

                    Action::Select => {
                        // TODO Should be checked if there are unsaved changes to ask what to do

                        let input = self.get_widget_mut("select");
                        let idx = input.get();
                        let last = input.get_size();
                        if idx != last && input.get_text(&last) == NEW_HOUSE {
                            input.del(&last);
                        }
                        let selected = idx != "0";
                        if selected {
                            self.show_house();
                        } else {
                            self.clear_house();
                        }
                        let not_new = !self.current_is_new_house();
                        self.set_buttons_new_save_delete(!selected || not_new, false, selected && not_new);
                        self.reset_inputs_color();
                    }

                    Action::Filter => {
                        self.win.deactivate();

                        let width = 800;
                        let height = 360;
                        let (x, y) = self.get_pos(width, height);

                        let mut dialog = FilterDialog::new(x, y, width, height);
                        let filter = dialog.run();

                        self.set_buttons_new_save_delete(true, false, false);
                        if let Some(filter) = filter {
                            self.current_filter = filter;
                            self.fill_select(); // update delete button
                        }

                        self.win.activate();
                        self.reset_inputs_color();
                    }

                    Action::Unfilter => {
                        self.set_buttons_new_save_delete(true, false, false);
                        self.current_filter = Filter::default();
                        self.fill_select(); // update delete button
                        self.reset_inputs_color();
                    }

                    Action::Change => {
                        self.set_button_status(Action::Save, true);
                        self.is_data_field_completed_and_correct();
                    }

                    Action::Close => {
                        // TODO Should be checked if there are unsaved changes to ask what to do
                        self.app.quit();
                    }
                }
            }
        }
    }
}

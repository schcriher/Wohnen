use std::fmt::Display;

use fltk::{
    browser::HoldBrowser,
    enums::Color,
    input::{FloatInput, Input, IntInput},
    menu::Choice,
    prelude::*,
};

// https://users.rust-lang.org/t/how-to-create-a-vector-with-different-types-of-gui-widgets

#[derive(Debug)]
pub enum Widget {
    TInput(Input),
    IInput(IntInput),
    FInput(FloatInput),
    Choice(Choice),       // start 0
    Browser(HoldBrowser), // start 1
}

impl Widget {
    pub fn get(&self) -> String {
        match self {
            Self::TInput(w) => w.value(),
            Self::IInput(w) => w.value(),
            Self::FInput(w) => w.value(),
            Self::Choice(w) => w.value().to_string(),
            Self::Browser(w) => w.value().to_string(),
        }
    }

    pub fn set<T: Display>(&mut self, value: T) -> &Self {
        let value = value.to_string();
        match self {
            Self::TInput(w) => w.set_value(&value),
            Self::IInput(w) => w.set_value(&value),
            Self::FInput(w) => w.set_value(&value),
            Self::Choice(w) => {
                let index: i32 = value.parse().unwrap_or(-1);
                w.set_value(index);
            }
            Self::Browser(w) => {
                let index: i32 = value.parse().unwrap_or(-1);
                if index == i32::MAX {
                    w.select(w.size());
                } else {
                    w.select(index);
                }
            }
        }
        self
    }

    pub fn set_color(&mut self, color: Color) {
        match self {
            Self::TInput(w) => w.set_color(color),
            Self::IInput(w) => w.set_color(color),
            Self::FInput(w) => w.set_color(color),
            Self::Choice(w) => w.set_color(color),
            Self::Browser(_) => {}
        }
    }

    pub fn clear(&mut self) -> &Self {
        match self {
            Self::Choice(w) => w.clear(),
            Self::Browser(w) => w.clear(),
            _ => panic!("unsupported operation"),
        }
        self
    }

    pub fn add(&mut self, value: &str) -> &Self {
        match self {
            Self::Choice(w) => {
                w.add_choice(value);
            }
            Self::Browser(w) => {
                w.add(value);
            }
            _ => panic!("unsupported operation"),
        }
        self
    }

    pub fn del(&mut self, index: &str) -> &Self {
        let index: i32 = index.parse().unwrap();
        match self {
            Self::Browser(w) => {
                w.remove(index);
            }
            _ => panic!("unsupported operation"),
        }
        self
    }

    pub fn get_text(&self, index: &str) -> String {
        let index: i32 = index.parse().unwrap();
        match self {
            Self::Choice(w) => w.text(index).unwrap(),
            Self::Browser(w) => w.text(index).unwrap_or("".to_owned()), // can be deselected
            _ => panic!("unsupported operation"),
        }
    }

    pub fn get_size(&self) -> String {
        match self {
            Self::Browser(w) => w.size().to_string(),
            _ => panic!("unsupported operation"),
        }
    }
}

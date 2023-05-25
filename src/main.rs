mod app;
mod base;
mod data;

fn main() {
    let mut dao = data::Service::new();
    let mut gui = app::Gui::new(&mut dao);
    gui.run();
}

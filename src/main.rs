/*
 * 𝐖𝐨𝐡𝐧𝐧𝐞𝐧 🖥 𝕾𝖈𝖍𝖈𝖗𝖎𝖍𝖊𝖗
 */
mod app;
mod base;
mod data;

fn main() {
    let mut dao = data::Service::new();
    let mut gui = app::Gui::new(&mut dao);
    gui.run();
}

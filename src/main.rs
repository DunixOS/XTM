mod api;
mod app;
mod models;
mod storage;
mod theme;
mod ui;
mod xbox;

fn main() -> eframe::Result<()> {
    app::run()
}

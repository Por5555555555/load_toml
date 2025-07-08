use model::h1::H1;
use model::main_models::ModelStrut;
use model::text::Text;
use tracing::info;

mod model;
mod service;

fn main() {
    tracing_subscriber::fmt::init();

    let h1: H1 = ModelStrut::test();
    let h1_write = ModelStrut::write(&h1);
    info!("Message: {:?}", h1_write);
    let h1_load = ModelStrut::load(&h1, "test/write/write.toml");
    info!("Message: {:#?}", h1_load);

    let text: Text = Text::new();
    let text_write = Text::write(&text);
    info!("Message: {:?}", text_write);
    let text_load = Text::load(&text, "test/text/text.toml");
    info!("Message: {:#?}", text_load);
}

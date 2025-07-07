use model::main_models::ModelStrut;

mod model;
mod service;

fn main() {
    tracing_subscriber::fmt::init();
    let mut h1 = model::h1::H1::new();
    model::h1::H1::test(&mut h1);

    // info!("{:#?}", h1);
    let _ = ModelStrut::load(&h1, "test/write/write.toml");
    let _ = ModelStrut::write(&h1);
}

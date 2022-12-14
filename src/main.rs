mod database;
mod discogs;
mod gui;
mod inputer;
mod models;
use inputer::inputer::InputReceiver;
use models::{
    app::{App, Navigation},
    error::Result,
    record::Label,
};

fn main() -> Result<()> {
    // return test();

    let receiver = inputer::inputer::start();

    let mut terminal = gui::terminal::start()?;

    let mut app = App::new()?;

    loop {
        app.render(&mut terminal)?;
        match app.input(receiver.recv()?)? {
            Navigation::Quit => break,
            _ => {}
        }
    }

    gui::terminal::end(&mut terminal)?;

    Ok(())
}

#[allow(dead_code)]
fn test() -> Result<()> {
    let original = Label {
        name: "Super Label".to_string(),
        catno: "Super duper".to_string(),
    };

    let cloned = original.clone();

    let serialized = serde_json::to_string(&original)?;
    let parsed = serde_json::from_str::<Label>(&serialized)?;

    println!("Cloned: {:?}", cloned == original);
    println!("Parsed: {:?}", parsed == original);

    Ok(())
}

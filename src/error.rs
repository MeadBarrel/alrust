use eframe::egui::Ui;
use crate::publicstate::PublicState;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not open file")]
    CouldNotOpenFile,
    #[error("Deserialization failed")]
    DeserializationFailed,
}


pub type Report = error_stack::Report<Error>;
pub type Result<T> = error_stack::Result<T, Error>;


pub fn handle_result(state: &mut PublicState, result: Result<()>) {
    match result {
        Ok(()) => (),
        Err(report) => handle_error(state, report),
    }
}

pub fn handle_error(state: &mut PublicState, report: Report) {
    let dialog = rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Error")
        .set_description(&format!("{}", report));
    dialog.show();
}

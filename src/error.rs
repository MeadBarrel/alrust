#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not open file")]
    CouldNotOpenFile,
    #[error("Deserialization failed")]
    DeserializationFailed,
}


pub type Report = error_stack::Report<Error>;
pub type Result<T> = error_stack::Result<T, Error>;


pub fn handle_error(report: Report) {
    let dialog = rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Error")
        .set_description(&format!("{}", report));
    dialog.show();        
}

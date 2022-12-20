use crate::error::Report;


#[derive(Debug, Default)]
pub struct Wishes {}


impl Wishes {
    pub fn handle_error(&self, report: Report) {
        let dialog = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Error")
            .set_description(&format!("{}", report));
        dialog.show();        
    }
}
use crate::error::Report;


#[derive(Debug, Default)]
pub struct Wishes {
    counter: usize,
}


impl Wishes {
    pub fn handle_error(&self, report: Report) {
        let dialog = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Error")
            .set_description(&format!("{}", report));
        dialog.show();        
    }

    pub fn counter(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
}
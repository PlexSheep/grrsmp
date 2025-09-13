#[derive(Debug, Default)]
pub(crate) struct TrackedWidgets {
    lbl_listener_status: Option<gtk::Label>,
}

impl TrackedWidgets {
    pub(crate) fn lbl_listener_status(&self) -> Option<&gtk::Label> {
        self.lbl_listener_status.as_ref()
    }

    pub(crate) fn set_lbl_listener_status(&mut self, lbl_listener_status: Option<gtk::Label>) {
        self.lbl_listener_status = lbl_listener_status;
    }
}

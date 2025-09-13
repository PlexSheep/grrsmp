#[derive(Debug, Default)]
pub(crate) struct TrackedWidgets {
    menu_item_listen_status: Option<gtk::gio::MenuItem>,
}

impl TrackedWidgets {
    pub(crate) fn set_menu_item_listen_status(
        &mut self,
        menu_item_listen_status: Option<gtk::gio::MenuItem>,
    ) {
        self.menu_item_listen_status = menu_item_listen_status;
    }

    pub(crate) fn menu_item_listen_status_mut(&mut self) -> &mut Option<gtk::gio::MenuItem> {
        &mut self.menu_item_listen_status
    }
}

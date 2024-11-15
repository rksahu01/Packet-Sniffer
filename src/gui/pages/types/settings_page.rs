
/// This enum defines the current settings page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SettingsPage {
    /// Settings Notifications page.
    Notifications,
    /// Settings Appearance page.
    Appearance,
    /// General settings.
    General,
}

impl SettingsPage {
    

   

    pub fn next(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::Appearance,
            SettingsPage::Appearance => SettingsPage::General,
            SettingsPage::General => SettingsPage::Notifications,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::General,
            SettingsPage::Appearance => SettingsPage::Notifications,
            SettingsPage::General => SettingsPage::Appearance,
        }
    }

   

}

#[cfg(test)]
mod tests {
    use crate::gui::pages::types::settings_page::SettingsPage;

    #[test]
    fn test_previous_settings_page() {
        assert_eq!(
            SettingsPage::Notifications.previous(),
            SettingsPage::General
        );
        assert_eq!(
            SettingsPage::Appearance.previous(),
            SettingsPage::Notifications
        );
        assert_eq!(SettingsPage::General.previous(), SettingsPage::Appearance);
    }

    #[test]
    fn test_next_settings_page() {
        assert_eq!(SettingsPage::Notifications.next(), SettingsPage::Appearance);
        assert_eq!(SettingsPage::Appearance.next(), SettingsPage::General);
        assert_eq!(SettingsPage::General.next(), SettingsPage::Notifications);
    }
}

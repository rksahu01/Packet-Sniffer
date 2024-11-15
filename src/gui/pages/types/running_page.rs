use crate::gui::types::message::Message;
use crate::translations::translations::overview_translation;
use crate::translations::translations_2::inspect_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

/// This enum defines the current GUI page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningPage {
    Init,
    Overview,
    Inspect,
}

impl RunningPage {
    pub const ALL: [RunningPage; 2] = [
        RunningPage::Overview,
        RunningPage::Inspect,
    ];

    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            RunningPage::Overview => overview_translation(language),
            RunningPage::Inspect => inspect_translation(language),
            RunningPage::Init => "",
        }
    }

    pub fn next(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Inspect,
            RunningPage::Inspect => RunningPage::Overview,
            RunningPage::Init => RunningPage::Init,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Inspect,
            RunningPage::Inspect => RunningPage::Overview,
            RunningPage::Init => RunningPage::Init,
        }
    }

    pub fn icon(self) -> iced::widget::Text<'static, StyleType> {
        match self {
            RunningPage::Overview => Icon::Overview,
            RunningPage::Inspect => Icon::Inspect,
            RunningPage::Init => Icon::Sniffnet,
        }
        .to_text()
    }

    pub fn action(self) -> Message {
        Message::ChangeRunningPage(self)
    }
}


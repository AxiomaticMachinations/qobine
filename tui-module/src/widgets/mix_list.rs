use std::sync::Arc;

use player_module::{
    AppResult,
    client::Client,
    database::Database,
    mixes::{DAILY_MIX_TITLE, WEEKLY_MIX_TITLE},
    notification::Notification,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::{
    app::{NotificationList, Output},
    popup::{PlaylistPopupState, Popup},
    ui::{COLUMN_SPACING, HIGHLIGHT_STYLE, SELECTED_STYLE, metadata_style, title_style},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mix {
    Daily,
    Weekly,
}

const MIXES: [Mix; 2] = [Mix::Daily, Mix::Weekly];

impl Mix {
    fn title(self) -> &'static str {
        match self {
            Mix::Daily => DAILY_MIX_TITLE,
            Mix::Weekly => WEEKLY_MIX_TITLE,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Mix::Daily => "Renewed daily from your listening history",
            Mix::Weekly => "Fresh discoveries every Friday",
        }
    }
}

pub struct MixList {
    database: Arc<Database>,
    state: TableState,
}

impl MixList {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database,
            state: TableState::default().with_selected(Some(0)),
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, focus: bool) {
        let rows: Vec<Row> = MIXES
            .iter()
            .map(|mix| {
                Row::new(vec![
                    Line::from(mix.title()),
                    Line::from(mix.description()).style(metadata_style()),
                ])
            })
            .collect();

        let table = Table::new(rows, [Constraint::Length(10), Constraint::Min(1)])
            .header(Row::new(["Mix", "Description"]).style(title_style()))
            .row_highlight_style(if focus {
                HIGHLIGHT_STYLE
            } else {
                SELECTED_STYLE
            })
            .column_spacing(COLUMN_SPACING);

        table.render(area, buf, &mut self.state);
    }

    pub async fn handle_events(
        &mut self,
        event: KeyCode,
        client: &Client,
        notifications: &mut NotificationList,
    ) -> AppResult<Output> {
        match event {
            KeyCode::Down | KeyCode::Char('j') => {
                self.state.select_next();
                Ok(Output::Consumed)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.state.select_previous();
                Ok(Output::Consumed)
            }
            KeyCode::Enter => {
                let Some(mix) = self.state.selected().and_then(|i| MIXES.get(i)).copied() else {
                    return Ok(Output::Consumed);
                };

                let playlist = match mix {
                    Mix::Daily => client.daily_mix(&self.database).await,
                    Mix::Weekly => client.weekly_mix().await,
                };

                match playlist {
                    Ok(playlist) if !playlist.tracks.is_empty() => Ok(Output::Popup(
                        Popup::Playlist(PlaylistPopupState::new_mix(playlist)),
                    )),
                    Ok(_) => {
                        notifications
                            .push(Notification::Warning(format!("{} is empty", mix.title())));
                        Ok(Output::Consumed)
                    }
                    Err(err) => {
                        notifications.push(Notification::Warning(format!(
                            "{} unavailable: {err}",
                            mix.title()
                        )));
                        Ok(Output::Consumed)
                    }
                }
            }
            _ => Ok(Output::NotConsumed),
        }
    }
}

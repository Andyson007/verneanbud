use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph, Wrap},
    Frame,
};

use ratatui::prelude::*;

use crate::{app::App, entities::sea_orm_active_enums::Issuekind};

pub fn render(app: &App, frame: &mut Frame, mainview: Rect, infoview: Rect) {
    render_select(app, frame, mainview);
    render_infoview(app, frame, infoview);
}

fn render_infoview(app: &App, frame: &mut Frame, view: Rect) {
    let raw_text = app
        .view_data
        .idea
        .selected
        .map(|x| &app.view_data.idea.ideas[x])
        .map_or_else(
            || "You need to select an element to view it's description.".to_string(),
            |selected_idea| selected_idea.description.clone(),
        );
    let widget = Paragraph::new(Text::from(raw_text))
        .wrap(Wrap { trim: false })
        .block(
            Block::bordered()
                .title("Description")
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

    frame.render_widget(widget, view);
}

fn render_select(app: &App, frame: &mut Frame, view: Rect) {
    let mut list_state = ListState::default().with_selected(app.view_data.idea.selected);
    let max_title_len = app
        .view_data
        .idea
        .ideas
        .iter()
        .map(|x| x.title.len())
        .max()
        .unwrap_or(0);

    let list = List::new(app.view_data.idea.ideas.iter().map(|idea| {
        let title = &idea.title;
        let state = if idea.solved { "solved" } else { "unsolved" };
        let kind = kind_str(&idea.kind).to_string();
        format!("{title:max_title_len$}ans | {state:10}| {kind}")
    }))
    .block(
        Block::bordered()
            .title("List")
            .style(Color::White)
            .border_type(ratatui::widgets::BorderType::Rounded),
    )
    .scroll_padding(3)
    .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(list, view, &mut list_state);
}

const fn kind_str(kind: &Issuekind) -> &str {
    match kind {
        Issuekind::Improvement => "Improvement",
        Issuekind::Issue => "Issue",
    }
}

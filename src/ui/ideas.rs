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
    if let Some(raw_text) = app
        .view_data
        .idea
        .selected
        .map(|x| &app.view_data.idea[x])
        .map(|selected_idea| selected_idea.0.get_entry().description.clone())
    {
        let widget = Paragraph::new(Text::from(
            raw_text
                .lines()
                .map(Span::raw)
                .chain([Span::styled(
                    "\u{2500}".repeat(50),
                    Style::new().fg(Color::Green),
                )])
                .map(Line::from)
                .collect::<Vec<_>>(),
        ))
        .wrap(Wrap { trim: false })
        .block(
            Block::bordered()
                .title("Description")
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
        frame.render_widget(widget, view);
    } else {
        let text = Paragraph::new("Select an entry to view").block(
            Block::bordered()
                .title("Description")
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
        frame.render_widget(text, view);
    }
}

fn render_select(app: &App, frame: &mut Frame, view: Rect) {
    let mut list_state = ListState::default().with_selected(app.view_data.idea.selected);
    let max_title_len = app
        .view_data
        .idea
        .ideas
        .iter()
        .map(|x| x.0.get_entry())
        .map(|x| x.title.len())
        .max()
        .unwrap_or(0);

    let max_author_len = app
        .view_data
        .idea
        .ideas
        .iter()
        .map(|x| x.0.get_entry())
        .map(|x| x.author.len())
        .max()
        .unwrap_or(0);

    let list = List::new(
        app.view_data
            .idea
            .ideas
            .iter()
            .map(|x| x.0.get_entry())
            .map(|idea| {
                let title = &idea.title;
                let kind = kind_str(&idea.kind).to_string();
                let author = idea.author.clone();
                let time = idea.time.format("%d/%m");
                // format!("{author}: {title:max_title_len$} | {state:10}| {kind}")
                Line::from(Vec::from([
                    Span::styled(format!("{time} "), Style::new().red()),
                    Span::raw(kind),
                    if idea.solved {
                        Span::styled(" \u{f41d} ", Style::new().magenta())
                    } else {
                        Span::styled(" \u{f41b} ", Style::new().green())
                    },
                    Span::styled(format!("{author:>max_author_len$}: "), Style::new().blue()),
                    Span::raw(format!("{title:max_title_len$} ")),
                ]))
            })
            .rev(),
    )
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

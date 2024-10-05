use std::iter;

use ratatui::{
    style::{Color, Style},
    widgets::{Block, List, ListState, Paragraph, Wrap},
    Frame,
};

use ratatui::prelude::*;

use crate::{
    app::App,
    entities::sea_orm_active_enums::Issuekind,
    view_data::{db_type::DbType, search_query::SearchQuery},
};

pub fn render(app: &App, frame: &mut Frame, mainview: Rect, infoview: Rect) {
    if let Some(ref search_query) = app.view_data.idea.search_query {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(mainview);
        render_select(app, frame, main_layout[0]);
        render_search(search_query, frame, main_layout[1]);
    } else {
        render_select(app, frame, mainview);
    }
    render_infoview(app, frame, infoview);
}

fn render_infoview(app: &App, frame: &mut Frame, view: Rect) {
    if let Some(selected_idea) = app.view_data.idea.current() {
        let raw_text = selected_idea.0.get_entry().description.clone();
        let widget = Paragraph::new(Text::from(
            raw_text
                .lines()
                .map(Span::raw)
                .chain([Span::styled(
                    "\u{2500}".repeat(50),
                    Style::new().fg(Color::Green),
                )])
                .chain(selected_idea.1.iter().map(DbType::get_entry).flat_map(|x| {
                    iter::once(Span::styled(
                        format!(
                            "{}, ({})",
                            x.author.clone(),
                            x.time.format("%d/%m/%Y [%H:%m]")
                        ),
                        Style::new().bold().underlined(),
                    ))
                    .chain(x.content.lines().map(|x| format!(" {x}")).map(Span::raw))
                }))
                .map(Line::from)
                .collect::<Vec<_>>(),
        ))
        .wrap(Wrap { trim: false })
        .block(
            Block::bordered()
                .title("Description")
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .scroll((selected_idea.2, 0));
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
    let ideas = app.view_data.idea.filtered_ideas();
    let mut list_state = ListState::default().with_selected(
        app.view_data
            .idea
            .selected
            .map(|x| ideas.clone().count() - x - 1),
    );
    let max_title_len = ideas.clone().map(|x| x.title.len()).max().unwrap_or(0);

    let max_author_len = ideas.clone().map(|x| x.author.len()).max().unwrap_or(0);

    let list = List::new(
        ideas
            .clone()
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

fn render_search(search_query: &SearchQuery, frame: &mut Frame, view: Rect) {
    frame.render_widget(Span::raw(format!("/{}", search_query.to_string())), view);
}

const fn kind_str(kind: &Issuekind) -> &str {
    match kind {
        Issuekind::Improvement => "Improvement",
        Issuekind::Issue => "Issue",
    }
}

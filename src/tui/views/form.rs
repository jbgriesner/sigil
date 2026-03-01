use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::app::{AppState, AppView, FormMode};

const LABELS: [&str; 6] = ["Name", "Username", "Password", "URL", "Tags", "Notes"];

pub fn render(f: &mut Frame, app: &AppState) {
    let AppView::Form {
        mode,
        draft,
        focused_field,
        show_password,
        error,
    } = &app.view
    else {
        return;
    };

    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let title = match mode {
        FormMode::Add => " New Secret ",
        FormMode::Edit(_) => " Edit Secret ",
    };

    let field_values: [&str; 6] = [
        &draft.name,
        &draft.username,
        &draft.password,
        &draft.url,
        &draft.tags,
        &draft.notes,
    ];

    let mut lines = vec![Line::from("")];

    for (i, (&label, &value)) in LABELS.iter().zip(field_values.iter()).enumerate() {
        let is_focused = i == *focused_field;

        // Mask password unless show_password is active.
        let display: String = if i == 2 && !show_password {
            "•".repeat(value.len())
        } else {
            value.to_string()
        };

        let label_style = if is_focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let value_style = if is_focused {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };

        let mut spans = vec![
            Span::styled(format!("  {:10}: ", label), label_style),
            Span::styled(format!("{display:<40}"), value_style),
        ];

        if i == 2 && is_focused {
            spans.push(Span::styled(
                "  [g] Generate  [Space] Toggle",
                Style::default().fg(Color::DarkGray),
            ));
        }

        lines.push(Line::from(spans));
        lines.push(Line::from(""));
    }

    if let Some(err) = error {
        lines.push(Line::from(Span::styled(
            format!("  ✗ {err}"),
            Style::default().fg(Color::Red),
        )));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);

    f.render_widget(Paragraph::new(lines).block(block), chunks[0]);

    f.render_widget(
        Paragraph::new("[Tab] Next field  [Shift+Tab] Prev  [Enter] Save  [Esc] Cancel")
            .style(Style::default().fg(Color::DarkGray)),
        chunks[1],
    );

    if app.generator_popup.is_some() {
        render_generator_popup(f, app);
    }
}

fn centered_fixed(w: u16, h: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    Rect {
        x,
        y,
        width: w.min(area.width),
        height: h.min(area.height),
    }
}

fn render_generator_popup(f: &mut Frame, app: &AppState) {
    let popup = match &app.generator_popup {
        Some(p) => p,
        None => return,
    };

    // 36 wide × 11 tall (9 content lines + 2 border lines)
    let popup_area = centered_fixed(36, 11, f.area());
    f.render_widget(Clear, popup_area);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    let (length_label_style, length_val_style) = if popup.focused == 0 {
        (
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            Style::default().fg(Color::White).bg(Color::DarkGray),
        )
    } else {
        (
            Style::default().fg(Color::DarkGray),
            Style::default().fg(Color::White),
        )
    };
    lines.push(Line::from(vec![
        Span::styled("  Length:  ", length_label_style),
        Span::styled(format!("[{:<10}]", popup.length_str), length_val_style),
    ]));
    lines.push(Line::from(""));

    lines.push(toggle_row(
        "Uppercase",
        popup.uppercase,
        popup.focused == 1,
        "Lowercase",
        popup.lowercase,
        popup.focused == 2,
    ));
    lines.push(toggle_row(
        "Digits  ",
        popup.digits,
        popup.focused == 3,
        "Symbols ",
        popup.symbols,
        popup.focused == 4,
    ));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(
            format!("▶ {} ◀", popup.preview),
            Style::default().fg(Color::Cyan),
        ),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "  [Enter] Apply   [Esc] Cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Generate password ");
    f.render_widget(Paragraph::new(lines).block(block), popup_area);
}

fn toggle_row(
    label1: &str,
    checked1: bool,
    focused1: bool,
    label2: &str,
    checked2: bool,
    focused2: bool,
) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = vec![Span::raw("  ")];
    spans.extend(toggle_spans(label1, checked1, focused1));
    spans.push(Span::raw("  "));
    spans.extend(toggle_spans(label2, checked2, focused2));
    Line::from(spans)
}

fn toggle_spans(label: &str, checked: bool, focused: bool) -> Vec<Span<'static>> {
    let checkbox: &'static str = if checked { "[x]" } else { "[ ]" };
    let checkbox_style = if focused {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else if checked {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    vec![
        Span::styled(checkbox, checkbox_style),
        Span::styled(
            format!(" {label}"),
            Style::default().fg(Color::White),
        ),
    ]
}

use anyhow::{bail, Result};
use byte_unit::Byte;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use magnet_url::Magnet;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};
use urlencoding::decode;

use crate::config::Config;

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::White)
    .add_modifier(Modifier::BOLD)
    .fg(Color::Black);

pub struct App {
    should_exit: bool,
    instance_list: InstanceList,
    magnet: Magnet,
    output_name: Option<String>,
}

struct InstanceList {
    instances: Vec<Instance>,
    state: ListState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
}

impl App {
    pub fn new(cfg: Config, magnet: Magnet, output_name: Option<String>) -> Self {
        Self {
            should_exit: false,
            instance_list: InstanceList {
                instances: cfg.instance,
                state: ListState::default(),
            },
            magnet,
            output_name,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.instance_list.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.instance_list.state.select_previous(),
            KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                let i = c.to_digit(10).unwrap() as usize - 1;
                if i < self.instance_list.instances.len() {
                    self.instance_list.state.select(Some(i));
                }
            }
            KeyCode::Char('l') | KeyCode::Enter => self.create_file().unwrap(),
            _ => {}
        }
    }

    fn create_file(&self) -> Result<()> {
        let Some(selected_idx) = self.instance_list.state.selected() else {
            bail!("No instance selected");
        };

        let instance = &self.instance_list.instances[selected_idx];

        let file_name: &str = match &self.output_name {
            Some(name) => name,
            None => self.magnet.dn.as_ref().unwrap(),
        };

        let mut file = File::create(instance.path.join(format!("{file_name}.magnet")))?;
        file.write_all(self.magnet.to_string().as_bytes())?;

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, info_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_info(info_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Select the instance to import to")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ or 0..9 to move, ↵ to select, and q to quit")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Instances").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN);

        let instances: Vec<ListItem> = self
            .instance_list
            .instances
            .iter()
            .map(ListItem::from)
            .collect();

        let list = List::new(instances)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("-> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.instance_list.state);
    }

    fn render_info(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Torrent Info").centered())
            .borders(Borders::ALL);

        Paragraph::new(self.build_info())
            .block(block)
            .render(area, buf);
    }

    fn build_info(&self) -> Vec<Line> {
        let mut info = vec![];

        if let Some(name) = &self.magnet.dn {
            info.push(Line::from(vec![
                "Name: ".bold(),
                decode(name).unwrap().into(),
            ]));
        }

        if let Some(size) = self.magnet.xl {
            let bytes = Byte::from_u64(size);
            info.push(Line::from(vec![
                "Size: ".bold(),
                format!("{bytes:#.10}").into(),
            ]))
        }

        let trackers = &self.magnet.tr;
        if !trackers.is_empty() {
            info.push(Line::from("Tracker(s):".bold()));
            for tr in trackers {
                info.push(Line::from(vec!["   -".into(), decode(tr).unwrap().into()]));
            }
        }

        info
    }
}

impl From<&Instance> for ListItem<'_> {
    fn from(value: &Instance) -> Self {
        ListItem::new(Line::styled(value.name.clone(), Color::White))
    }
}

use std::time::Duration;

use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{self, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListState},
};

use crate::sorting::{SortingAlgorithm, SortingStep, SortingTrace, bubble_sort};

const POLLING_DURATION_MS: u64 = 10;

#[derive(PartialEq)]
pub enum AppState {
    Main,
    Animation,
}

pub struct App {
    state: AppState,
    selected_state: ListState,
    array: Vec<u16>,
    current_step: Option<SortingStep>,
    step_tick: u16,
    trace: Option<SortingTrace>,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::Main,
            selected_state: ListState::default().with_selected(Some(0_usize)),
            array: (0..30).map(|_| rand::random_range(1..70)).collect(),
            current_step: None,
            step_tick: 0,
            trace: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(Duration::from_millis(POLLING_DURATION_MS))?
                && let Event::Key(key) = event::read()?
            {
                match self.state {
                    AppState::Main => match key.code {
                        KeyCode::Char('j') | KeyCode::Down => self.selected_state.select_next(),
                        KeyCode::Char('k') | KeyCode::Up => self.selected_state.select_previous(),
                        KeyCode::Char(' ') | KeyCode::Enter => self.state = AppState::Animation,
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    AppState::Animation => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::Main,
                        _ => {}
                    },
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::new(
            layout::Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(5),
                Constraint::Length(1),
            ],
        )
        .split(frame.area());

        let header_block = Block::default()
            .borders(Borders::NONE)
            .title(" KEIKO ")
            .title_alignment(layout::HorizontalAlignment::Center);
        let footer_block = Block::default().borders(Borders::NONE);

        let inner_layout = Layout::new(
            layout::Direction::Horizontal,
            [Constraint::Length(15), Constraint::Min(3)],
        )
        .split(layout[1]);

        let left_panel = Block::default()
            .borders(Borders::NONE)
            .title(" Method ")
            .title_alignment(layout::HorizontalAlignment::Center);

        let main_panel = Block::default()
            .borders(Borders::ALL)
            .title(" Animation ")
            .title_alignment(layout::HorizontalAlignment::Center);

        let sorting_list = List::new(Vec::<&str>::from(SortingAlgorithm::Bubble))
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        let mut bars: Vec<Bar> = self
            .array
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                Bar::default()
                    .value(v.into())
                    .label(i.to_string())
                    .style(Style::new().fg(Color::Gray))
                    .value_style(Style::new().bg(Color::Gray).fg(Color::Black))
            })
            .collect();

        if self.state == AppState::Animation
            && let Some(trace) = &mut self.trace
        {
            if let Some(step) = self.current_step {
                match step {
                    SortingStep::Swap(i, j) => match self.step_tick {
                        0..=10 => {
                            bars[i] = bars[i]
                                .clone()
                                .style(Style::new().fg(Color::Yellow))
                                .value_style(Style::new().bg(Color::Yellow));
                            bars[j] = bars[j]
                                .clone()
                                .style(Style::new().fg(Color::Yellow))
                                .value_style(Style::new().bg(Color::Yellow));
                        }
                        11..=20 => {
                            bars[i] = bars[i]
                                .clone()
                                .style(Style::new().fg(Color::LightYellow))
                                .value_style(Style::new().bg(Color::LightYellow));
                            bars[j] = bars[j]
                                .clone()
                                .style(Style::new().fg(Color::LightYellow))
                                .value_style(Style::new().bg(Color::LightYellow));
                        }
                        _ => {
                            self.array.swap(i, j);
                            self.current_step = None;
                            self.step_tick = 0;
                        }
                    },
                    SortingStep::Compare(i, j) => match self.step_tick {
                        0..=13 => {
                            bars[i] = bars[i]
                                .clone()
                                .style(Style::new().fg(Color::DarkGray))
                                .value_style(Style::new().bg(Color::DarkGray));
                            bars[j] = bars[j]
                                .clone()
                                .style(Style::new().fg(Color::DarkGray))
                                .value_style(Style::new().bg(Color::DarkGray));
                        }
                        14..=20 => {
                            if trace.is_next_swap() {
                                bars[i] = bars[i]
                                    .clone()
                                    .style(Style::new().fg(Color::Cyan))
                                    .value_style(Style::new().bg(Color::Cyan));
                                bars[j] = bars[j]
                                    .clone()
                                    .style(Style::new().fg(Color::Cyan))
                                    .value_style(Style::new().bg(Color::Cyan));
                            } else {
                                bars[i] = bars[i]
                                    .clone()
                                    .style(Style::new().fg(Color::Red))
                                    .value_style(Style::new().bg(Color::Red));
                                bars[j] = bars[j]
                                    .clone()
                                    .style(Style::new().fg(Color::Red))
                                    .value_style(Style::new().bg(Color::Red));
                            }
                        }
                        _ => {
                            self.current_step = None;
                            self.step_tick = 0;
                        }
                    },
                }
            } else {
                self.current_step = trace.next();
                if self.current_step.is_none() {
                    self.state = AppState::Main;
                }
            }

            self.step_tick = self.step_tick.saturating_add(1);
        } else {
            self.trace = Some(SortingTrace::new(&self.array, bubble_sort));
        }

        let chart = BarChart::default()
            .data(BarGroup::default().bars(&bars))
            .bar_width(2)
            .bar_gap(1)
            .max(100);

        frame.render_widget(header_block, layout[0]);
        frame.render_widget(&left_panel, inner_layout[0]);
        frame.render_stateful_widget(
            sorting_list.clone(),
            left_panel.inner(inner_layout[0]),
            &mut self.selected_state,
        );
        frame.render_widget(&main_panel, inner_layout[1]);
        frame.render_widget(
            chart,
            main_panel.inner(inner_layout[1]).centered(
                Constraint::Length(bars.len() as u16 * 2 + bars.len().saturating_sub(1) as u16), // n * width + (n - 1) * gap
                Constraint::Length(13),
            ),
        );

        frame.render_widget(footer_block, layout[2]);
    }
}

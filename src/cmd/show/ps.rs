use super::{
    common::{single_widget_loop, StatefulWidget},
    events::Config,
};
use anyhow::{anyhow, Result};
use rsys::linux::ps::{processes, Process};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Row, Table},
    Frame,
};

pub struct ProcessMonitor {
    processes: Vec<Process>,
}

impl StatefulWidget for ProcessMonitor {
    fn update(&mut self) {
        for process in &mut self.processes {
            if let Ok(_) = process.update() {}
        }
    }
    fn render_widget<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        self.render_storage_info_widget(f, chunks[0]);
    }
}

impl ProcessMonitor {
    pub fn new() -> Result<ProcessMonitor> {
        Ok(ProcessMonitor {
            processes: processes()?,
        })
    }

    fn render_storage_info_widget<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let headers = ["pid", "name", "state", "vsize", "rss", "utime", "stime"];
        let data = self.processes.iter().map(|s| {
            Row::StyledData(
                vec![
                    s.pid.to_string(),
                    s.name.to_string(),
                    s.state.to_string(),
                    s.vsize.to_string(),
                    s.rss.to_string(),
                    s.utime.to_string(),
                    s.stime.to_string(),
                ]
                .into_iter(),
                Style::default(),
            )
        });

        let table = Table::new(headers.iter(), data).widths(&[
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
        ]);

        f.render_widget(table, chunks[0]);
    }

    pub fn display_loop() -> Result<()> {
        let mut pmon = ProcessMonitor::new()?;
        single_widget_loop(&mut pmon, Config::default())
    }
}

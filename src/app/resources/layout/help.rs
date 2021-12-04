use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Tabs};
use crate::config_render::ConfigRender;

pub struct LayoutHelp {
    config_render: ConfigRender
}

impl LayoutHelp {

    pub fn new ()-> LayoutHelp {
        LayoutHelp{
            config_render: ConfigRender::new()
        }
    }

    pub fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {

        let config_render= &self.config_render;

        let size = frame.size();
        let panels = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(config_render.height_header),
                    Constraint::Min(config_render.min_height_main),
                    Constraint::Length(config_render.height_footer),
                ].as_ref(),
            )
            .split(size);

        frame.render_widget(self.render_tabs(), panels[0]);
        frame.render_widget(self.render_home(), panels[1]);
        frame.render_widget(self.render_footer(), panels[2]);
    }


    fn render_menu(&self)->Vec<Spans>{
        let menu= self.config_render.get_main_titles()
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        menu
    }


    fn render_tabs(&self)->Tabs {
        let tabs = Tabs::new(self.render_menu())
            .select(1)
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));
        tabs
    }


    fn render_footer(&self)->Paragraph {

        let copyright = Paragraph::new("Savne SAS 2021 - Todos los derechos reservados")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            );
        copyright
    }

    fn render_home(&self) -> Paragraph{
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Bienvenido")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("al")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "cliente gráfico FTP-CLI",
                Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Hola mundo como estaś")]),
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        home
    }


}



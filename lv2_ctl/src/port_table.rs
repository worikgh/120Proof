//! Make a table widget of control Port information

//use crate::lv2::ModHostController;
use crate::port::ControlPortProperties;
use crate::port::Port;
use crate::port::PortType;
use ratatui::layout::Constraint;
use ratatui::style::palette::tailwind;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::Cell;
use ratatui::widgets::HighlightSpacing;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use std::collections::HashMap;

// `val` is a String representing an integer.  Somewhere along the way
// it has become "0.000".  But in `labels_values` it is like ")".  Fix
// it up here
pub fn value_from_scale_control(val: &str) -> String {
    let i = val.parse::<usize>();
    let f = val.parse::<f64>();
    if i.is_ok() {
        format!("{}", i.unwrap())
    } else if f.is_ok() {
        format!("{}", f.unwrap().round())
    } else {
        "".to_string()
    }
}

pub fn port_table<'a>(
    ports: &[Port],
    pv: &HashMap<String, Option<String>>,
) -> Table<'a> {
    // Colours for the table
    let even_row_colour: Color = tailwind::SLATE.c950;
    let odd_row_colour: Color = tailwind::SLATE.c900;
    let row_fg: Color = tailwind::SLATE.c200;
    let buffer_bg: Color = tailwind::SLATE.c950;
    let selected_style_fg: Color = tailwind::BLUE.c400;

    let rows = ports.iter().enumerate().map(|(i, port)| {
        let colour = match i % 2 {
            0 => even_row_colour,
            _ => odd_row_colour,
        };

        // Set variables for the Port
        let (min, max, def, logarithmic) =
            if let Some(PortType::Control(control_port)) = port
                .types
                .iter()
                .find(|x| matches!(x, PortType::Control(_cp)))
            {
                match control_port {
                    ControlPortProperties::Continuous(cp) => {
                        let min = format!("{:2}", cp.min);
                        let max = format!("{:2}", cp.max);
                        let log = format!("{}", cp.logarithmic);
                        let val = match pv.get(port.symbol.as_str()) {
                            None => panic!("Cannot find {}", port.symbol),
                            Some(ov) => match ov {
                                Some(v) => v.clone(),
                                None => "".to_string(),
                            },
                        };
                        (min, max, val, log)
                    }
                    ControlPortProperties::Scale(scale) => {
                        let val: String = match pv.get(port.symbol.as_str()) {
                            None => panic!("Cannot find {}", port.symbol),
                            Some(ov) => match ov {
                                Some(v) => v.clone(),
                                None => {
                                    eprintln!(
                                        "DBG No port value for {}",
                                        port.symbol
                                    );
                                    "".to_string()
                                }
                            },
                        };

                        let value = value_from_scale_control(val.as_str());
                        let label: String = match scale
                            .labels_values
                            .iter()
                            .find(|ll| ll.1 == value)
                        {
                            None => format!("?{val}"),
                            Some(sv) => sv.0.clone(),
                        };
                        (
                            scale.labels_values[0].0.clone(),
                            scale
                                .labels_values
                                .last()
                                .expect("Expect some labels for port table")
                                .0
                                .clone(),
                            label,
                            "false".to_string(),
                        )
                    }
                }
            } else {
                panic!("A port in port table that is not a Controlport")
            };

        // The row itself as a styled row
        let item = [port.name.clone(), min, def, max, logarithmic];
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(row_fg).bg(colour))
            .height(2) // How does this react with the scroll bar?
    });

    // Find the longest name for assigning space in row
    let ln_name =
        ports
            .iter()
            .fold(0, |a, b| if a < b.name.len() { b.name.len() } else { a });
    let ln_name = ln_name as u16;

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(selected_style_fg);

    Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Min(ln_name + 1),
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Min(6),
        ],
    )
    .highlight_style(selected_style)
    .bg(buffer_bg)
    .highlight_spacing(HighlightSpacing::Always)
}

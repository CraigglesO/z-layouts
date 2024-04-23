use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    layouts: Vec<LayoutInfo>,
    filter: String,
    // The file name of the layout that is currently selected
    selected: Option<String>,
    ignore_case: bool,
}
// impl Default for State {
//     fn default() -> Self {
//         Self {
//             layouts: Vec::default(),
//             filter: String::default(),
//             selected: None,
//             ignore_case: false,
//         }
//     }
// }

impl State {
    fn filter(&self, layout: &&LayoutInfo) -> bool {
        if self.ignore_case {
            layout.name().to_lowercase() == self.filter.to_lowercase()
                || layout
                    .name()
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
        } else {
            layout.name() == self.filter || layout.name().contains(&self.filter)
        }
    }

    fn viewable_layouts_iter(&self) -> impl Iterator<Item = &LayoutInfo> {
        self.layouts.iter().filter(|layout| self.filter(layout))
    }

    fn viewable_layouts(&self) -> Vec<&LayoutInfo> {
        self.viewable_layouts_iter().collect()
    }

    fn reset_selection(&mut self) {
        let layouts = self.viewable_layouts();

        if layouts.is_empty() {
            self.selected = None
        } else if let Some(layout) = layouts.first() {
            self.selected = Some(layout.name().to_string())
        }
    }

    fn select_down(&mut self) {
        let layouts = self.layouts.iter().filter(|tab| self.filter(tab));

        let mut can_select = false;
        let mut first = None;
        for layout in layouts {
            let name = layout.name().to_string();
            if first.is_none() {
                first.replace(name.clone());
            }

            if can_select {
                self.selected = Some(name);
                return;
            } else if Some(name) == self.selected {
                can_select = true;
            }
        }

        if let Some(name) = first {
            self.selected = Some(name)
        }
    }

    fn select_up(&mut self) {
        let layouts = self.layouts.iter().filter(|tab| self.filter(tab)).rev();

        let mut can_select = false;
        let mut last = None;
        for layout in layouts {
            let name = layout.name().to_string();
            if last.is_none() {
                last.replace(name.clone());
            }

            if can_select {
                self.selected = Some(name);
                return;
            } else if Some(name) == self.selected {
                can_select = true;
            }
        }

        if let Some(name) = last {
            self.selected = Some(name)
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        // we need the ReadApplicationState permission to receive the ModeUpdate
        // events
        // we need the ChangeApplicationState permission to Change Zellij state (Panes, layouts and UI)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        self.ignore_case = match configuration.get("ignore_case" as &str) {
            Some(value) => value.trim().parse().unwrap(),
            None => true,
        };

        subscribe(&[EventType::SessionUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::SessionUpdate(session_infos, _) => {
                let first_session = session_infos[0].clone();
                let curr_session = session_infos
                    .into_iter()
                    .find(|session| session.is_current_session)
                    .unwrap_or(first_session);
                // filter out the current session and get it's layouts
                self.layouts = curr_session.available_layouts;
            }
            Event::Key(Key::Esc | Key::Ctrl('c')) => {
                close_focus();
            }

            Event::Key(Key::Down | Key::BackTab) => {
                self.select_down();

                should_render = true;
            }
            Event::Key(Key::Up | Key::Ctrl('k')) => {
                self.select_up();

                should_render = true;
            }
            Event::Key(Key::Char('\n')) => {
                let layout = self
                    .layouts
                    .iter()
                    .find(|layout| layout.name() == self.selected.as_ref().unwrap());

                if let Some(tlayout) = layout {
                    close_focus();
                    new_tabs_with_layout(tlayout.name());
                }
            }
            Event::Key(Key::Backspace) => {
                self.filter.pop();

                self.reset_selection();

                should_render = true;
            }
            Event::Key(Key::Char(c)) if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                self.filter.push(c);

                self.reset_selection();

                should_render = true;
            }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        println!(
            "{} {}",
            ">".cyan().bold(),
            if self.filter.is_empty() {
                "(filter)".dimmed().italic().to_string()
            } else {
                self.filter.dimmed().italic().to_string()
            }
        );

        println!(
            "{}",
            self.viewable_layouts_iter()
                .map(|layout| {
                    let row = layout.name().to_string();

                    if Some(layout.name()) == self.selected.as_deref() {
                        row.on_cyan().to_string()
                    } else {
                        row
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

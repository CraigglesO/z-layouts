use owo_colors::{AnsiColors, OwoColorize};
use std::collections::BTreeMap;
use std::fmt::Write;
use zellij_tile::prelude::*;

// #[derive(Default)]
struct State {
    layouts: Vec<LayoutInfo>,
    filter: String,
    quick_jump: bool,
    // The file name of the layout that is currently selected
    selected: Option<String>,
    ignore_case: bool,
    selection_color: AnsiColors,
    apply_selection_for_foreground_instead: bool,
    active_tab_color: Option<AnsiColors>,
    underline_active: bool,
    apply_active_color_for_background_instead: bool,
}
impl Default for State {
    fn default() -> Self {
        Self {
            layouts: Vec::default(),
            filter: String::default(),
            selected: None,
            ignore_case: false,
            quick_jump: false,
            selection_color: AnsiColors::Cyan,
            apply_selection_for_foreground_instead: true,
            active_tab_color: None,
            underline_active: false,
            apply_active_color_for_background_instead: false,
        }
    }
}

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
    fn load(&mut self, mut configuration: BTreeMap<String, String>) {
        // we need the ReadApplicationState permission to receive the ModeUpdate
        // events
        // we need the ChangeApplicationState permission to Change Zellij state (Panes, layouts and UI)
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        // self.ignore_case = match configuration.get("ignore_case" as &str) {
        //     Some(value) => value.trim().parse().unwrap(),
        //     None => true,
        // };

        if let Some(value) = configuration.remove("ignore_case") {
            self.ignore_case = value.trim().parse().unwrap_or_else(|_| {
                panic!(
                    "'ingnore_case' config value must be 'true' or 'false', but it's \"{value}\""
                )
            });
        };

        if let Some(quick_jump) = configuration.remove("quick_jump") {
            self.quick_jump = quick_jump.trim().parse().unwrap_or_else(|_| {
                panic!("'quick_jump' config value must be 'true' or 'false', but it's \"{quick_jump}\"")
            });
        }

        if let Some(color) = configuration.remove("selection_color") {
            // TODO: validate input
            self.selection_color = color.trim().into();
        }

        if let Some(x) = configuration.remove("apply_selection_accent_to") {
            match x.as_str() {
                "background" | "bg" => self.apply_selection_for_foreground_instead = false,
                "foreground" | "fg" => self.apply_selection_for_foreground_instead = true,
                _ => panic!("'apply_selection_accent_to' config value must be 'fg', 'foreground', 'bg' or 'background', but it's \"{x}\""),
            }
        }

        if let Some(color) = configuration.remove("active_tab_color") {
            // TODO: validate input
            let temp = color.trim();
            if temp == "none" {
                self.active_tab_color = None;
            } else {
                self.active_tab_color = Some(temp.into());
            }
        }

        if let Some(x) = configuration.remove("apply_tab_color_to") {
            match x.as_str() {
                "background" | "bg" => self.apply_active_color_for_background_instead = true,
                "foreground" | "fg" => self.apply_active_color_for_background_instead = false,
                _ => panic!("'apply_tab_color_to' config value must be 'fg', 'foreground', 'bg' or 'background', but it's \"{x}\""),
            }
        }

        if let Some(value) = configuration.remove("underline_active") {
            self.underline_active = value.trim().parse().unwrap_or_else(|_| {
                panic!(
                    "'underline_active' config value must be 'true' or 'false', but it's \"{value}\""
                )
            });
        };

        if !configuration.is_empty() {
            let stringified_map = configuration
                .iter()
                .fold(String::new(), |mut output, (k, v)| {
                    let _ = writeln!(output, "('{k}': '{v}')\n");
                    output
                });

            eprintln!("WARNING: The user added a config entry that isn't used.");

            eprint!("{stringified_map}");
        }

        // TODO: Get plugin position from config and update name to be visually nicer
        // rename_plugin_pane(0, "zellij-layouts");

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
                should_render = true;
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Esc => {
                    close_focus();
                }
                BareKey::Char('c') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    close_focus();
                }
                BareKey::Down => {
                    self.select_down();

                    should_render = true;
                }
                BareKey::Tab if key.has_no_modifiers() => {
                    self.select_down();

                    should_render = true;
                }
                BareKey::Char('n') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_down();

                    should_render = true;
                }
                BareKey::Up => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Tab if key.has_modifiers(&[KeyModifier::Shift]) => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Char('k') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_up();

                    should_render = true;
                }
                BareKey::Char('p') if key.has_modifiers(&[KeyModifier::Ctrl]) => {
                    self.select_up();

                    should_render = true;
                }

                BareKey::Enter => {
                    let selected = self.selected.clone().unwrap();
                    let layout = self.layouts.iter().find(|layout| layout.name() == selected);
                    eprintln!("SELECTED LAYOUT: {:?}", layout);

                    if let Some(t_layout) = layout {
                        close_focus();
                        new_tabs_with_layout_info(t_layout.clone());
                    }
                }

                BareKey::Backspace => {
                    self.filter.pop();
                    self.reset_selection();
                    should_render = true;
                }
                BareKey::Char(c) if c.is_ascii_digit() && self.quick_jump => {
                    close_focus();
                    switch_tab_to(c.to_digit(10).unwrap());
                }

                BareKey::Char(c) if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
                    self.filter.push(c);
                    self.reset_selection();
                    should_render = true;
                }
                _ => (),
            },

            // Event::Key(Key::Down | Key::BackTab) => {
            //     self.select_down();

            //     should_render = true;
            // }
            // Event::Key(Key::Up | Key::Ctrl('k')) => {
            //     self.select_up();

            //     should_render = true;
            // }
            // Event::Key(Key::Char('\n')) => {
            //     let selected = self.selected.clone().unwrap();
            //     let layout = self.layouts.iter().find(|layout| layout.name() == selected);
            //     eprintln!("SELECTED LAYOUT: {:?}", layout);

            //     if let Some(t_layout) = layout {
            //         close_focus();
            //         new_tabs_with_layout_info(t_layout.clone());
            //     }
            // }
            // Event::Key(Key::Backspace) => {
            //     self.filter.pop();

            //     self.reset_selection();

            //     should_render = true;
            // }
            // Event::Key(Key::Char(c)) if c.is_ascii_alphabetic() || c.is_ascii_digit() => {
            //     self.filter.push(c);

            //     self.reset_selection();

            //     should_render = true;
            // }
            _ => (),
        };

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        // TODO: use new variables
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

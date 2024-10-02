use std::cmp::min;

use super::App;

impl App {
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tab_data.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tab_data.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    fn tabs_len(&self) -> usize {
        self.tabs.len()
    }
    fn current_tab(&self) -> usize {
        self.curr_tab
    }
    pub fn next_tab(&mut self) {
        self.curr_tab = min(self.current_tab() + 1, self.tabs_len() - 1);
    }

    pub fn previous_tab(&mut self) {
        self.curr_tab = self.current_tab().saturating_sub(1);
    }
}

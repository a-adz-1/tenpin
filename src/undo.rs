//! A bounded undo history of whole-state snapshots, kept by the editor.

use std::collections::VecDeque;

pub struct History<T> {
    /// The state as of the last record, undo or redo.
    last: T,
    undo: VecDeque<T>,
    redo: Vec<T>,
    cap: usize,
}

impl<T: PartialEq> History<T> {
    pub fn new(now: T, cap: usize) -> Self {
        Self {
            last: now,
            undo: VecDeque::new(),
            redo: Vec::new(),
            cap,
        }
    }

    pub fn last(&self) -> &T {
        &self.last
    }

    /// Take in the current state. A change the user made becomes an undo step; one they didn't,
    /// like host automation, only moves the baseline.
    pub fn record(&mut self, now: T, by_user: bool) {
        if now == self.last {
            return;
        }
        let before = std::mem::replace(&mut self.last, now);
        if by_user {
            self.redo.clear();
            if self.undo.len() == self.cap {
                self.undo.pop_front();
            }
            self.undo.push_back(before);
        }
    }

    /// Step back, returning the state to restore.
    pub fn undo(&mut self) -> Option<&T> {
        let s = self.undo.pop_back()?;
        self.redo.push(std::mem::replace(&mut self.last, s));
        Some(&self.last)
    }

    pub fn redo(&mut self) -> Option<&T> {
        let s = self.redo.pop()?;
        self.undo.push_back(std::mem::replace(&mut self.last, s));
        Some(&self.last)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_and_redo_walk_the_edits() {
        let mut h = History::new(0, 10);
        h.record(1, true);
        h.record(2, true);
        assert_eq!(h.undo(), Some(&1));
        assert_eq!(h.undo(), Some(&0));
        assert_eq!(h.undo(), None);
        assert_eq!(h.redo(), Some(&1));
        assert_eq!(h.redo(), Some(&2));
        assert_eq!(h.redo(), None);
        assert_eq!(h.last(), &2);
    }

    #[test]
    fn a_new_edit_drops_the_redo_steps() {
        let mut h = History::new(0, 10);
        h.record(1, true);
        h.undo();
        h.record(5, true);
        assert!(!h.can_redo());
        assert_eq!(h.undo(), Some(&0));
    }

    #[test]
    fn changes_not_made_by_the_user_are_no_step() {
        let mut h = History::new(0, 10);
        h.record(1, true);
        h.record(1, true);
        h.record(7, false);
        // Undo goes to the state before the edit, not before the automation.
        assert_eq!(h.undo(), Some(&0));
        assert!(!h.can_undo());
        assert_eq!(h.redo(), Some(&7));
    }

    #[test]
    fn history_keeps_only_the_newest_steps() {
        let mut h = History::new(0, 3);
        (1..=5).for_each(|i| h.record(i, true));
        let mut back = vec![];
        while let Some(&s) = h.undo() {
            back.push(s);
        }
        assert_eq!(back, [4, 3, 2]);
    }
}

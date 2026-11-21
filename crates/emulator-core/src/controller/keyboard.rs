use std::collections::VecDeque;

use crate::controller::interrupt::ProgrammableInterruptController;

#[derive(Debug, Default)]
pub struct KeyboardController {
    data: Option<u8>,
    pending: VecDeque<u8>,
}

impl KeyboardController {
    pub fn new() -> Self {
        KeyboardController::default()
    }

    pub fn push_scan_code(&mut self, value: u8, pic: &mut ProgrammableInterruptController) {
        self.pending.push_back(value);
        self.next(pic);
    }

    pub(crate) fn read_data(&mut self, pic: &mut ProgrammableInterruptController) -> u8 {
        let scan_code = self.data.take().unwrap_or(0);
        self.next(pic);
        
        scan_code
    }

    fn next(&mut self, pic: &mut ProgrammableInterruptController) {
        if self.data.is_none() {
            if let Some(scan_code) = self.pending.pop_front() {
                self.data = Some(scan_code);
                pic.request(1);
            }
        }
    }
}

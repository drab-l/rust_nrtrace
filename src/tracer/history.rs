use std::collections::BTreeMap;

type HistData = peek::SyscallSummery;
type HistMapData = BTreeMap::<types::Pid, HistData>;
pub struct HistMap {
    data: HistMapData,
}

impl HistMap {
    pub fn new() -> HistMap {
        HistMap{data: HistMapData::new()}
    }

    fn update_entry(&mut self, pid: types::Pid, e: peek::SyscallInfoEntry) {
        match self.data.get_mut(&pid) {
            Some(d) => {
                d.renew_from_entry(e);
            },
            None => {
                self.data.insert(pid, peek::SyscallSummery::new_from_entry(e));
            },
        }
    }

    fn update_exit(&mut self, pid: types::Pid, e: peek::SyscallInfoExit) {
        if let Some(d) = self.data.get_mut(&pid) {
            d.add_exit(e);
        }
    }

    pub fn update(&mut self, pid: types::Pid, e: peek::SyscallInfo) -> Option<&HistData> {
        match e {
            peek::SyscallInfo::ENTRY(e) => self.update_entry(pid, e),
            peek::SyscallInfo::EXIT(e) => self.update_exit(pid, e),
        }
        self.data.get(&pid)
    }

    pub fn clear(&mut self, pid: types::Pid) {
        self.data.remove(&pid);
    }
}

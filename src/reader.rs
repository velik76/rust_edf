use crate::common_header::CommonHeader;
use crate::signal::Signal;
use std::fs::File;
use std::path::PathBuf;

pub struct Reader {
    cmn_header: CommonHeader,
    file_name: PathBuf,
    file: Option<File>,
    signals: Vec<Signal>,
}

impl Reader {
    pub fn new(_full_file_name: &String) -> Reader {
        Reader {
            cmn_header: CommonHeader::new(),
            file_name: PathBuf::from(_full_file_name),
            file: None,
            signals: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), &str> {
        self.file = Some(File::open(&self.file_name).unwrap());
        let mut _file = self.file.as_mut().unwrap();
        self.cmn_header.read_from_file(_file).unwrap();

        self.signals
            .resize(self.cmn_header.get_signals_count() as usize, Signal::new());

        for signal in self.signals.iter_mut() {
            signal.read_from_file_signal_label(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_transducer(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_phys_dimension(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_phys_min(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_phys_max(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_dig_min(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_dig_max(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_filter(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_samples_per_record(_file).unwrap();
        }

        for signal in self.signals.iter_mut() {
            signal.read_from_file_comment(_file).unwrap();
        }
        Ok(())
    }

    pub fn stop(&mut self) {}

    pub fn read_signals(&self, _start_sec: u32, _end_sec: u32) -> Vec<i16> {
        let result: Vec<i16> = Vec::new();
        result
    }
}

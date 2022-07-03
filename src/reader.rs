use crate::common_header::CommonHeader;
use crate::signal::Signal;
use std::fs::File;
// use std::io::*;
use std::path::PathBuf;
use std::result::Result;

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

    pub fn get_records_count(&self) -> u32 {
        self.cmn_header.get_records_count()
    }

    pub fn get_signals_count(&self) -> u32 {
        self.cmn_header.get_signals_count()
    }

    pub fn read_signals(&mut self, _start_record: u32, _records_cnt: u32) -> Result<(), &str> {
        /*         let mut _file = self.file.as_mut().unwrap();

               let mut _record_size: u64 = 0;
               for signal in self.signals.iter() {
                   _record_size = record_size + signal.get_signal_samples_per_record() as u64;
               }

               _file
                   .seek(SeekFrom::Start(
                       self.cmn_header.get_header_size() as u64 + _record_size as u64 * _start_record as u64 *,
                   ))
                   .unwrap();
        */
        Ok(())
        //        let result: Vec<i16> = Vec::new();
        //        &self.signals
    }
}

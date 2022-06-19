use crate::common_header::CommonHeader;
use crate::signal::Signal;
use crate::signal_info::SignalInfo;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct Writer {
    started: bool,
    cmn_header: CommonHeader,
    file_name: PathBuf,
    file: Option<File>,
    signals: Vec<Signal>,
    records: u32,
}

impl Writer {
    pub fn new(_full_file_name: &String) -> Writer {
        Writer {
            started: false,
            cmn_header: CommonHeader::new(),
            file_name: PathBuf::from(_full_file_name),
            file: None,
            signals: Vec::new(),
            records: 0,
        }
    }

    pub fn add_signal(&mut self, _signal_info: SignalInfo) -> usize {
        let signal: Signal = Signal::new(_signal_info);
        self.signals.push(signal);
        self.signals.len() - 1
    }

    pub fn start(
        &mut self,
        _patient_id: String,
        _record_id: String,
        _comment: String,
    ) -> Result<(), &str> {
        if self.signals.len() == 0 {
            return Err("No signals initialized");
        }

        self.file = Some(File::create(&self.file_name).unwrap());
        self.cmn_header
            .start(
                _patient_id,
                _record_id,
                _comment,
                1,
                self.signals.len() as u32,
            )
            .unwrap();

        self.cmn_header
            .dump2file(self.file.as_mut().unwrap())
            .unwrap();

        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_signal_label().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_transducer().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_phys_dimension().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_phys_min().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_phys_max().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_dig_min().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_dig_max().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_filter().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_samples_per_record().as_bytes())
                .unwrap();
        }
        for signal in self.signals.iter() {
            self.file
                .as_ref()
                .unwrap()
                .write_all(signal.get_string_comment().as_bytes())
                .unwrap();
        }

        let mut max_record_per_sample: u16 = 0;
        for signal in self.signals.iter() {
            if signal.get_samples_per_record() > max_record_per_sample {
                max_record_per_sample = signal.get_samples_per_record();
            }
        }

        for signal in self.signals.iter() {
            if max_record_per_sample % signal.get_samples_per_record() != 0 {
                return Err("Bad storing frequency for signal: ");
            }
        }

        for signal in self.signals.iter_mut() {
            signal.set_downsample_factor(max_record_per_sample / signal.get_samples_per_record());
        }

        self.records = 0;
        self.started = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), &str> {
        if self.started == false {
            return Ok(());
        }
        self.started = false;

        Ok(())
    }

    pub fn signals(&self) -> usize {
        self.signals.len()
    }

    pub fn set_samples(&mut self, _samples: &mut Vec<u16>) -> Result<(), &str> {
        if self.signals.len() == 0 {
            return Err("Not configured");
        }

        if _samples.len() != self.signals.len() {
            return Err("Not matching vector");
        }

        for i in 0.._samples.len() {
            self.signals[i].set_sample(_samples[i]).unwrap();
        }

        if self.signals[0].is_samples_buffer_full() {
            for signal in self.signals.iter_mut() {
                signal.write_values_to_file(self.file.as_mut().unwrap());
            }

            self.records = self.records + 1;
            self.cmn_header.setRecordsCount(self.records);
            self.cmn_header
                .dump2file(self.file.as_mut().unwrap())
                .unwrap();
        }

        Ok(())
    }
}

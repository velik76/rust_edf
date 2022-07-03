use crate::common_header::CommonHeader;
use crate::signal::Signal;
use crate::signal_info::SignalInfo;
use std::fs::File;
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

    pub fn add_signal(&mut self, _signal_info: SignalInfo) {
        let mut signal: Signal = Signal::new();
        signal.set_signal_info(_signal_info);
        self.signals.push(signal);
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
            .write_to_file(self.file.as_mut().unwrap())
            .unwrap();

        let mut _file = self.file.as_mut().unwrap();
        for signal in self.signals.iter() {
            signal.write_to_file_signal_label(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_transducer(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_phys_dimension(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_phys_min(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_phys_max(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_dig_min(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_dig_max(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_filter(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_samples_per_record(_file).unwrap();
        }
        for signal in self.signals.iter() {
            signal.write_to_file_comment(_file).unwrap();
        }

        let mut max_record_per_sample = 0;
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
            signal.set_write_downsample_factor(
                max_record_per_sample / signal.get_samples_per_record(),
            );
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

    pub fn set_samples(&mut self, _samples: &mut Vec<i16>) -> Result<(), &str> {
        if self.signals.len() == 0 {
            return Err("Not configured");
        }

        if _samples.len() != self.signals.len() {
            return Err("Not matching vector");
        }

        for i in 0.._samples.len() {
            self.signals[i].set_write_sample(_samples[i]).unwrap();
        }

        if self.signals[0].is_write_buffer_full() {
            let mut _file = self.file.as_mut().unwrap();

            for signal in self.signals.iter_mut() {
                signal.write_values_to_file(_file);
            }

            self.records = self.records + 1;
            self.cmn_header.set_records_count(self.records);
            self.cmn_header.write_to_file(_file).unwrap();
        }

        Ok(())
    }
}

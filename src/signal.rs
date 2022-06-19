use crate::signal_info::SignalInfo;
use std::fs::*;
use std::io::*;
use std::result::Result;
use std::{mem, slice};

pub struct Signal {
    signal_info: SignalInfo,
    downsample_factor: u16,
    actual_downsample_value: u16,
    samples: Vec<u16>,
}

impl Signal {
    pub fn new(_signal_info: SignalInfo) -> Signal {
        Signal {
            signal_info: _signal_info,
            downsample_factor: 0,
            actual_downsample_value: 0,
            samples: Vec::new(),
        }
    }

    pub fn get_string_signal_label(&self) -> String {
        self.signal_info.get_formatted_signal_info()
    }
    pub fn get_string_transducer(&self) -> String {
        self.signal_info.get_formatted_transducer()
    }
    pub fn get_string_phys_dimension(&self) -> String {
        self.signal_info.get_formatted_phys_dimension()
    }
    pub fn get_string_phys_min(&self) -> String {
        self.signal_info.get_formatted_phys_min()
    }
    pub fn get_string_phys_max(&self) -> String {
        self.signal_info.get_formatted_phys_max()
    }
    pub fn get_string_dig_min(&self) -> String {
        self.signal_info.get_formatted_dig_min()
    }
    pub fn get_string_dig_max(&self) -> String {
        self.signal_info.get_formatted_dig_max()
    }
    pub fn get_string_filter(&self) -> String {
        self.signal_info.get_formatted_filter()
    }
    pub fn get_string_samples_per_record(&self) -> String {
        self.signal_info.get_formatted_samples_per_record()
    }
    pub fn get_string_comment(&self) -> String {
        self.signal_info.get_formatted_comment()
    }

    pub fn get_samples_per_record(&self) -> u16 {
        self.signal_info.samples_per_record
    }

    pub fn set_downsample_factor(&mut self, _downsample_factor: u16) {
        self.downsample_factor = _downsample_factor;
    }

    pub fn set_sample(&mut self, _sample: u16) -> Result<(), &str> {
        self.actual_downsample_value = self.actual_downsample_value + 1;
        if self.actual_downsample_value < self.downsample_factor {
            return Ok(());
        }

        self.actual_downsample_value = 0;

        if self.samples.len() < self.signal_info.samples_per_record as usize {
            self.samples.push(_sample);
            Ok(())
        } else {
            Err("No place in samples buffer")
        }
    }

    pub fn is_samples_buffer_full(&self) -> bool {
        let len = self.samples.len();
        len >= self.signal_info.samples_per_record as usize
    }

    pub fn write_values_to_file(&mut self, _file: &mut File) {
        let slice_u8: &[u8] = unsafe {
            slice::from_raw_parts(
                self.samples.as_ptr() as *const u8,
                self.samples.len() * mem::size_of::<u16>(),
            )
        };
        _file.seek(std::io::SeekFrom::End(0)).unwrap();
        _file.write_all(slice_u8).unwrap();
        self.samples.clear();
    }
}

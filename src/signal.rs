use crate::signal_info::Element;
use crate::signal_info::SignalInfo;
use std::fs::*;
use std::io::*;
use std::result::Result;
use std::{mem, slice};

#[derive(Clone)]
struct WriteControl {
    downsample_factor: u16,
    actual_downsample_value: u16,
    samples: Vec<i16>,
}

impl WriteControl {
    pub fn new() -> WriteControl {
        WriteControl {
            downsample_factor: 0,
            actual_downsample_value: 0,
            samples: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct ReadControl {
    samples: Vec<i16>,
    start_record: u32,
    records_cnt: u32,
}

impl ReadControl {
    pub fn new() -> ReadControl {
        ReadControl {
            samples: Vec::new(),
            start_record: 0,
            records_cnt: 0,
        }
    }
}

#[derive(Clone)]
pub struct Signal {
    signal_info: SignalInfo,
    write_control: WriteControl,
    read_control: ReadControl,
}

impl Signal {
    pub fn new() -> Signal {
        Signal {
            signal_info: SignalInfo::new(),
            write_control: WriteControl::new(),
            read_control: ReadControl::new(),
        }
    }

    pub fn set_signal_info(&mut self, _signal_info: SignalInfo) {
        self.signal_info = _signal_info;
    }

    pub fn get_signal_samples_per_record(&self) -> u16 {
        self.signal_info.samples_per_record
    }

    pub fn write_to_file_signal_label(&self, _file: &mut File) -> Result<(), std::io::Error> {
        //        _file.write_all(self.get_string_signal_label().as_bytes())
        _file.write_all(self.signal_info.get_formatted(Element::Label).as_bytes())
    }
    pub fn read_from_file_signal_label(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::LABEL_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.label = std::str::from_utf8(&buffer).unwrap().trim().to_string();
        Ok(())
    }

    pub fn write_to_file_transducer(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_transducer().as_bytes())
    }
    pub fn read_from_file_transducer(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::TRANSDUCER_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.transducer = std::str::from_utf8(&buffer).unwrap().trim().to_string();
        Ok(())
    }

    pub fn write_to_file_phys_dimension(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_phys_dimension().as_bytes())
    }
    pub fn read_from_file_phys_dimension(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::PHYS_DIMENSION_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.phys_dimension = std::str::from_utf8(&buffer).unwrap().trim().to_string();
        Ok(())
    }

    pub fn write_to_file_phys_min(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_phys_min().as_bytes())
    }
    pub fn read_from_file_phys_min(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::PHYS_MIN_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.phys_min = std::str::from_utf8(&buffer)
            .unwrap()
            .trim()
            .parse::<i16>()
            .unwrap();
        Ok(())
    }

    pub fn write_to_file_phys_max(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_phys_max().as_bytes())
    }
    pub fn read_from_file_phys_max(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::PHYS_MAX_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.phys_max = std::str::from_utf8(&buffer)
            .unwrap()
            .trim()
            .parse::<i16>()
            .unwrap();
        Ok(())
    }

    pub fn write_to_file_dig_min(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_dig_min().as_bytes())
    }
    pub fn read_from_file_dig_min(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::DIG_MIN_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.dig_min = std::str::from_utf8(&buffer)
            .unwrap()
            .trim()
            .parse::<i16>()
            .unwrap();
        Ok(())
    }

    pub fn write_to_file_dig_max(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_dig_max().as_bytes())
    }
    pub fn read_from_file_dig_max(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::DIG_MAX_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.dig_max = std::str::from_utf8(&buffer)
            .unwrap()
            .trim()
            .parse::<i16>()
            .unwrap();
        Ok(())
    }

    pub fn write_to_file_filter(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_filter().as_bytes())
    }
    pub fn read_from_file_filter(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::FILTER_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.filter = std::str::from_utf8(&buffer).unwrap().trim().to_string();
        Ok(())
    }

    pub fn write_to_file_samples_per_record(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_samples_per_record().as_bytes())
    }
    pub fn read_from_file_samples_per_record(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::SAMPLES_PER_RECORD_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.samples_per_record = std::str::from_utf8(&buffer)
            .unwrap()
            .trim()
            .parse::<u16>()
            .unwrap();
        Ok(())
    }

    pub fn write_to_file_comment(&self, _file: &mut File) -> Result<(), std::io::Error> {
        _file.write_all(self.get_string_comment().as_bytes())
    }
    pub fn read_from_file_comment(&mut self, _file: &mut File) -> Result<(), &str> {
        let mut buffer = [0u8; crate::signal_info::COMMENT_LENGTH];
        _file.read_exact(&mut buffer).unwrap();
        self.signal_info.comment = std::str::from_utf8(&buffer).unwrap().trim().to_string();
        Ok(())
    }

    pub fn get_formatted(&self, _element: Element) -> String {
        self.signal_info.get_formatted(_element)
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

    pub fn set_write_downsample_factor(&mut self, _downsample_factor: u16) {
        self.write_control.downsample_factor = _downsample_factor;
    }

    pub fn set_write_sample(&mut self, _sample: i16) -> Result<(), &str> {
        self.write_control.actual_downsample_value = self.write_control.actual_downsample_value + 1;
        if self.write_control.actual_downsample_value < self.write_control.downsample_factor {
            return Ok(());
        }

        self.write_control.actual_downsample_value = 0;

        if self.write_control.samples.len() < self.signal_info.samples_per_record as usize {
            if _sample < self.signal_info.dig_min {
                self.write_control.samples.push(self.signal_info.dig_min);
            } else if _sample > self.signal_info.dig_max {
                self.write_control.samples.push(self.signal_info.dig_max);
            } else {
                self.write_control.samples.push(_sample);
            }
            Ok(())
        } else {
            Err("No place in samples buffer")
        }
    }

    #[inline(always)]
    pub fn is_write_buffer_full(&self) -> bool {
        let len = self.write_control.samples.len();
        len >= self.signal_info.samples_per_record as usize
    }

    pub fn write_values_to_file(&mut self, _file: &mut File) {
        let slice_u8: &[u8] = unsafe {
            slice::from_raw_parts(
                self.write_control.samples.as_ptr() as *const u8,
                self.write_control.samples.len() * mem::size_of::<u16>(),
            )
        };
        _file.seek(std::io::SeekFrom::End(0)).unwrap();
        _file.write_all(slice_u8).unwrap();
        self.write_control.samples.clear();
    }

    pub fn set_read_values(&mut self, _start_record: u32, _records_cnt: u32) {}
}

use chrono::{DateTime, Local};
use std::fs::File;
use std::io::*;
use std::result::Result;
use std::time::SystemTime;

pub const HEADER_LENGTH: usize = 256;

pub const VERSION_LENGTH: usize = 8;
pub const PATIENT_ID_LENGTH: usize = 80;
pub const RECORD_ID_LENGTH: usize = 80;
pub const START_DATE_LENGTH: usize = 8;
pub const START_TIME_LENGTH: usize = 8;
pub const HEADER_SIZE_LENGTH: usize = 8;
pub const COMMENT_LENGTH: usize = 44;
pub const RECORDS_COUNT_LENGTH: usize = 8;
pub const RECORD_DURATION_LENGTH: usize = 8;
pub const SIGNALS_COUNT_LENGTH: usize = 4;

pub const VERSION_OFFSET: usize = 0;
pub const PATIENT_ID_OFFSET: usize = VERSION_OFFSET + VERSION_LENGTH;
pub const RECORD_ID_OFFSET: usize = PATIENT_ID_OFFSET + PATIENT_ID_LENGTH;
pub const START_DATE_OFFSET: usize = RECORD_ID_OFFSET + RECORD_ID_LENGTH;
pub const START_TIME_OFFSET: usize = START_DATE_OFFSET + START_DATE_LENGTH;
pub const HEADER_SIZE_OFFSET: usize = START_TIME_OFFSET + START_TIME_LENGTH;
pub const COMMENT_OFFSET: usize = HEADER_SIZE_OFFSET + HEADER_SIZE_LENGTH;
pub const RECORDS_COUNT_OFFSET: usize = COMMENT_OFFSET + COMMENT_LENGTH;
pub const RECORD_DURATION_OFFSET: usize = RECORDS_COUNT_OFFSET + RECORDS_COUNT_LENGTH;
pub const SIGNALS_COUNT_OFFSET: usize = RECORD_DURATION_OFFSET + RECORD_DURATION_LENGTH;

pub struct CommonHeader {
    edf_version: u32,
    patient_id: String,
    record_id: String,
    start_date: String,
    start_time: String,
    header_size: u32,
    comment: String,
    records_count: u32,
    record_duration: u32,
    signals_count: u32,
}

impl CommonHeader {
    pub fn new() -> CommonHeader {
        CommonHeader {
            edf_version: 0,
            patient_id: String::new(),
            record_id: String::new(),
            start_date: String::new(),
            start_time: String::new(),
            header_size: 0,
            comment: String::new(),
            records_count: 0,
            record_duration: 0,
            signals_count: 0,
        }
    }

    pub fn start(
        &mut self,
        _patient_id: String,
        _record_id: String,
        _comment: String,
        _record_duration: u32,
        _signals_count: u32,
    ) -> Result<(), &str> {
        if _signals_count == 0 {
            return Err("No signals");
        }

        self.patient_id = _patient_id;
        self.record_id = _record_id;

        let datetime: DateTime<Local> = SystemTime::now().into();
        self.start_date = format!("{:<1$}", datetime.format("%d.%m.%y"), START_DATE_LENGTH);
        self.start_time = format!("{:<1$}", datetime.format("%H.%M.%S"), START_TIME_LENGTH);

        self.header_size = 256 * (_signals_count + 1);
        self.comment = _comment;
        self.record_duration = _record_duration;
        self.signals_count = _signals_count;

        Ok(())
    }

    pub fn set_records_count(&mut self, _records_count: u32) {
        self.records_count = _records_count;
    }
    pub fn get_records_count(&self) -> u32 {
        self.records_count
    }

    pub fn get_signals_count(&self) -> u32 {
        self.signals_count
    }

    pub fn get_header_size(&self) -> u32 {
        self.header_size
    }

    pub fn write_to_file(&mut self, _file: &mut File) -> Result<(), &str> {
        _file.seek(SeekFrom::Start(0)).unwrap();

        let mut var: String = format!("{:<1$}", self.edf_version, VERSION_LENGTH);
        var.push_str(&format!("{:<1$}", self.patient_id, PATIENT_ID_LENGTH));
        var.push_str(&format!("{:<1$}", self.record_id, RECORD_ID_LENGTH));
        var.push_str(&format!("{:<1$}", self.start_date, START_DATE_LENGTH));
        var.push_str(&format!("{:<1$}", self.start_time, START_TIME_LENGTH));
        var.push_str(&format!("{:<1$}", self.header_size, HEADER_SIZE_LENGTH));
        var.push_str(&format!("{:<1$}", self.comment, COMMENT_LENGTH));
        var.push_str(&format!("{:<1$}", self.records_count, RECORDS_COUNT_LENGTH));
        var.push_str(&format!(
            "{:<1$}",
            self.record_duration, RECORD_DURATION_LENGTH
        ));
        var.push_str(&format!("{:<1$}", self.signals_count, SIGNALS_COUNT_LENGTH));
        _file.write_all(var.as_bytes()).unwrap();

        Ok(())
    }

    pub fn read_from_file(&mut self, _file: &mut File) -> Result<(), &str> {
        _file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut buffer = [0u8; HEADER_LENGTH];
        _file.seek(SeekFrom::Start(0)).unwrap();
        _file.read_exact(&mut buffer).unwrap();

        self.edf_version =
            std::str::from_utf8(&buffer[VERSION_OFFSET..VERSION_OFFSET + VERSION_LENGTH])
                .unwrap()
                .trim()
                .parse::<u32>()
                .unwrap();

        self.patient_id =
            std::str::from_utf8(&buffer[PATIENT_ID_OFFSET..PATIENT_ID_OFFSET + PATIENT_ID_LENGTH])
                .unwrap()
                .to_string();

        self.record_id =
            std::str::from_utf8(&buffer[RECORD_ID_OFFSET..RECORD_ID_OFFSET + RECORD_ID_LENGTH - 1])
                .unwrap()
                .to_string();

        self.start_date =
            std::str::from_utf8(&buffer[START_DATE_OFFSET..START_DATE_OFFSET + START_DATE_LENGTH])
                .unwrap()
                .to_string();

        self.start_time =
            std::str::from_utf8(&buffer[START_TIME_OFFSET..START_TIME_OFFSET + START_TIME_LENGTH])
                .unwrap()
                .to_string();

        self.header_size = std::str::from_utf8(
            &buffer[HEADER_SIZE_OFFSET..HEADER_SIZE_OFFSET + HEADER_SIZE_LENGTH],
        )
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

        self.comment =
            std::str::from_utf8(&buffer[COMMENT_OFFSET..COMMENT_OFFSET + COMMENT_LENGTH])
                .unwrap()
                .to_string();

        self.records_count = std::str::from_utf8(
            &buffer[RECORDS_COUNT_OFFSET..RECORDS_COUNT_OFFSET + RECORDS_COUNT_LENGTH],
        )
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

        self.record_duration = std::str::from_utf8(
            &buffer[RECORD_DURATION_OFFSET..RECORD_DURATION_OFFSET + RECORD_DURATION_LENGTH],
        )
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

        self.signals_count = std::str::from_utf8(
            &buffer[SIGNALS_COUNT_OFFSET..SIGNALS_COUNT_OFFSET + SIGNALS_COUNT_LENGTH],
        )
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
        Ok(())
    }
}

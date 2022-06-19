use chrono::{DateTime, Local};
use std::fs::*;
use std::io::*;
use std::result::Result;
use std::time::SystemTime;

const VERSION_LENGTH: usize = 8;
const PATIENT_ID_LENGTH: usize = 80;
const RECORD_ID_LENGTH: usize = 80;
const START_DATE_LENGTH: usize = 8;
const START_TIME_LENGTH: usize = 8;
const HEADER_SIZE_LENGTH: usize = 8;
const COMMENT_LENGTH: usize = 44;
const RECORDS_COUNT_LENGTH: usize = 8;
const RECORD_DURATION_LENGTH: usize = 8;
const SIGNALS_COUNT_LENGTH: usize = 4;

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
        let a = CommonHeader {
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
        };
        a
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

    pub fn setRecordsCount(&mut self, _records_cont: u32) {
        self.records_count = _records_cont;
    }

    pub fn dump2file(&mut self, _file: &mut File) -> Result<(), &str> {
        _file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let mut var = format!("{:<1$}", self.edf_version, VERSION_LENGTH);
        var.truncate(VERSION_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();
        var = format!("{:<1$}", self.patient_id, PATIENT_ID_LENGTH);
        var.truncate(PATIENT_ID_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.record_id, RECORD_ID_LENGTH);
        var.truncate(RECORD_ID_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.start_date, START_DATE_LENGTH);
        var.truncate(START_DATE_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.start_time, START_TIME_LENGTH);
        var.truncate(START_TIME_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.header_size, HEADER_SIZE_LENGTH);
        var.truncate(HEADER_SIZE_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.comment, COMMENT_LENGTH);
        var.truncate(COMMENT_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.records_count, RECORDS_COUNT_LENGTH);
        var.truncate(RECORDS_COUNT_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.record_duration, RECORD_DURATION_LENGTH);
        var.truncate(RECORD_DURATION_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        var = format!("{:<1$}", self.signals_count, SIGNALS_COUNT_LENGTH);
        var.truncate(SIGNALS_COUNT_LENGTH);
        _file.write_all(var.as_bytes()).unwrap();

        Ok(())
    }
}

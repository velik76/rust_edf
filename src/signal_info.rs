const LABEL_LENGTH: usize = 16;
const TRANSDUCER_LENGTH: usize = 80;
const PHYS_DIMENSION_LENGTH: usize = 8;
const PHYS_MIN_LENGTH: usize = 8;
const PHYS_MAX_LENGTH: usize = 8;
const DIG_MIN_LENGTH: usize = 8;
const DIG_MAX_LENGTH: usize = 8;
const FILTER_LENGTH: usize = 80;
const SAMPLES_PER_RECORD_LENGTH: usize = 8;
const COMMENT_LENGTH: usize = 32;

#[derive(Clone)]
pub struct SignalInfo {
    pub label: &'static str,
    pub transducer: &'static str,
    pub phys_dimension: &'static str,
    pub phys_min: i16,
    pub phys_max: i16,
    pub dig_min: i16,
    pub dig_max: i16,
    pub filter: &'static str,
    pub samples_per_record: u16,
    pub comment: &'static str,
}

impl SignalInfo {
    pub fn get_formatted_signal_info(&self) -> String {
        format!("{:<1$}", self.label, LABEL_LENGTH)
    }

    pub fn get_formatted_transducer(&self) -> String {
        format!("{:<1$}", self.transducer, TRANSDUCER_LENGTH)
    }

    pub fn get_formatted_phys_dimension(&self) -> String {
        format!("{:<1$}", self.phys_dimension, PHYS_DIMENSION_LENGTH)
    }

    pub fn get_formatted_phys_min(&self) -> String {
        format!("{:<1$}", self.phys_min, PHYS_MIN_LENGTH)
    }

    pub fn get_formatted_phys_max(&self) -> String {
        format!("{:<1$}", self.phys_max, PHYS_MAX_LENGTH)
    }

    pub fn get_formatted_dig_min(&self) -> String {
        format!("{:<1$}", self.dig_min, DIG_MIN_LENGTH)
    }

    pub fn get_formatted_dig_max(&self) -> String {
        format!("{:<1$}", self.dig_max, DIG_MAX_LENGTH)
    }

    pub fn get_formatted_filter(&self) -> String {
        format!("{:<1$}", self.filter, FILTER_LENGTH)
    }

    pub fn get_formatted_samples_per_record(&self) -> String {
        format!("{:<1$}", self.samples_per_record, SAMPLES_PER_RECORD_LENGTH)
    }

    pub fn get_formatted_comment(&self) -> String {
        format!("{:<1$}", self.comment, COMMENT_LENGTH)
    }
}

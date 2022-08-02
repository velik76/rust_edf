pub const LABEL_LENGTH: usize = 16;
pub const TRANSDUCER_LENGTH: usize = 80;
pub const PHYS_DIMENSION_LENGTH: usize = 8;
pub const PHYS_MIN_LENGTH: usize = 8;
pub const PHYS_MAX_LENGTH: usize = 8;
pub const DIG_MIN_LENGTH: usize = 8;
pub const DIG_MAX_LENGTH: usize = 8;
pub const FILTER_LENGTH: usize = 80;
pub const SAMPLES_PER_RECORD_LENGTH: usize = 8;
pub const COMMENT_LENGTH: usize = 32;

pub enum Element {
    Label,
    Transducer,
    PhysDim,
    PhysMin,
    PhysMax,
    DigMin,
    DigMax,
    Filter,
    SamplesPerSec,
    Comment,
}

#[derive(Clone)]
pub struct SignalInfo {
    pub label: String,
    pub transducer: String,
    pub phys_dimension: String,
    pub phys_min: i16,
    pub phys_max: i16,
    pub dig_min: i16,
    pub dig_max: i16,
    pub filter: String,
    pub samples_per_record: u16,
    pub comment: String,
}

impl SignalInfo {
    pub fn new() -> SignalInfo {
        SignalInfo {
            label: String::new(),
            transducer: String::new(),
            phys_dimension: String::new(),
            phys_min: 0,
            phys_max: 0,
            dig_min: 0,
            dig_max: 0,
            filter: String::new(),
            samples_per_record: 0,
            comment: String::new(),
        }
    }

    pub fn get_formatted(&self, _element: Element) -> String {
        match _element {
            Element::Label => format!("{:<1$}", self.label, LABEL_LENGTH),
            Element::Transducer => format!("{:<1$}", self.transducer, TRANSDUCER_LENGTH),
            Element::PhysDim => format!("{:<1$}", self.phys_dimension, PHYS_DIMENSION_LENGTH),
            Element::PhysMin => format!("{:<1$}", self.phys_min, PHYS_MIN_LENGTH),
            Element::PhysMax => format!("{:<1$}", self.phys_max, PHYS_MAX_LENGTH),
            Element::DigMin => format!("{:<1$}", self.dig_min, DIG_MIN_LENGTH),
            Element::DigMax => format!("{:<1$}", self.dig_max, DIG_MAX_LENGTH),
            Element::Filter => format!("{:<1$}", self.filter, FILTER_LENGTH),
            Element::SamplesPerSec => {
                format!("{:<1$}", self.samples_per_record, SAMPLES_PER_RECORD_LENGTH)
            }
            Element::Comment => format!("{:<1$}", self.comment, COMMENT_LENGTH),
        }
    }

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

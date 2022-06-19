#[macro_use]
extern crate log;
use edf::signal_info::SignalInfo;
use edf::writer::Writer;

const SIGNAL_INFO1: SignalInfo = SignalInfo {
    label: "Signal1",
    transducer: "No Transducer",
    phys_dimension: "mV",
    phys_min: 0,
    phys_max: 100,
    dig_min: 0,
    dig_max: 100,
    filter: "No Filter",
    samples_per_record: 1,
    comment: "",
};

const SIGNAL_INFO2: SignalInfo = SignalInfo {
    label: "Signal2",
    transducer: "No Transducer",
    phys_dimension: "mV",
    phys_min: 0,
    phys_max: 100,
    dig_min: 0,
    dig_max: 100,
    filter: "No Filter",
    samples_per_record: 5,
    comment: "",
};

const SIGNAL_INFO3: SignalInfo = SignalInfo {
    label: "Signal3",
    transducer: "No Transducer",
    phys_dimension: "mV",
    phys_min: 0,
    phys_max: 100,
    dig_min: 0,
    dig_max: 100,
    filter: "No Filter",
    samples_per_record: 10,
    comment: "",
};

fn main() {
    let file_name: String = "./test.edf".to_string();
    let patient_id: String = "First Patient".to_string();
    let record_id: String = "NoId".to_string();
    let comment: String = "Rust EDF Writer".to_string();

    let mut writer: Writer = Writer::new(&file_name);

    writer.add_signal(SIGNAL_INFO1);
    writer.add_signal(SIGNAL_INFO2);
    writer.add_signal(SIGNAL_INFO3);

    info!("Elements: {}", writer.signals());

    writer.start(patient_id, record_id, comment).unwrap();

    for i in 0..1000 {
        let mut data: Vec<u16> = Vec::new();
        data.push(i);
        data.push(i);
        data.push(i);
        writer.set_samples(&mut data);
    }

    writer.stop().unwrap();
}

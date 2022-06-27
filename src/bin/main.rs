use edf::reader::Reader;
use edf::signal_info::SignalInfo;
use edf::writer::Writer;

fn get_signals_infos() -> Vec<SignalInfo> {
    let mut result: Vec<SignalInfo> = Vec::new();

    result.push(SignalInfo {
        label: "Signal1".to_string(),
        transducer: "No Transducer".to_string(),
        phys_dimension: "mV".to_string(),
        phys_min: 0,
        phys_max: 100,
        dig_min: 0,
        dig_max: 100,
        filter: "No Filter".to_string(),
        samples_per_record: 1,
        comment: "".to_string(),
    });
    result.push(SignalInfo {
        label: "Signal2".to_string(),
        transducer: "No Transducer".to_string(),
        phys_dimension: "mV".to_string(),
        phys_min: 0,
        phys_max: 100,
        dig_min: 0,
        dig_max: 100,
        filter: "No Filter".to_string(),
        samples_per_record: 5,
        comment: "".to_string(),
    });
    result.push(SignalInfo {
        label: "Signal3".to_string(),
        transducer: "No Transducer".to_string(),
        phys_dimension: "mV".to_string(),
        phys_min: 0,
        phys_max: 100,
        dig_min: 0,
        dig_max: 100,
        filter: "No Filter".to_string(),
        samples_per_record: 10,
        comment: "".to_string(),
    });
    result
}

fn main() {
    //    const signals_infos
    let file_name: String = "./test.edf".to_string();
    let patient_id: String = "First Patient".to_string();
    let record_id: String = "NoId".to_string();
    let comment: String = "Rust EDF Writer".to_string();

    let mut writer: Writer = Writer::new(&file_name);

    for signal_info in get_signals_infos() {
        writer.add_signal(signal_info);
    }

    writer.start(patient_id, record_id, comment).unwrap();

    for i in 0..1000 {
        let mut data: Vec<i16> = Vec::new();
        for _ in get_signals_infos() {
            data.push(i);
        }

        writer.set_samples(&mut data).unwrap();
    }

    writer.stop().unwrap();

    let mut reader: Reader = Reader::new(&file_name);
    reader.start().unwrap();
}

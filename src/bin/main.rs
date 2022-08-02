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

fn test_writer(_filename: &'static str) {
    let mut writer = Writer::new(&_filename.to_string());

    let patient_id = "First Patient";
    let record_id = "NoId";
    let comment = "Rust EDF Writer";

    for signal_info in get_signals_infos() {
        writer.add_signal(signal_info);
    }

    writer
        .start(
            &patient_id.to_string(),
            &record_id.to_string(),
            &comment.to_string(),
        )
        .unwrap();

    for i in 0..1000 {
        let mut data: Vec<i16> = Vec::new();
        for _ in get_signals_infos() {
            data.push(i % 100);
        }

        writer.set_samples(&mut data).unwrap();
    }

    writer.stop().unwrap();
}

fn test_reader(_filename: &'static str) {
    let mut reader = Reader::new(&_filename.to_string());
    reader.start().unwrap();
    println!("Records coung: {:?}", reader.get_records_count());
}

fn main() {
    let file_name = "./test.edf";
    test_writer(file_name);
    test_reader(file_name);
}

use std::{fs, fs::File, io::Write};

use defispring::api::{data_storage::update_api_data, processor::get_round_breakdown};
use serde_json::json;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

struct RawInputTestEnv {
    path: String,
}

impl RawInputTestEnv {
    fn new(path: &str) -> Self {
        fs::create_dir_all(path).expect("failed to create raw_input directory for tests");
        let env = Self {
            path: path.to_string(),
        };
        env.clear();
        env
    }

    fn clear(&self) {
        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    fs::remove_file(entry.path()).expect("failed to clean raw_input test files");
                }
            }
        }
    }

    fn write_round(&self, round: u8, allocations: &[(&str, &str)]) {
        let file = File::create(format!("{}/raw_{}.zip", &self.path, round))
            .expect("failed to create raw_input zip file");
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        zip.start_file(format!("raw_{}.json", round), options)
            .expect("failed to start file inside raw_input zip");

        let payload = allocations
            .iter()
            .map(|(address, amount)| json!({"address": address, "amount": amount}))
            .collect::<Vec<_>>();

        let serialized = serde_json::to_vec(&payload).expect("failed to serialize payload");
        zip.write_all(&serialized)
            .expect("failed to write payload to raw_input zip");
        zip.finish().expect("failed to finish raw_input zip");
    }
}

impl Drop for RawInputTestEnv {
    fn drop(&mut self) {
        self.clear();
        update_api_data();
    }
}

#[test]
fn round_breakdown_matches_expected_amounts() {
    let env = RawInputTestEnv::new("./raw_input");
    env.write_round(1, &[("0x1", "5"), ("0x2", "50")]);
    env.write_round(2, &[("0x2", "25")]);
    env.write_round(3, &[("0x3", "7")]);

    update_api_data();

    let second = get_round_breakdown("0x2").expect("expected breakdown for 0x2");
    assert_eq!(second.rounds.len(), 3);
    assert_eq!(second.rounds[0].round, 1);
    assert_eq!(second.rounds[0].amount, "50");
    assert_eq!(second.rounds[0].cumulative, "50");
    assert_eq!(second.rounds[1].round, 2);
    assert_eq!(second.rounds[1].amount, "25");
    assert_eq!(second.rounds[1].cumulative, "75");
    assert_eq!(second.rounds[2].round, 3);
    assert_eq!(second.rounds[2].amount, "0");
    assert_eq!(second.rounds[2].cumulative, "75");

    let third = get_round_breakdown("0x3").expect("expected breakdown for 0x3");
    assert_eq!(third.rounds.len(), 1);
    assert_eq!(third.rounds[0].round, 3);
    assert_eq!(third.rounds[0].amount, "7");
    assert_eq!(third.rounds[0].cumulative, "7");

    let missing = get_round_breakdown("0x4").expect("expected empty breakdown");
    assert!(missing.rounds.is_empty());

    assert!(get_round_breakdown("not_an_address").is_err());
}

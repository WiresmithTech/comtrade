use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::{FixedOffset, NaiveDate};
use float_cmp::approx_eq;

use comtrade::{
    AnalogChannel, AnalogScalingMode, Comtrade, ComtradeParserBuilder, DataFormat, FormatRevision,
    LeapSecondStatus, SamplingRate, StatusChannel, TimeQuality,
};

mod common;

use common::{assert_comtrades_eq, HOUR, MINUTE, SAMPLE_COMTRADE_DIR};

#[test]
fn it_correctly_parses_sample_2013_files_with_ascii_data_using_utf8() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_ascii_utf8.cfg");
    let dat_path = dir.join("sample_2013_ascii.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected_sample_rate = 1200.0;

    let expected = Comtrade {
        station_name: "SMARTSTATION testing text encoding: hgvcj터파크387".to_string(),
        recording_device_id: "IED123".to_string(),
        revision: FormatRevision::Revision2013,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: expected_sample_rate,
            end_sample_number: 40,
        }],
        start_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 750_110),
        trigger_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 782_610),
        data_format: DataFormat::Ascii,
        timestamp_multiplication_factor: 1.0,
        time_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        local_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        time_quality: Some(TimeQuality::ClockUnlocked(1)),
        leap_second_status: Some(LeapSecondStatus::NoCapability),
        num_analog_channels: 4,
        num_status_channels: 4,
        num_total_channels: 8,

        sample_numbers: (1..=40).collect(),
        timestamps: (0..40).map(|i| i as f64 / expected_sample_rate).collect(),

        analog_channels: vec![
            AnalogChannel {
                index: 1,
                name: "IA ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -9.39605712890625,
                    -1.65142822265625,
                    6.32098388671875,
                    13.95172119140625,
                    20.78521728515625,
                    26.02423095703125,
                    29.66876220703125,
                    30.92156982421875,
                    29.66876220703125,
                    26.02423095703125,
                    20.32965087890625,
                    12.92669677734375,
                    4.95428466796875,
                    -3.35980224609375,
                    -10.76275634765625,
                    -17.02679443359375,
                    -21.24078369140625,
                    -22.94915771484375,
                    -22.15191650390625,
                    -18.73516845703125,
                    -13.38226318359375,
                    -6.43487548828125,
                    1.19586181640625,
                    8.94049072265625,
                    15.77398681640625,
                    21.35467529296875,
                    24.99920654296875,
                    26.25201416015625,
                    25.22698974609375,
                    21.81024169921875,
                    16.34344482421875,
                    9.50994873046875,
                    1.99310302734375,
                    -5.63763427734375,
                    -12.58502197265625,
                    -18.27960205078125,
                    -22.15191650390625,
                    -23.63250732421875,
                    -22.60748291015625,
                    -19.19073486328125,
                ],
            },
            AnalogChannel {
                index: 2,
                name: "IB ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    7.80157470703125,
                    0.62640380859375,
                    -5.97930908203125,
                    -10.87664794921875,
                    -13.49615478515625,
                    -13.72393798828125,
                    -11.78778076171875,
                    -7.68768310546875,
                    -2.10699462890625,
                    4.49871826171875,
                    11.44610595703125,
                    18.05181884765625,
                    23.51861572265625,
                    26.93536376953125,
                    28.41595458984375,
                    27.73260498046875,
                    24.88531494140625,
                    20.10186767578125,
                    14.06561279296875,
                    7.00433349609375,
                    -0.17083740234375,
                    -6.89044189453125,
                    -12.47113037109375,
                    -16.34344482421875,
                    -18.05181884765625,
                    -18.05181884765625,
                    -15.77398681640625,
                    -11.90167236328125,
                    -6.32098388671875,
                    0.28472900390625,
                    7.00433349609375,
                    13.49615478515625,
                    18.84906005859375,
                    22.49359130859375,
                    24.20196533203125,
                    23.86029052734375,
                    21.35467529296875,
                    17.02679443359375,
                    11.33221435546875,
                    4.72650146484375,
                ],
            },
            AnalogChannel {
                index: 3,
                name: "IC ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    0.85418701171875,
                    0.51251220703125,
                    0.05694580078125,
                    -0.17083740234375,
                    -0.74029541015625,
                    -1.19586181640625,
                    -1.53753662109375,
                    -1.87921142578125,
                    -1.99310302734375,
                    -2.10699462890625,
                    -2.10699462890625,
                    -1.76531982421875,
                    -1.30975341796875,
                    -0.51251220703125,
                    0.28472900390625,
                    0.74029541015625,
                    1.30975341796875,
                    1.87921142578125,
                    2.10699462890625,
                    2.22088623046875,
                    1.99310302734375,
                    1.53753662109375,
                    1.08197021484375,
                    0.51251220703125,
                    -0.17083740234375,
                    -0.74029541015625,
                    -1.19586181640625,
                    -1.53753662109375,
                    -1.76531982421875,
                    -1.87921142578125,
                    -1.65142822265625,
                    -1.42364501953125,
                    -0.96807861328125,
                    -0.39862060546875,
                    0.28472900390625,
                    0.74029541015625,
                    1.30975341796875,
                    1.76531982421875,
                    1.99310302734375,
                    2.10699462890625,
                ],
            },
            AnalogChannel {
                index: 4,
                name: "3I0".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -0.85418701171875,
                    -0.62640380859375,
                    0.28472900390625,
                    2.79034423828125,
                    6.43487548828125,
                    10.87664794921875,
                    16.22955322265625,
                    21.24078369140625,
                    25.45477294921875,
                    28.30206298828125,
                    29.66876220703125,
                    29.09930419921875,
                    26.93536376953125,
                    23.06304931640625,
                    17.82403564453125,
                    11.21832275390625,
                    4.84039306640625,
                    -1.08197021484375,
                    -6.09320068359375,
                    -9.62384033203125,
                    -11.67388916015625,
                    -12.01556396484375,
                    -10.30718994140625,
                    -7.00433349609375,
                    -2.56256103515625,
                    2.44866943359375,
                    7.91546630859375,
                    12.69891357421875,
                    17.02679443359375,
                    20.10186767578125,
                    21.58245849609375,
                    21.46856689453125,
                    19.64630126953125,
                    16.45733642578125,
                    11.78778076171875,
                    6.09320068359375,
                    0.51251220703125,
                    -4.95428466796875,
                    -9.39605712890625,
                    -12.47113037109375,
                ],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "51A".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 2,
                name: "51B".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 3,
                name: "51C".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            StatusChannel {
                index: 4,
                name: "51N".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
        ],
    };

    assert_comtrades_eq(&expected, &record);
}

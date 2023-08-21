use ble_data_class::data_types::{complete_local_name::CompleteLocalName, data_type::DataType};

#[test]
fn test_is_complete_local_name() {
    // assert!(is_complete_local_name(complete_local_name::DATA_TYPE));
    // assert!(!is_complete_local_name(1));

    let name = "complete_local_name";
    let result =
        {
            let complete_local_name = name.to_string();
            CompleteLocalName {
                length: complete_local_name.as_bytes().len() as u8 + 1,
                complete_local_name: complete_local_name.to_string(),
            }
        };

    let mut data: Vec<u8> = Vec::new();
    data.push(name.as_bytes().len() as u8 + 1);
    data.push(ble_data_class::data_types::complete_local_name::CompleteLocalName::data_type());
    data.append(&mut name.to_string().into_bytes());
    let into_data: Vec<u8> = result.into();
    assert_eq!(data, into_data);
}

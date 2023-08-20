use ble_data_class::vec_converter::VecConverter;

#[test]
fn test_is_complete_local_name() {
    // assert!(is_complete_local_name(complete_local_name::DATA_TYPE));
    // assert!(!is_complete_local_name(1));

    let name = "complete_local_name";
    let result =
        ble_data_class::data_types::complete_local_name::CompleteLocalName::new(name.to_string());

    let mut data: Vec<u8> = Vec::new();
    data.push(name.as_bytes().len() as u8);
    data.push(ble_data_class::data_types::complete_local_name::DATA_TYPE);
    data.append(&mut name.to_string().into_bytes());
    assert_eq!(data, result.into_bytes());
}

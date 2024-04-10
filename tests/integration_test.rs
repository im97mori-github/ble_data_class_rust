use uuid::{uuid, Uuid};

#[test]
fn test_uuid() {
    let expected = uuid!("00000001-a087-4fa3-add4-3b8a7d5d4921");
    let bytes = [
        0x21u8, 0x49u8, 0x5Du8, 0x7Du8, 0x8Au8, 0x3Bu8, 0xD4u8, 0xADu8, 0xA3u8, 0x4Fu8, 0x87u8,
        0xA0u8, 0x01u8, 0x00u8, 0x00u8, 0x00u8,
    ];
    assert_eq!(expected.as_u128().to_le_bytes().to_vec(), bytes.to_vec());
    let serialized = u128::from_le_bytes(bytes);
    assert_eq!(expected, Uuid::from_u128(serialized));
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use ble_data_struct::{
        data_types::data_type_parser::DataTypeParseResult,
        descriptors::client_characteristic_configuration::ClientCharacteristicConfiguration,
        windows::buffer::{i_buffer_to_vec, vec_to_i_buffer},
    };
    use std::{
        sync::mpsc::{self},
        time::{self, Duration},
    };
    use uuid::{uuid, Uuid};

    use windows::{
        core::GUID,
        Devices::Bluetooth::{
            Advertisement::{
                BluetoothLEAdvertisementFlags, BluetoothLEAdvertisementReceivedEventArgs,
                BluetoothLEAdvertisementWatcher, BluetoothLEManufacturerData,
                BluetoothLEScanningMode,
            },
            BluetoothLEDevice,
            GenericAttributeProfile::GattClientCharacteristicConfigurationDescriptorValue,
        },
        Foundation::{
            AsyncOperationCompletedHandler, AsyncStatus, IAsyncOperation, TypedEventHandler,
        },
    };

    #[test]
    fn test_guid() {
        let expected = "adab0001-6e7d-4601-bda2-bffaa68956ba";
        let guid = GUID::from(expected);
        let uuid = Uuid::from_u128(guid.to_u128());
        assert_eq!(expected, uuid.to_string());
    }

    #[test]
    #[ignore]
    fn test_try_from() {
        let watcher = BluetoothLEAdvertisementWatcher::new().unwrap();
        let function =
            |_: &Option<BluetoothLEAdvertisementWatcher>,
             args: &Option<BluetoothLEAdvertisementReceivedEventArgs>| {
                let advertisement = match args {
                    Some(val) => val,
                    None => panic!("called `Option::unwrap()` on a `None` value"),
                }
                .Advertisement()
                .unwrap();

                let data_sections = advertisement.DataSections().unwrap();

                let mut guids: Vec<Uuid> = advertisement
                    .ServiceUuids()
                    .unwrap()
                    .into_iter()
                    .map(|f| Uuid::from_u128(f.to_u128()))
                    .collect();
                guids.sort();

                let mut manufacturer_data: Vec<BluetoothLEManufacturerData> = advertisement
                    .ManufacturerData()
                    .unwrap()
                    .into_iter()
                    .collect();

                let data_section_size = data_sections.Size().unwrap();
                for data_section_index in 0..data_section_size {
                    let data_section = data_sections.GetAt(data_section_index).unwrap();
                    let data_type_parse_result = DataTypeParseResult::from(data_section);
                    match data_type_parse_result {
                        DataTypeParseResult::CompleteLocalNameResult(result) => {
                            let actual = match result {
                                Ok(name) => name.complete_local_name,
                                _ => panic!(),
                            };
                            println!("{}", actual);
                            let expected = advertisement.LocalName().unwrap().to_string();
                            println!("\t{}", actual);
                            assert_eq!(expected, actual);
                        }
                        DataTypeParseResult::ShortenedLocalNameResult(result) => {
                            let actual = match result {
                                Ok(name) => name.shortened_local_name,
                                _ => panic!(),
                            };
                            println!("{}", actual);
                            let expected = advertisement.LocalName().unwrap().to_string();
                            println!("\t{}", actual);
                            assert_eq!(expected, actual);
                        }
                        DataTypeParseResult::FlagsResult(result) => match result {
                            Ok(flags) => {
                                let actual = [
                                    flags.is_le_limited_discoverable_mode(),
                                    flags.is_le_general_discoverable_mode(),
                                    flags.is_br_edr_not_supported(),
                                    flags.is_simultaneous_controller(),
                                ];
                                println!("{},{},{},{}", actual[0], actual[1], actual[2], actual[3]);

                                let bluetooth_le_advertisement_flags: BluetoothLEAdvertisementFlags =
                            advertisement.Flags().unwrap().Value().unwrap();
                                let expected = [
                                    bluetooth_le_advertisement_flags.contains(
                                        BluetoothLEAdvertisementFlags::LimitedDiscoverableMode,
                                    ),
                                    bluetooth_le_advertisement_flags.contains(
                                        BluetoothLEAdvertisementFlags::GeneralDiscoverableMode,
                                    ),
                                    bluetooth_le_advertisement_flags.contains(
                                        BluetoothLEAdvertisementFlags::ClassicNotSupported,
                                    ),
                                    bluetooth_le_advertisement_flags.contains(
                                        BluetoothLEAdvertisementFlags::DualModeControllerCapable,
                                    ),
                                ];

                                println!(
                                    "\t{},{},{},{}",
                                    expected[0], expected[1], expected[2], expected[3]
                                );
                                assert_eq!(expected, actual);
                            }
                            _ => panic!(),
                        },
                        DataTypeParseResult::CompleteListOf128BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => {
                                            guids.remove(x);
                                        }
                                        _ => {
                                            println!("uuid:{}", f);
                                            guids
                                                .to_vec()
                                                .into_iter()
                                                .for_each(|x| println!("\tguid:{}", x));
                                        }
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::CompleteListOf16BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => guids.remove(x),
                                        _ => panic!(),
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::CompleteListOf32BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => guids.remove(x),
                                        _ => panic!(),
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::IncompleteListOf128BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => {
                                            guids.remove(x);
                                        }
                                        _ => {
                                            println!("uuid:{}", f);
                                            guids
                                                .to_vec()
                                                .into_iter()
                                                .for_each(|x| println!("\tguid:{}", x));
                                        }
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::IncompleteListOf16BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => guids.remove(x),
                                        _ => panic!(),
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::IncompleteListOf32BitServiceUuidsResult(result) => {
                            match result {
                                Ok(uuids) => uuids.uuids.into_iter().for_each(|f| {
                                    match guids.binary_search(&f) {
                                        Ok(x) => guids.remove(x),
                                        _ => panic!(),
                                    };
                                }),
                                _ => panic!(),
                            }
                        }
                        DataTypeParseResult::ManufacturerSpecificDataResult(result) => match result
                        {
                            Ok(data) => {
                                println!("{:02x}", data.company_identifier);
                                for (index, value) in
                                    manufacturer_data.to_vec().into_iter().enumerate()
                                {
                                    if data.company_identifier == value.CompanyId().unwrap() {
                                        let vec = i_buffer_to_vec(value.Data().unwrap()).unwrap();
                                        if vec == data.manufacturer_specific_data.to_vec() {
                                            manufacturer_data.remove(index);
                                        }
                                    }
                                }
                            }
                            _ => panic!(),
                        },
                        DataTypeParseResult::AdvertisingIntervalResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::AdvertisingIntervalLongResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::AppearanceResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::BigInfoResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::BroadcastCodeResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::ChannelMapUpdateIndicationResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::ClassOfDeviceResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::EncryptedDataResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::LeBluetoothDeviceAddressResult(result) => match result
                        {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::LeRoleResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::LeSecureConnectionsConfirmationValueResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::LeSecureConnectionsRandomValueResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::LeSupportedFeaturesResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::ListOf128BitServiceSolicitationUUIDsResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::ListOf16BitServiceSolicitationUUIDsResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::ListOf32BitServiceSolicitationUUIDsResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::PeriodicAdvertisingResponseTimingInformationResult(
                            result,
                        ) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::PeripheralConnectionIntervalRangeResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::PublicTargetAddressResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::RandomTargetAddressResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::SecureSimplePairingHashC192Result(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::SecureSimplePairingHashC256Result(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::SecureSimplePairingRandomizerR192Result(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::SecureSimplePairingRandomizerR256Result(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::SecurityManagerOutOfBandResult(result) => match result
                        {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::SecurityManagerTkValueResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::ServiceData128BitUUIDResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::ServiceData16BitUUIDResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::ServiceData32BitUUIDResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::TxPowerLevelResult(result) => match result {
                            Err(_) => panic!(),
                            _ => {}
                        },
                        DataTypeParseResult::UniformResourceIdentifierResult(result) => {
                            match result {
                                Err(_) => panic!(),
                                _ => {}
                            }
                        }
                        DataTypeParseResult::DataTypeParseError(_) => {}
                    };
                }

                if !guids.is_empty() {
                    println!("{}", "guid not removed:".to_string());
                    guids.to_vec().into_iter().for_each(|x| println!("\t{}", x));
                }

                if !manufacturer_data.is_empty() {
                    println!("{}", "manufacturer data not removed:".to_string());
                    manufacturer_data
                        .to_vec()
                        .into_iter()
                        .for_each(|x| println!("\t{}", x.CompanyId().unwrap()));
                }

                windows::core::Result::Ok(())
            };
        let handler: TypedEventHandler<
            BluetoothLEAdvertisementWatcher,
            BluetoothLEAdvertisementReceivedEventArgs,
        > = TypedEventHandler::new(function);

        let token = watcher.Received(&handler).unwrap();
        println!("{}", token.Value);

        watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)
            .unwrap();

        watcher.Start().unwrap();

        let duration = time::Duration::from_secs(5);
        std::thread::sleep(duration);

        watcher.Stop().unwrap();
    }

    fn wait_operation<TResult: windows::core::RuntimeType + core::marker::Send>(
        operation: IAsyncOperation<TResult>,
        duration: Duration,
    ) -> TResult {
        let (tx, rx) = mpsc::channel();

        let function = move |option: core::option::Option<&IAsyncOperation<TResult>>,
                             async_status: AsyncStatus| {
            if async_status == AsyncStatus::Completed {
                tx.send(option.unwrap().GetResults().unwrap()).unwrap();
            }

            windows::core::Result::Ok(())
        };
        let handler = AsyncOperationCompletedHandler::new(function);
        operation.SetCompleted(&handler).unwrap();

        let result = rx.recv_timeout(duration).unwrap();
        operation.Close().unwrap();
        result
    }

    #[test]
    #[ignore]
    fn test_descriptors() {
        let duration = time::Duration::from_secs(5);
        let (tx, rx) = mpsc::channel();

        let watcher = BluetoothLEAdvertisementWatcher::new().unwrap();
        let function =
            move |_: &Option<BluetoothLEAdvertisementWatcher>,
                  option_arg: &Option<BluetoothLEAdvertisementReceivedEventArgs>| {
                let args = match option_arg {
                    Some(val) => val,
                    None => panic!("called `Option::unwrap()` on a `None` value"),
                };

                let advertisement = args.Advertisement().unwrap();

                if advertisement
                    .ServiceUuids()
                    .unwrap()
                    .into_iter()
                    .map(|f| Uuid::from_u128(f.to_u128()))
                    .any(|f| f == uuid!("00001810-0000-1000-8000-00805f9b34fb"))
                {
                    tx.send(args.BluetoothAddress().unwrap()).unwrap();
                }

                windows::core::Result::Ok(())
            };
        let handler: TypedEventHandler<
            BluetoothLEAdvertisementWatcher,
            BluetoothLEAdvertisementReceivedEventArgs,
        > = TypedEventHandler::new(function);

        let token = watcher.Received(&handler).unwrap();
        println!("{}", token.Value);

        watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)
            .unwrap();
        watcher.Start().unwrap();

        std::thread::sleep(duration);

        watcher.Stop().unwrap();
        println!("stopped");

        let blp_address = rx.recv_timeout(duration).unwrap();
        let operation = BluetoothLEDevice::FromBluetoothAddressAsync(blp_address).unwrap();
        let device = wait_operation(operation, duration);

        let operation = device
            .GetGattServicesForUuidAsync(GUID::from("00001810-0000-1000-8000-00805f9b34fb"))
            .unwrap();
        let service_result = wait_operation(operation, duration);
        let service = service_result.Services().unwrap().GetAt(0).unwrap();

        let operation = service
            .GetCharacteristicsForUuidAsync(GUID::from("00002a35-0000-1000-8000-00805f9b34fb"))
            .unwrap();

        let characteristic_result = wait_operation(operation, duration);
        let characteristic = characteristic_result
            .Characteristics()
            .unwrap()
            .GetAt(0)
            .unwrap();

        let operation = characteristic
            .GetDescriptorsForUuidAsync(GUID::from("00002902-0000-1000-8000-00805f9b34fb"))
            .unwrap();
        let descriptor_result = wait_operation(operation, duration);
        let descriptor = descriptor_result.Descriptors().unwrap().GetAt(0).unwrap();

        let operation = descriptor.ReadValueAsync().unwrap();
        let read_descriptor_result = wait_operation(operation, duration);
        let cccd_vec = i_buffer_to_vec(read_descriptor_result.Value().unwrap()).unwrap();
        let cccd = ClientCharacteristicConfiguration::try_from(&cccd_vec).unwrap();

        assert!(!cccd.is_notification());
        assert!(!cccd.is_indication());

        let operation = characteristic
            .ReadClientCharacteristicConfigurationDescriptorAsync()
            .unwrap();
        let read_client_characteristic_configuration_descriptor_result =
            wait_operation(operation, duration);
        let gatt_client_characteristic_configuration_descriptor =
            read_client_characteristic_configuration_descriptor_result
                .ClientCharacteristicConfigurationDescriptor()
                .unwrap();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::None,
            gatt_client_characteristic_configuration_descriptor
        );

        let operation = characteristic
            .WriteClientCharacteristicConfigurationDescriptorWithResultAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Indicate,
            )
            .unwrap();
        wait_operation(operation, duration);

        let operation = descriptor.ReadValueAsync().unwrap();
        let read_descriptor_result = wait_operation(operation, duration);
        let cccd_vec = i_buffer_to_vec(read_descriptor_result.Value().unwrap()).unwrap();
        let cccd = ClientCharacteristicConfiguration::try_from(&cccd_vec).unwrap();

        assert!(!cccd.is_notification());
        assert!(cccd.is_indication());

        let operation = characteristic
            .ReadClientCharacteristicConfigurationDescriptorAsync()
            .unwrap();
        let read_client_characteristic_configuration_descriptor_result =
            wait_operation(operation, duration);
        let gatt_client_characteristic_configuration_descriptor =
            read_client_characteristic_configuration_descriptor_result
                .ClientCharacteristicConfigurationDescriptor()
                .unwrap();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::Indicate,
            gatt_client_characteristic_configuration_descriptor
        );

        let vec: Vec<u8> = ClientCharacteristicConfiguration::new(0).into();
        let operation = descriptor
            .WriteValueAsync(&vec_to_i_buffer(&vec).unwrap())
            .unwrap();
        wait_operation(operation, duration);

        let operation = descriptor.ReadValueAsync().unwrap();
        let read_descriptor_result = wait_operation(operation, duration);
        let cccd_vec = i_buffer_to_vec(read_descriptor_result.Value().unwrap()).unwrap();
        let cccd = ClientCharacteristicConfiguration::try_from(&cccd_vec).unwrap();

        assert!(!cccd.is_notification());
        assert!(!cccd.is_indication());

        let operation = characteristic
            .ReadClientCharacteristicConfigurationDescriptorAsync()
            .unwrap();
        let read_client_characteristic_configuration_descriptor_result =
            wait_operation(operation, duration);
        let gatt_client_characteristic_configuration_descriptor =
            read_client_characteristic_configuration_descriptor_result
                .ClientCharacteristicConfigurationDescriptor()
                .unwrap();
        assert_eq!(
            GattClientCharacteristicConfigurationDescriptorValue::None,
            gatt_client_characteristic_configuration_descriptor
        );
    }
}

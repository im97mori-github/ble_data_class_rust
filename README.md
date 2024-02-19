# BLE data struct for Rust
data structs for ble advertising/descriptor/characteristic

Some as [https://github.com/im97mori-github/JavaBLEUtil/wiki](https://github.com/im97mori-github/JavaBLEUtil/wiki)

- [ ] Advertising(includes EIR/AD/SRD/OOB)
    - [x] Flags(0x01)
    - [x] Incomplete List of 16-bit Service Class UUIDs(0x02)
    - [x] Complete List of 16-bit Service Class UUIDs(0x03)
    - [x] Incomplete List of 32-bit Service Class UUIDs(0x04)
    - [x] Complete List of 32-bit Service Class UUIDs(0x05)
    - [x] Incomplete List of 128-bit Service Class UUIDs(0x06)
    - [x] Complete List of 128-bit Service Class UUIDs(0x07)
    - [x] Shortened Local Name(0x08)
    - [x] Complete Local Name(0x09)
    - [x] Tx Power Level(0x0A)
    - [x] Class of Device(0x0D)
    - [x] Simple Pairing Hash C-192(0x0E)
    - [x] Simple Pairing Randomizer R-192(0x0F)
    - [ ] Device ID(0x10)
    - [x] Security Manager TK Value(0x10)
    - [x] Security Manager Out of Band Flags(0x11)
    - [x] Peripheral Connection Interval Range(0x12)
    - [x] List of 16-bit Service Solicitation UUIDs(0x14)
    - [x] List of 128-bit Service Solicitation UUIDs(0x15)
    - [x] Service Data - 16-bit UUID(0x16)
    - [x] Public Target Address(0x17)
    - [x] Random Target Address(0x18)
    - [x] Appearance(0x19)
    - [x] Advertising Interval(0x1A)
    - [x] LE Bluetooth Device Address(0x1B)
    - [x] LE Role(0x1C)
    - [x] Simple Pairing Hash C-256(0x1D)
    - [x] Simple Pairing Randomizer R-256(0x1E)
    - [x] List of 32-bit Service Solicitation UUIDs(0x1F)
    - [x] Service Data - 32-bit UUID(0x20)
    - [x] Service Data - 128-bit UUID(0x21)
    - [x] LE Secure Connections Confirmation Value(0x22)
    - [x] LE Secure Connections Random Value(0x23)
    - [x] URI(0x24)
    - [ ] Indoor Positioning(0x25)
    - [ ] Transport Discovery Data(0x26)
    - [x] LE Supported Features(0x27)
    - [x] Channel Map Update Indication(0x28)
    - [ ] PB-ADV(0x29)
    - [ ] Mesh Message(0x2A)
    - [ ] Mesh Beacon(0x2B)
    - [x] BIGInfo(0x2C)
    - [x] Broadcast_Code(0x2D)
    - [ ] Resolvable Set Identifier(0x2E)
    - [x] Advertising Interval - long(0x2F)
    - [ ] Broadcast_Name(0x30)
    - [x] Encrypted Advertising Data(0x31)
    - [x] Periodic Advertising Response Timing Information(0x32)
    - [ ] Electronic Shelf Label(0x34)
    - [ ] 3D Information Data(0x3D)
    - [x] Manufacturer Specific Data(0xFF)
- [ ] Descriptor
    - [x] Characteristic Extended Properties(0x2900)
    - [ ] Characteristic User Description(0x2901)
    - [x] Client Characteristic Configuration(0x2902)
    - [ ] Server Characteristic Configuration(0x2903)
    - [ ] Characteristic Presentation Format(0x2904)
    - [ ] Characteristic Aggregate Format(0x2905)
    - [ ] Valid Range(0x2906)
    - [ ] External Report Reference(0x2907)
    - [ ] Report Reference(0x2908)
    - [ ] Number of Digitals(0x2909)
    - [ ] Value Trigger Setting(0x290A)
    - [ ] Environmental Sensing Configuration(0x290B)
    - [ ] Environmental Sensing Measurement(0x290C)
    - [ ] Environmental Sensing Trigger Setting(0x290D)
    - [ ] Time Trigger Setting(0x290E)
    - [ ] Complete BR-EDR Transport Block Data(0x290F)
    - [ ] Observation Schedule(0x2910)
    - [ ] Valid Range and Accuracy(0x2911)
- [ ] Characteristic
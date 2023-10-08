//! BIGInfo (Data Type Value:0x2c) module.

use crate::data_types::data_type::DataType;

/// BIGInfo.
#[derive(Debug, PartialEq, Clone)]
pub struct BigInfo {
    /// data length
    pub length: u8,

    /// BIG_Offset
    pub big_offset: u16,

    /// BIG_Offset_Units
    pub big_offset_units: bool,

    /// ISO_Interval
    pub iso_interval: u16,

    /// Num_BIS
    pub num_bis: u8,

    /// NSE
    pub nse: u8,

    /// BN
    pub bn: u8,

    /// Sub_Interval
    pub sub_interval: u32,

    /// PTO
    pub pto: u8,

    /// BIS_Spacing
    pub bis_spacing: u32,

    /// IRC
    pub irc: u8,

    /// Max_PDU
    pub max_pdu: u8,

    /// RFU
    pub rfu: u8,

    /// SeedAccessAddress
    pub seed_access_address: u32,

    /// SDU_Interval
    pub sdu_interval: u32,

    /// Max_SDU
    pub max_sdu: u16,

    /// BaseCRCInit
    pub base_crc_init: u16,

    /// ChM
    pub ch_m: u64,

    /// PHY
    pub phy: u8,

    /// bisPayloadCount
    pub bis_payload_count: u64,

    /// Framing
    pub framing: bool,

    /// GIV
    pub giv: Option<[u8; 8]>,

    /// GSKD
    pub gskd: Option<[u8; 16]>,
}

impl BigInfo {
    /// Create [`BigInfo`] from Parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::big_info::BigInfo;
    ///
    /// let big_offset: u16 = 1;
    /// let big_offset_units: bool = true;
    /// let iso_interval: u16 = 2;
    /// let num_bis: u8 = 3;
    /// let nse: u8 = 4;
    /// let bn: u8 = 5;
    /// let sub_interval: u32 = 6;
    /// let pto: u8 = 7;
    /// let bis_spacing: u32 = 8;
    /// let irc: u8 = 9;
    /// let max_pdu: u8 = 10;
    /// let rfu: u8 = 11;
    /// let seed_access_address: u32 = 12;
    /// let sdu_interval: u32 = 13;
    /// let max_sdu: u16 = 14;
    /// let base_crc_init: u16 = 15;
    /// let ch_m: u64 = 16;
    /// let phy: u8 = 17;
    /// let bis_payload_count: u64 = 18;
    /// let framing: bool = false;
    /// let giv: Option<[u8; 8]> = None;
    /// let gskd: Option<[u8; 16]> = None;
    /// let result = BigInfo::new(
    ///     big_offset,
    ///     big_offset_units,
    ///     iso_interval,
    ///     num_bis,
    ///     nse,
    ///     bn,
    ///     sub_interval,
    ///     pto,
    ///     bis_spacing,
    ///     irc,
    ///     max_pdu,
    ///     rfu,
    ///     seed_access_address,
    ///     sdu_interval,
    ///     max_sdu,
    ///     base_crc_init,
    ///     ch_m,
    ///     phy,
    ///     bis_payload_count,
    ///     framing,
    ///     giv,
    ///     gskd,
    /// );
    /// assert_eq!(34, result.length);
    /// assert_eq!(big_offset, result.big_offset);
    /// assert_eq!(big_offset_units, result.big_offset_units);
    /// assert_eq!(iso_interval, result.iso_interval);
    /// assert_eq!(num_bis, result.num_bis);
    /// assert_eq!(nse, result.nse);
    /// assert_eq!(bn, result.bn);
    /// assert_eq!(sub_interval, result.sub_interval);
    /// assert_eq!(pto, result.pto);
    /// assert_eq!(bis_spacing, result.bis_spacing);
    /// assert_eq!(irc, result.irc);
    /// assert_eq!(max_pdu, result.max_pdu);
    /// assert_eq!(rfu, result.rfu);
    /// assert_eq!(seed_access_address, result.seed_access_address);
    /// assert_eq!(sdu_interval, result.sdu_interval);
    /// assert_eq!(max_sdu, result.max_sdu);
    /// assert_eq!(base_crc_init, result.base_crc_init);
    /// assert_eq!(ch_m, result.ch_m);
    /// assert_eq!(phy, result.phy);
    /// assert_eq!(bis_payload_count, result.bis_payload_count);
    /// assert_eq!(framing, result.framing);
    /// assert_eq!(giv, result.giv);
    /// assert_eq!(gskd, result.gskd);
    ///
    /// let giv: Option<[u8; 8]> = Some([19, 0, 0, 0, 0, 0, 0, 0]);
    /// let gskd: Option<[u8; 16]> = Some([20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// let result = BigInfo::new(
    ///     big_offset,
    ///     big_offset_units,
    ///     iso_interval,
    ///     num_bis,
    ///     nse,
    ///     bn,
    ///     sub_interval,
    ///     pto,
    ///     bis_spacing,
    ///     irc,
    ///     max_pdu,
    ///     rfu,
    ///     seed_access_address,
    ///     sdu_interval,
    ///     max_sdu,
    ///     base_crc_init,
    ///     ch_m,
    ///     phy,
    ///     bis_payload_count,
    ///     framing,
    ///     giv,
    ///     gskd,
    /// );
    /// assert_eq!(58, result.length);
    /// assert_eq!(big_offset, result.big_offset);
    /// assert_eq!(big_offset_units, result.big_offset_units);
    /// assert_eq!(iso_interval, result.iso_interval);
    /// assert_eq!(num_bis, result.num_bis);
    /// assert_eq!(nse, result.nse);
    /// assert_eq!(bn, result.bn);
    /// assert_eq!(sub_interval, result.sub_interval);
    /// assert_eq!(pto, result.pto);
    /// assert_eq!(bis_spacing, result.bis_spacing);
    /// assert_eq!(irc, result.irc);
    /// assert_eq!(max_pdu, result.max_pdu);
    /// assert_eq!(rfu, result.rfu);
    /// assert_eq!(seed_access_address, result.seed_access_address);
    /// assert_eq!(sdu_interval, result.sdu_interval);
    /// assert_eq!(max_sdu, result.max_sdu);
    /// assert_eq!(base_crc_init, result.base_crc_init);
    /// assert_eq!(ch_m, result.ch_m);
    /// assert_eq!(phy, result.phy);
    /// assert_eq!(bis_payload_count, result.bis_payload_count);
    /// assert_eq!(framing, result.framing);
    /// assert_eq!(giv, result.giv);
    /// assert_eq!(gskd, result.gskd);
    /// ```
    pub fn new(
        big_offset: u16,
        big_offset_units: bool,
        iso_interval: u16,
        num_bis: u8,
        nse: u8,
        bn: u8,
        sub_interval: u32,
        pto: u8,
        bis_spacing: u32,
        irc: u8,
        max_pdu: u8,
        rfu: u8,
        seed_access_address: u32,
        sdu_interval: u32,
        max_sdu: u16,
        base_crc_init: u16,
        ch_m: u64,
        phy: u8,
        bis_payload_count: u64,
        framing: bool,
        giv: Option<[u8; 8]>,
        gskd: Option<[u8; 16]>,
    ) -> Self {
        Self {
            length: if let None = giv { 34 } else { 58 },
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis: num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        }
    }
}

impl TryFrom<&Vec<u8>> for BigInfo {
    type Error = String;
    /// Create [`BigInfo`] from `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{big_info::BigInfo, data_type::DataType};
    ///
    /// let length = 34;
    /// let big_offset: u16 = 1;
    /// let big_offset_units: bool = true;
    /// let iso_interval: u16 = 2;
    /// let num_bis: u8 = 3;
    /// let nse: u8 = 4;
    /// let bn: u8 = 5;
    /// let sub_interval: u32 = 6;
    /// let pto: u8 = 7;
    /// let bis_spacing: u32 = 8;
    /// let irc: u8 = 9;
    /// let max_pdu: u8 = 10;
    /// let rfu: u8 = 11;
    /// let seed_access_address: u32 = 12;
    /// let sdu_interval: u32 = 13;
    /// let max_sdu: u16 = 14;
    /// let base_crc_init: u16 = 15;
    /// let ch_m: u64 = 16;
    /// let phy: u8 = 17;
    /// let bis_payload_count: u64 = 18;
    /// let framing: bool = false;
    /// let giv: Option<[u8; 8]> = None;
    /// let gskd: Option<[u8; 16]> = None;
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BigInfo::data_type());
    /// data.push(big_offset as u8);
    /// let mut value: u8 = (big_offset >> 8) as u8;
    /// if big_offset_units {
    ///     value |= 0b01000000;
    /// }
    /// value |= (iso_interval << 7) as u8;
    /// data.push(value);
    ///
    /// data.push((iso_interval >> 1) as u8);
    ///
    /// value = (iso_interval >> 9) as u8;
    /// value |= num_bis << 3;
    /// data.push(value);
    ///
    /// value = nse;
    /// value |= bn << 5;
    /// data.push(value);
    ///
    /// data.push(sub_interval as u8);
    /// data.push((sub_interval >> 8) as u8);
    /// value = (sub_interval >> 16) as u8;
    /// value |= pto << 4;
    /// data.push(value);
    ///
    /// data.push(bis_spacing as u8);
    /// data.push((bis_spacing >> 8) as u8);
    /// value = (bis_spacing >> 16) as u8;
    /// value |= irc << 4;
    /// data.push(value);
    ///
    /// data.push(max_pdu);
    /// data.push(rfu);
    ///
    /// data.append(&mut seed_access_address.to_le_bytes().to_vec());
    ///
    /// data.push(sdu_interval as u8);
    /// data.push((sdu_interval >> 8) as u8);
    /// value = (sdu_interval >> 16) as u8;
    /// value |= (max_sdu << 4) as u8;
    /// data.push(value);
    /// data.push((max_sdu >> 4) as u8);
    ///
    /// data.append(&mut base_crc_init.to_le_bytes().to_vec());
    ///
    /// data.push(ch_m as u8);
    /// data.push((ch_m >> 8) as u8);
    /// data.push((ch_m >> 16) as u8);
    /// data.push((ch_m >> 24) as u8);
    /// value = (ch_m >> 32) as u8;
    /// value |= phy << 5;
    /// data.push(value);
    ///
    /// data.push(bis_payload_count as u8);
    /// data.push((bis_payload_count >> 8) as u8);
    /// data.push((bis_payload_count >> 16) as u8);
    /// data.push((bis_payload_count >> 24) as u8);
    /// value = (bis_payload_count >> 32) as u8;
    /// if framing {
    ///     value |= 0b10000000;
    /// }
    /// data.push(value);
    ///
    /// let result = BigInfo::try_from(&data);
    /// assert!(result.is_ok());
    /// let data_type = result.unwrap();
    /// assert_eq!(length, data_type.length);
    /// assert_eq!(big_offset, data_type.big_offset);
    /// assert_eq!(big_offset_units, data_type.big_offset_units);
    /// assert_eq!(iso_interval, data_type.iso_interval);
    /// assert_eq!(num_bis, data_type.num_bis);
    /// assert_eq!(nse, data_type.nse);
    /// assert_eq!(bn, data_type.bn);
    /// assert_eq!(sub_interval, data_type.sub_interval);
    /// assert_eq!(pto, data_type.pto);
    /// assert_eq!(bis_spacing, data_type.bis_spacing);
    /// assert_eq!(irc, data_type.irc);
    /// assert_eq!(max_pdu, data_type.max_pdu);
    /// assert_eq!(rfu, data_type.rfu);
    /// assert_eq!(seed_access_address, data_type.seed_access_address);
    /// assert_eq!(sdu_interval, data_type.sdu_interval);
    /// assert_eq!(max_sdu, data_type.max_sdu);
    /// assert_eq!(base_crc_init, data_type.base_crc_init);
    /// assert_eq!(ch_m, data_type.ch_m);
    /// assert_eq!(phy & 0b00000111, data_type.phy);
    /// assert_eq!(bis_payload_count, data_type.bis_payload_count);
    /// assert_eq!(framing, data_type.framing);
    /// assert_eq!(giv, data_type.giv);
    /// assert_eq!(gskd, data_type.gskd);
    ///
    /// let length = 58;
    /// let giv: Option<[u8; 8]> = Some([19, 0, 0, 0, 0, 0, 0, 0]);
    /// let gskd: Option<[u8; 16]> = Some([20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    ///
    /// data = Vec::new();
    /// data.push(0);
    /// data.push(length);
    /// data.push(BigInfo::data_type());
    /// data.push(big_offset as u8);
    /// let mut value: u8 = (big_offset >> 8) as u8;
    /// if big_offset_units {
    ///     value |= 0b01000000;
    /// }
    /// value |= (iso_interval << 7) as u8;
    /// data.push(value);
    ///
    /// data.push((iso_interval >> 1) as u8);
    ///
    /// value = (iso_interval >> 9) as u8;
    /// value |= num_bis << 3;
    /// data.push(value);
    ///
    /// value = nse;
    /// value |= bn << 5;
    /// data.push(value);
    ///
    /// data.push(sub_interval as u8);
    /// data.push((sub_interval >> 8) as u8);
    /// value = (sub_interval >> 16) as u8;
    /// value |= pto << 4;
    /// data.push(value);
    ///
    /// data.push(bis_spacing as u8);
    /// data.push((bis_spacing >> 8) as u8);
    /// value = (bis_spacing >> 16) as u8;
    /// value |= irc << 4;
    /// data.push(value);
    ///
    /// data.push(max_pdu);
    /// data.push(rfu);
    ///
    /// data.append(&mut seed_access_address.to_le_bytes().to_vec());
    ///
    /// data.push(sdu_interval as u8);
    /// data.push((sdu_interval >> 8) as u8);
    /// value = (sdu_interval >> 16) as u8;
    /// value |= (max_sdu << 4) as u8;
    /// data.push(value);
    /// data.push((max_sdu >> 4) as u8);
    ///
    /// data.append(&mut base_crc_init.to_le_bytes().to_vec());
    ///
    /// data.push(ch_m as u8);
    /// data.push((ch_m >> 8) as u8);
    /// data.push((ch_m >> 16) as u8);
    /// data.push((ch_m >> 24) as u8);
    /// value = (ch_m >> 32) as u8;
    /// value |= phy << 5;
    /// data.push(value);
    ///
    /// data.push(bis_payload_count as u8);
    /// data.push((bis_payload_count >> 8) as u8);
    /// data.push((bis_payload_count >> 16) as u8);
    /// data.push((bis_payload_count >> 24) as u8);
    /// value = (bis_payload_count >> 32) as u8;
    /// if framing {
    ///     value |= 0b10000000;
    /// }
    /// data.push(value);
    /// data.append(&mut giv.unwrap().clone().to_vec());
    /// data.append(&mut gskd.unwrap().clone().to_vec());
    ///
    /// let data: Vec<u8> = Vec::new();
    /// let result = BigInfo::try_from(&data);
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     format!("Invalid data size :{}", data.len()),
    ///     result.unwrap_err()
    /// );
    /// ```
    fn try_from(value: &Vec<u8>) -> Result<Self, String> {
        let len = value.len();
        if len < 34 {
            return Err(format!("Invalid data size :{}", len).to_string());
        }
        let length = value[0];
        let value1 = u16::from_le_bytes(value[2..4].try_into().unwrap());
        let big_offset = value1 & 0b00111111_11111111;
        let big_offset_units = value1 & 0b01000000_00000000 != 0;

        let value2 = u16::from_le_bytes(value[4..6].try_into().unwrap());
        let iso_interval =
            ((value1 & 0b1000000000000000) >> 15) | ((value2 & 0b0000011111111111) << 1);
        let num_bis: u8 = ((value2 & 0b1111100000000000) >> 11) as u8;

        let nse = value[6] & 0b00011111;

        let bn = (value[6] & 0b11100000) >> 5;

        let value1 = u32::from_le_bytes(value[7..11].try_into().unwrap());
        let sub_interval = value1 & 0b00000000_00001111_11111111_11111111;
        let pto = ((value1 & 0b00000000_11110000_00000000_00000000) >> 20) as u8;

        let value1 = u32::from_le_bytes(value[10..14].try_into().unwrap())
            & 0b00000000_11111111_11111111_11111111;
        let bis_spacing = value1 & 0b00001111_11111111_11111111;
        let irc = ((value1 & 0b11110000_00000000_00000000) >> 20) as u8;

        let max_pdu = value[13];

        let rfu = value[14];

        let seed_access_address = u32::from_le_bytes(value[15..19].try_into().unwrap());

        let value1 = u32::from_le_bytes(value[19..23].try_into().unwrap())
            & 0b00000000_11111111_11111111_11111111;
        let sdu_interval = value1 & 0b00001111_11111111_11111111;
        let max_sdu = (value1 >> 20) as u16 | (value[22] << 4) as u16;

        let base_crc_init = u16::from_le_bytes(value[23..25].try_into().unwrap());

        let value1 = u64::from_le_bytes(value[25..33].try_into().unwrap()) & 0x000000ffffffffff;
        let ch_m = value1 & 0b00011111_11111111_11111111_11111111_11111111;
        let phy = ((value1 & 0b11100000_00000000_00000000_00000000_00000000) >> 37) as u8;

        let mut tmp = value.clone();
        tmp.append(&mut [0u8; 3].to_vec());
        let value1 = u64::from_le_bytes(tmp[30..38].try_into().unwrap()) & 0x000000ffffffffff;
        let bis_payload_count = value1 & 0b01111111_11111111_11111111_11111111_11111111;
        let framing = value1 & 0b10000000_00000000_00000000_00000000_00000000 != 0;

        let (giv, gskd) = if length == 58 {
            (
                Some(value[35..43].try_into().unwrap()),
                Some(value[43..59].try_into().unwrap()),
            )
        } else {
            (None, None)
        };
        Ok(Self {
            length,
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        })
    }
}

impl Into<Vec<u8>> for BigInfo {
    /// Create `Vec<u8>` from [`BigInfo`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{big_info::BigInfo, data_type::DataType};
    ///
    /// let length = 34;
    /// let big_offset: u16 = 1;
    /// let big_offset_units: bool = true;
    /// let iso_interval: u16 = 2;
    /// let num_bis: u8 = 3;
    /// let nse: u8 = 4;
    /// let bn: u8 = 5;
    /// let sub_interval: u32 = 6;
    /// let pto: u8 = 7;
    /// let bis_spacing: u32 = 8;
    /// let irc: u8 = 9;
    /// let max_pdu: u8 = 10;
    /// let rfu: u8 = 11;
    /// let seed_access_address: u32 = 12;
    /// let sdu_interval: u32 = 13;
    /// let max_sdu: u16 = 14;
    /// let base_crc_init: u16 = 15;
    /// let ch_m: u64 = 16;
    /// let phy: u8 = 17;
    /// let bis_payload_count: u64 = 18;
    /// let framing: bool = false;
    /// let giv: Option<[u8; 8]> = None;
    /// let gskd: Option<[u8; 16]> = None;
    ///
    /// let mut data: Vec<u8> = Vec::new();
    /// data.push(length);
    /// data.push(BigInfo::data_type());
    /// data.push(big_offset as u8);
    /// let mut value: u8 = (big_offset >> 8) as u8;
    /// if big_offset_units {
    ///     value |= 0b01000000;
    /// }
    /// value |= (iso_interval << 7) as u8;
    /// data.push(value);
    ///
    /// data.push((iso_interval >> 1) as u8);
    ///
    /// value = (iso_interval >> 9) as u8;
    /// value |= num_bis << 3;
    /// data.push(value);
    ///
    /// value = nse;
    /// value |= bn << 5;
    /// data.push(value);
    ///
    /// data.push(sub_interval as u8);
    /// data.push((sub_interval >> 8) as u8);
    /// value = (sub_interval >> 16) as u8;
    /// value |= pto << 4;
    /// data.push(value);
    ///
    /// data.push(bis_spacing as u8);
    /// data.push((bis_spacing >> 8) as u8);
    /// value = (bis_spacing >> 16) as u8;
    /// value |= irc << 4;
    /// data.push(value);
    ///
    /// data.push(max_pdu);
    /// data.push(rfu);
    ///
    /// data.append(&mut seed_access_address.to_le_bytes().to_vec());
    ///
    /// data.push(sdu_interval as u8);
    /// data.push((sdu_interval >> 8) as u8);
    /// value = (sdu_interval >> 16) as u8;
    /// value |= (max_sdu << 4) as u8;
    /// data.push(value);
    /// data.push((max_sdu >> 4) as u8);
    ///
    /// data.append(&mut base_crc_init.to_le_bytes().to_vec());
    ///
    /// data.push(ch_m as u8);
    /// data.push((ch_m >> 8) as u8);
    /// data.push((ch_m >> 16) as u8);
    /// data.push((ch_m >> 24) as u8);
    /// value = (ch_m >> 32) as u8;
    /// value |= phy << 5;
    /// data.push(value);
    ///
    /// data.push(bis_payload_count as u8);
    /// data.push((bis_payload_count >> 8) as u8);
    /// data.push((bis_payload_count >> 16) as u8);
    /// data.push((bis_payload_count >> 24) as u8);
    /// value = (bis_payload_count >> 32) as u8;
    /// if framing {
    ///     value |= 0b10000000;
    /// }
    /// data.push(value);
    ///
    /// let result1 = BigInfo::new(
    ///     big_offset,
    ///     big_offset_units,
    ///     iso_interval,
    ///     num_bis,
    ///     nse,
    ///     bn,
    ///     sub_interval,
    ///     pto,
    ///     bis_spacing,
    ///     irc,
    ///     max_pdu,
    ///     rfu,
    ///     seed_access_address,
    ///     sdu_interval,
    ///     max_sdu,
    ///     base_crc_init,
    ///     ch_m,
    ///     phy,
    ///     bis_payload_count,
    ///     framing,
    ///     giv,
    ///     gskd,
    /// );
    ///
    /// let into_data: Vec<u8> = result1.into();
    /// assert_eq!(data, into_data);
    ///
    /// let result2 = BigInfo::try_from(&data);
    /// assert!(result2.is_ok());
    /// let data_type = result2.unwrap();
    /// let into_data: Vec<u8> = data_type.into();
    /// assert_eq!(data, into_data);
    /// ```
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.length);
        data.push(Self::data_type());
        data.push(self.big_offset as u8);
        let mut value: u8 = (self.big_offset >> 8) as u8;
        if self.big_offset_units {
            value |= 0b01000000;
        }
        value |= (self.iso_interval << 7) as u8;
        data.push(value);

        data.push((self.iso_interval >> 1) as u8);

        value = (self.iso_interval >> 9) as u8;
        value |= self.num_bis << 3;
        data.push(value);

        value = self.nse;
        value |= self.bn << 5;
        data.push(value);

        data.push(self.sub_interval as u8);
        data.push((self.sub_interval >> 8) as u8);
        value = (self.sub_interval >> 16) as u8;
        value |= self.pto << 4;
        data.push(value);

        data.push(self.bis_spacing as u8);
        data.push((self.bis_spacing >> 8) as u8);
        value = (self.bis_spacing >> 16) as u8;
        value |= self.irc << 4;
        data.push(value);

        data.push(self.max_pdu);
        data.push(self.rfu);

        data.append(&mut self.seed_access_address.to_le_bytes().to_vec());

        data.push(self.sdu_interval as u8);
        data.push((self.sdu_interval >> 8) as u8);
        value = (self.sdu_interval >> 16) as u8;
        value |= (self.max_sdu << 4) as u8;
        data.push(value);
        data.push((self.max_sdu >> 4) as u8);

        data.append(&mut self.base_crc_init.to_le_bytes().to_vec());

        data.push(self.ch_m as u8);
        data.push((self.ch_m >> 8) as u8);
        data.push((self.ch_m >> 16) as u8);
        data.push((self.ch_m >> 24) as u8);
        value = (self.ch_m >> 32) as u8;
        value |= self.phy << 5;
        data.push(value);

        data.push(self.bis_payload_count as u8);
        data.push((self.bis_payload_count >> 8) as u8);
        data.push((self.bis_payload_count >> 16) as u8);
        data.push((self.bis_payload_count >> 24) as u8);
        value = (self.bis_payload_count >> 32) as u8;
        if self.framing {
            value |= 0b10000000;
        }
        data.push(value);
        match self.giv {
            Some(x) => data.append(&mut x.clone().to_vec()),
            None => (),
        }
        match self.gskd {
            Some(x) => data.append(&mut x.clone().to_vec()),
            None => (),
        }
        return data;
    }
}

impl DataType for BigInfo {
    /// return `0x2c`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ble_data_struct::data_types::{big_info::BigInfo, data_type::DataType};
    ///
    /// assert_eq!(0x2c, BigInfo::data_type());
    /// ```
    fn data_type() -> u8 {
        0x2c
    }
}

/// check `BIGInfo` data type.
///
/// # Examples
///
/// ```
/// use ble_data_struct::data_types::big_info::*;
/// use ble_data_struct::data_types::data_type::DataType;
///
/// assert!(is_big_info(0x2c));
/// assert!(!is_big_info(0x00));
/// ```
pub fn is_big_info(data_type: u8) -> bool {
    BigInfo::data_type() == data_type
}

#[cfg(test)]
mod tests {
    use crate::data_types::{big_info::*, data_type::DataType};

    #[test]
    fn test_new() {
        let big_offset: u16 = 1;
        let big_offset_units: bool = true;
        let iso_interval: u16 = 2;
        let num_bis: u8 = 3;
        let nse: u8 = 4;
        let bn: u8 = 5;
        let sub_interval: u32 = 6;
        let pto: u8 = 7;
        let bis_spacing: u32 = 8;
        let irc: u8 = 9;
        let max_pdu: u8 = 10;
        let rfu: u8 = 11;
        let seed_access_address: u32 = 12;
        let sdu_interval: u32 = 13;
        let max_sdu: u16 = 14;
        let base_crc_init: u16 = 15;
        let ch_m: u64 = 16;
        let phy: u8 = 17;
        let bis_payload_count: u64 = 18;
        let framing: bool = false;
        let giv: Option<[u8; 8]> = None;
        let gskd: Option<[u8; 16]> = None;
        let result = BigInfo::new(
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        );
        assert_eq!(34, result.length);
        assert_eq!(big_offset, result.big_offset);
        assert_eq!(big_offset_units, result.big_offset_units);
        assert_eq!(iso_interval, result.iso_interval);
        assert_eq!(num_bis, result.num_bis);
        assert_eq!(nse, result.nse);
        assert_eq!(bn, result.bn);
        assert_eq!(sub_interval, result.sub_interval);
        assert_eq!(pto, result.pto);
        assert_eq!(bis_spacing, result.bis_spacing);
        assert_eq!(irc, result.irc);
        assert_eq!(max_pdu, result.max_pdu);
        assert_eq!(rfu, result.rfu);
        assert_eq!(seed_access_address, result.seed_access_address);
        assert_eq!(sdu_interval, result.sdu_interval);
        assert_eq!(max_sdu, result.max_sdu);
        assert_eq!(base_crc_init, result.base_crc_init);
        assert_eq!(ch_m, result.ch_m);
        assert_eq!(phy, result.phy);
        assert_eq!(bis_payload_count, result.bis_payload_count);
        assert_eq!(framing, result.framing);
        assert_eq!(giv, result.giv);
        assert_eq!(gskd, result.gskd);

        let giv: Option<[u8; 8]> = Some([19, 0, 0, 0, 0, 0, 0, 0]);
        let gskd: Option<[u8; 16]> = Some([20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let result = BigInfo::new(
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        );
        assert_eq!(58, result.length);
        assert_eq!(big_offset, result.big_offset);
        assert_eq!(big_offset_units, result.big_offset_units);
        assert_eq!(iso_interval, result.iso_interval);
        assert_eq!(num_bis, result.num_bis);
        assert_eq!(nse, result.nse);
        assert_eq!(bn, result.bn);
        assert_eq!(sub_interval, result.sub_interval);
        assert_eq!(pto, result.pto);
        assert_eq!(bis_spacing, result.bis_spacing);
        assert_eq!(irc, result.irc);
        assert_eq!(max_pdu, result.max_pdu);
        assert_eq!(rfu, result.rfu);
        assert_eq!(seed_access_address, result.seed_access_address);
        assert_eq!(sdu_interval, result.sdu_interval);
        assert_eq!(max_sdu, result.max_sdu);
        assert_eq!(base_crc_init, result.base_crc_init);
        assert_eq!(ch_m, result.ch_m);
        assert_eq!(phy, result.phy);
        assert_eq!(bis_payload_count, result.bis_payload_count);
        assert_eq!(framing, result.framing);
        assert_eq!(giv, result.giv);
        assert_eq!(gskd, result.gskd);
    }

    #[test]
    fn test_try_from() {
        let length = 34;
        let big_offset: u16 = 1;
        let big_offset_units: bool = true;
        let iso_interval: u16 = 2;
        let num_bis: u8 = 3;
        let nse: u8 = 4;
        let bn: u8 = 5;
        let sub_interval: u32 = 6;
        let pto: u8 = 7;
        let bis_spacing: u32 = 8;
        let irc: u8 = 9;
        let max_pdu: u8 = 10;
        let rfu: u8 = 11;
        let seed_access_address: u32 = 12;
        let sdu_interval: u32 = 13;
        let max_sdu: u16 = 14;
        let base_crc_init: u16 = 15;
        let ch_m: u64 = 16;
        let phy: u8 = 17;
        let bis_payload_count: u64 = 18;
        let framing: bool = false;
        let giv: Option<[u8; 8]> = None;
        let gskd: Option<[u8; 16]> = None;

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BigInfo::data_type());
        data.push(big_offset as u8);
        let mut value: u8 = (big_offset >> 8) as u8;
        if big_offset_units {
            value |= 0b01000000;
        }
        value |= (iso_interval << 7) as u8;
        data.push(value);

        data.push((iso_interval >> 1) as u8);

        value = (iso_interval >> 9) as u8;
        value |= num_bis << 3;
        data.push(value);

        value = nse;
        value |= bn << 5;
        data.push(value);

        data.push(sub_interval as u8);
        data.push((sub_interval >> 8) as u8);
        value = (sub_interval >> 16) as u8;
        value |= pto << 4;
        data.push(value);

        data.push(bis_spacing as u8);
        data.push((bis_spacing >> 8) as u8);
        value = (bis_spacing >> 16) as u8;
        value |= irc << 4;
        data.push(value);

        data.push(max_pdu);
        data.push(rfu);

        data.append(&mut seed_access_address.to_le_bytes().to_vec());

        data.push(sdu_interval as u8);
        data.push((sdu_interval >> 8) as u8);
        value = (sdu_interval >> 16) as u8;
        value |= (max_sdu << 4) as u8;
        data.push(value);
        data.push((max_sdu >> 4) as u8);

        data.append(&mut base_crc_init.to_le_bytes().to_vec());

        data.push(ch_m as u8);
        data.push((ch_m >> 8) as u8);
        data.push((ch_m >> 16) as u8);
        data.push((ch_m >> 24) as u8);
        value = (ch_m >> 32) as u8;
        value |= phy << 5;
        data.push(value);

        data.push(bis_payload_count as u8);
        data.push((bis_payload_count >> 8) as u8);
        data.push((bis_payload_count >> 16) as u8);
        data.push((bis_payload_count >> 24) as u8);
        value = (bis_payload_count >> 32) as u8;
        if framing {
            value |= 0b10000000;
        }
        data.push(value);

        let result = BigInfo::try_from(&data);
        assert!(result.is_ok());
        let data_type = result.unwrap();
        assert_eq!(length, data_type.length);
        assert_eq!(big_offset, data_type.big_offset);
        assert_eq!(big_offset_units, data_type.big_offset_units);
        assert_eq!(iso_interval, data_type.iso_interval);
        assert_eq!(num_bis, data_type.num_bis);
        assert_eq!(nse, data_type.nse);
        assert_eq!(bn, data_type.bn);
        assert_eq!(sub_interval, data_type.sub_interval);
        assert_eq!(pto, data_type.pto);
        assert_eq!(bis_spacing, data_type.bis_spacing);
        assert_eq!(irc, data_type.irc);
        assert_eq!(max_pdu, data_type.max_pdu);
        assert_eq!(rfu, data_type.rfu);
        assert_eq!(seed_access_address, data_type.seed_access_address);
        assert_eq!(sdu_interval, data_type.sdu_interval);
        assert_eq!(max_sdu, data_type.max_sdu);
        assert_eq!(base_crc_init, data_type.base_crc_init);
        assert_eq!(ch_m, data_type.ch_m);
        assert_eq!(phy & 0b00000111, data_type.phy);
        assert_eq!(bis_payload_count, data_type.bis_payload_count);
        assert_eq!(framing, data_type.framing);
        assert_eq!(giv, data_type.giv);
        assert_eq!(gskd, data_type.gskd);

        let mut data: Vec<u8> = vec![0u8; 33];
        data[0] = data.len() as u8 - 1;
        let result = BigInfo::try_from(&data);
        assert!(result.is_err());
        assert_eq!(
            format!("Invalid data size :{}", data.len()),
            result.unwrap_err()
        );
    }

    #[test]
    fn test_into() {
        let length = 34;
        let big_offset: u16 = 1;
        let big_offset_units: bool = true;
        let iso_interval: u16 = 2;
        let num_bis: u8 = 3;
        let nse: u8 = 4;
        let bn: u8 = 5;
        let sub_interval: u32 = 6;
        let pto: u8 = 7;
        let bis_spacing: u32 = 8;
        let irc: u8 = 9;
        let max_pdu: u8 = 10;
        let rfu: u8 = 11;
        let seed_access_address: u32 = 12;
        let sdu_interval: u32 = 13;
        let max_sdu: u16 = 14;
        let base_crc_init: u16 = 15;
        let ch_m: u64 = 16;
        let phy: u8 = 17;
        let bis_payload_count: u64 = 18;
        let framing: bool = false;
        let giv: Option<[u8; 8]> = None;
        let gskd: Option<[u8; 16]> = None;

        let mut data: Vec<u8> = Vec::new();
        data.push(length);
        data.push(BigInfo::data_type());
        data.push(big_offset as u8);
        let mut value: u8 = (big_offset >> 8) as u8;
        if big_offset_units {
            value |= 0b01000000;
        }
        value |= (iso_interval << 7) as u8;
        data.push(value);

        data.push((iso_interval >> 1) as u8);

        value = (iso_interval >> 9) as u8;
        value |= num_bis << 3;
        data.push(value);

        value = nse;
        value |= bn << 5;
        data.push(value);

        data.push(sub_interval as u8);
        data.push((sub_interval >> 8) as u8);
        value = (sub_interval >> 16) as u8;
        value |= pto << 4;
        data.push(value);

        data.push(bis_spacing as u8);
        data.push((bis_spacing >> 8) as u8);
        value = (bis_spacing >> 16) as u8;
        value |= irc << 4;
        data.push(value);

        data.push(max_pdu);
        data.push(rfu);

        data.append(&mut seed_access_address.to_le_bytes().to_vec());

        data.push(sdu_interval as u8);
        data.push((sdu_interval >> 8) as u8);
        value = (sdu_interval >> 16) as u8;
        value |= (max_sdu << 4) as u8;
        data.push(value);
        data.push((max_sdu >> 4) as u8);

        data.append(&mut base_crc_init.to_le_bytes().to_vec());

        data.push(ch_m as u8);
        data.push((ch_m >> 8) as u8);
        data.push((ch_m >> 16) as u8);
        data.push((ch_m >> 24) as u8);
        value = (ch_m >> 32) as u8;
        value |= phy << 5;
        data.push(value);

        data.push(bis_payload_count as u8);
        data.push((bis_payload_count >> 8) as u8);
        data.push((bis_payload_count >> 16) as u8);
        data.push((bis_payload_count >> 24) as u8);
        value = (bis_payload_count >> 32) as u8;
        if framing {
            value |= 0b10000000;
        }
        data.push(value);

        let result1 = BigInfo::new(
            big_offset,
            big_offset_units,
            iso_interval,
            num_bis,
            nse,
            bn,
            sub_interval,
            pto,
            bis_spacing,
            irc,
            max_pdu,
            rfu,
            seed_access_address,
            sdu_interval,
            max_sdu,
            base_crc_init,
            ch_m,
            phy,
            bis_payload_count,
            framing,
            giv,
            gskd,
        );

        let into_data: Vec<u8> = result1.into();
        assert_eq!(data, into_data);

        let result2 = BigInfo::try_from(&data);
        assert!(result2.is_ok());
        let data_type = result2.unwrap();
        let into_data: Vec<u8> = data_type.into();
        assert_eq!(data, into_data);
    }

    #[test]
    fn test_data_type() {
        assert_eq!(0x2c, BigInfo::data_type());
    }

    #[test]
    fn test_is_big_info() {
        assert!(is_big_info(0x2c));
        assert!(!is_big_info(0x00));
    }
}

extern crate image_base64_wasm;
extern crate qrcode_generator;
extern crate unidecode;

pub mod pix {
    use qrcode_generator::QrCodeEcc;
    #[derive(Default)]
    struct PixStatic {
        name: Option<String>,
        city: Option<String>,
        zip_code: Option<String>,
        identificator: Option<String>,
        description: Option<String>,
        amount: Option<f64>,
        key: Option<String>,
        is_unique_transaction: bool,
    }

    use std::ops::{Bound, RangeBounds};

    trait StringUtils {
        fn substring(&self, start: usize, len: usize) -> &str;
        fn slice(&self, range: impl RangeBounds<usize>) -> &str;
    }

    impl StringUtils for str {
        fn substring(&self, start: usize, len: usize) -> &str {
            let mut char_pos = 0;
            let mut byte_start = 0;
            let mut it = self.chars();
            loop {
                if char_pos == start {
                    break;
                }
                if let Some(c) = it.next() {
                    char_pos += 1;
                    byte_start += c.len_utf8();
                } else {
                    break;
                }
            }
            char_pos = 0;
            let mut byte_end = byte_start;
            loop {
                if char_pos == len {
                    break;
                }
                if let Some(c) = it.next() {
                    char_pos += 1;
                    byte_end += c.len_utf8();
                } else {
                    break;
                }
            }
            &self[byte_start..byte_end]
        }
        fn slice(&self, range: impl RangeBounds<usize>) -> &str {
            let start = match range.start_bound() {
                Bound::Included(bound) | Bound::Excluded(bound) => *bound,
                Bound::Unbounded => 0,
            };
            let len = match range.end_bound() {
                Bound::Included(bound) => *bound + 1,
                Bound::Excluded(bound) => *bound,
                Bound::Unbounded => self.len(),
            } - start;
            self.substring(start, len)
        }
    }

    trait Pix {
        fn generate_account_information(&self) -> String;
        fn get_brcode(&self) -> String;
        // fn get_qrcode(&self) -> String;

        fn set_name(&mut self, name: String);
        fn set_city(&mut self, city: String);
        fn set_zip_code(&mut self, zip_code: String);
        fn set_identificator(&mut self, identificator: String);
        fn set_description(&mut self, description: String);
        fn set_amount(&mut self, amount: f64);
        fn set_key(&mut self, key: String);
        fn set_is_unique_transaction(&mut self, is_unique_transaction: bool);
    }

    impl Pix for PixStatic {
        //Merchant Account Information
        fn generate_account_information(&self) -> String {
            let mut payload = String::from("");
            payload.push_str(&get_emv("00".to_string(), "br.gov.bcb.pix".to_string()));

            match &self.key {
                None => (),
                Some(e) => payload.push_str(&get_emv("01".to_string(), e.to_string())),
            }

            match &self.description {
                None => (),
                Some(e) => payload.push_str(&get_emv("02".to_string(), e.to_string())),
            }

            payload
        }

        fn get_brcode(&self) -> String {
            let mut lines: Vec<String> = Vec::new();
            // Payload Format Indicator
            lines.push(get_emv("00".to_string(), "01".to_string()));

            // Is Unique Transaction?
            lines.push(get_emv(
                "01".to_string(),
                if self.is_unique_transaction {
                    "12".to_string()
                } else {
                    "11".to_string()
                },
            )); //010212

            lines.push(get_emv(
                "26".to_string(),
                self.generate_account_information(),
            ));
            // Merchant Category Code
            lines.push(get_emv("52".to_string(), "0000".to_string()));

            // Transaction Currency | 986 – BRL: real brasileiro - ISO4217
            lines.push(get_emv("53".to_string(), "986".to_string()));
            //Transaction Amount
            match &self.amount {
                None => (),
                Some(e) => lines.push(get_emv("54".to_string(), format!("{:.2}", e))),
            }
            // Country Code
            lines.push(get_emv("58".to_string(), "BR".to_string()));

            // Merchant Name
            match &self.name {
                None => (),
                Some(e) => lines.push(get_emv("59".to_string(), e.to_string())),
            }

            // Merchant City
            match &self.city {
                None => (),
                Some(e) => lines.push(get_emv("60".to_string(), e.to_string())),
            }

            // Postal Code
            match &self.zip_code {
                None => (),
                Some(e) => lines.push(get_emv("61".to_string(), e.to_string())),
            }

            lines.push(additional_data_field());

            lines.push("6304".to_string());

            lines.push(crc16_ccitt(&lines.join("")));

            format!("{}", lines.join(""))
        }

        fn set_name(&mut self, name: String) {
            if name.len() > 25 {
                panic!(
                    "The maximum number of characters for the recipient's name is 25 .  name: {}. len {}",
                    name, name.len()
                );
            }

            self.name = Some(name);
        }
        fn set_city(&mut self, city: String) {
            if city.len() > 15 {
                panic!(
                    "The maximum number of characters for the city is 15.  city: {}.",
                    city
                );
            }
            self.city = Some(city);
        }
        fn set_zip_code(&mut self, zip_code: String) {
            if zip_code.len() != 8 {
                panic!(
                    "The number of characters for the zip code is 8.  zip_code: {}.",
                    zip_code
                );
            }

            self.zip_code = Some(zip_code);
        }
        fn set_identificator(&mut self, identificator: String) {
            self.identificator = Some(identificator);
        }
        fn set_description(&mut self, description: String) {
            if description.len() > 50 {
                panic!(
                    "The maximum number of characters for the description is 50.  description: {}.",
                    description
                );
            }

            self.description = Some(description);
        }
        fn set_amount(&mut self, amount: f64) {
            self.amount = Some(amount);
        }
        fn set_key(&mut self, key: String) {
            if key.len() < 10 {
                panic!("The key field is invalid.  Key: {}.", key);
            }
            self.key = Some(key);
        }
        fn set_is_unique_transaction(&mut self, is_unique_transaction: bool) {
            self.is_unique_transaction = is_unique_transaction;
        }
    }

    fn get_emv(id: String, value: String) -> String {
        let len: String = format!("{:0>2}", value.len().to_string()); // corrigir
        let val = format!("{}{}{}", id, len, value);
        val
    }

    fn additional_data_field() -> String {
        get_emv(
            "62".to_string(),
            get_emv("05".to_string(), "***".to_string()),
        )
    }

    fn crc16_ccitt(message: &str) -> String {
        let mut crc: u16 = 0xFFFF; // initial value
        let polynomial: u16 = 0x1021; // 0001 0000 0010 0001  (0, 5, 12)
        let bytes = message.as_bytes();
        for b in bytes {
            for i in 0u16..8u16 {
                let bit = (b >> (7 - i) & 1) == 1;
                let c15 = (crc >> 15 & 1) == 1;
                crc <<= 1;
                if c15 ^ bit {
                    crc ^= polynomial
                };
            }
        }
        crc &= 0xffff;
        format!("{:X}", crc).prepend_remaining_length(4, '0')
    }

    trait Field {
        fn prepend_remaining_length(&self, length: usize, character: char) -> String;
    }
    impl Field for String {
        fn prepend_remaining_length(&self, length: usize, character: char) -> String {
            let mut string = self.to_owned();
            let limit = length - string.len();
            for _i in 0..limit {
                string.insert(0, character);
            }
            string
        }
    }

    pub fn brcode(
        key: &str,
        name: &str,
        amount: Option<f64>,
        city: Option<&str>,
        description: Option<&str>,
        zip_code: Option<&str>,
        is_unique_transaction: Option<bool>,
    ) -> String {
        use unidecode::unidecode;

        let mut brcode: PixStatic = Default::default();
        brcode.set_key(unidecode(key.trim()));
        brcode.set_name(unidecode(&name.trim().substring(0, 25)));

        match city {
            None => brcode.set_city("Sao Paulo".to_string()),
            Some(e) => brcode.set_city(unidecode(&e.trim().substring(0, 15))),
        }
        match amount {
            None => (),
            Some(e) => brcode.set_amount(e),
        }
        match description {
            None => (),
            Some(e) => brcode.set_description(unidecode(&e.trim().substring(0, 50))),
        }
        match zip_code {
            None => (),
            Some(e) => brcode.set_zip_code(unidecode(&e.trim().substring(0, 8))),
        }

        match is_unique_transaction {
            None => brcode.set_is_unique_transaction(false),
            Some(e) => brcode.set_is_unique_transaction(e),
        }
        brcode.set_identificator("52".to_string());
        brcode.get_brcode()
    }

    pub fn save_qrcode_png(payload: String, path: &str) {
        qrcode_generator::to_png_to_file(payload, QrCodeEcc::Low, 1024, path).unwrap();
    }

    pub fn get_qrcode_data_uri(payload: String) -> String {
        let img_data: Vec<u8> =
            qrcode_generator::to_png_to_vec(payload, QrCodeEcc::Low, 1024).unwrap();
        let base64 = image_base64_wasm::vec_to_base64(img_data);
        base64
    }

    pub fn save_qrcode_svg(payload: String, path: &str) {
        qrcode_generator::to_svg_to_file(payload, QrCodeEcc::Low, 1024, None::<&str>, path)
            .unwrap();
    }
    pub fn get_qrcode_svg(payload: String) -> String {
        let result: String =
            qrcode_generator::to_svg_to_string(payload, QrCodeEcc::Low, 1024, None::<&str>)
                .unwrap();
        result
    }
}

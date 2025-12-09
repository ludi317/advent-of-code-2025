advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use super::*;

    // Define OCRA suite enum - exhaustive, no "default" variant
    #[derive(Debug, Clone, Copy)]
    enum OcraSuite {
        V1HotpSha256T0,
        V1HotpSha256T0TpmBug,
        V1HotpSha256T0Tpm,
        V1HotpSha384T0Tpm,
    }

    // Define OCRA data struct
    struct OcraData {
        suite: OcraSuite,
        data: Vec<u8>,
    }

    impl OcraData {
        fn data_input(&self) -> &[u8] {
            &self.data
        }
    }

    // The main function - returns Vec<u8>, no error possible for unsupported suite
    fn ocra_v1_hotp_t0(k: &[u8], in_data: &OcraData) -> Vec<u8> {
        // Compute HMAC based on the suite - all cases handled, no error
        match in_data.suite {
            OcraSuite::V1HotpSha256T0
            | OcraSuite::V1HotpSha256T0TpmBug
            | OcraSuite::V1HotpSha256T0Tpm => {
                // HMAC::new_from_slice can fail if key length is invalid
                // but that's a programming error, so we'll panic for simplicity
                let mut hmac = Hmac::<Sha256>::new_from_slice(k)
                    .expect("Invalid key length for HMAC-SHA256");
                hmac.update(in_data.data_input());
                hmac.finalize().into_bytes().to_vec()
            }
            OcraSuite::V1HotpSha384T0Tpm => {
                let mut hmac = Hmac::<Sha384>::new_from_slice(k)
                    .expect("Invalid key length for HMAC-SHA384");
                hmac.update(in_data.data_input());
                hmac.finalize().into_bytes().to_vec()
            }
        }
    }

    #[test]
    fn test_part_one() {
        describe_color(Red);

    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

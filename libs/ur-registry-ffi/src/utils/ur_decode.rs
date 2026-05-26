use crate::export;
use hex;
use serde_json::json;
use ur::ur::Kind;

fn sanitize_part(part: &str) -> String {
    part.trim()
        .trim_matches(|c| c == '"' || c == '\'' || c == '[' || c == ']')
        .to_string()
}

fn split_ur_parts(ur_input: &str) -> Vec<String> {
    let trimmed = ur_input.trim();
    if trimmed.is_empty() {
        return vec![];
    }

    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        if let Ok(parts) = serde_json::from_str::<Vec<String>>(trimmed) {
            let parts = parts
                .into_iter()
                .map(|part| sanitize_part(&part))
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();
            if !parts.is_empty() {
                return parts;
            }
        }
    }

    let parts = trimmed
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '|')
        .map(sanitize_part)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    if parts.len() > 1 {
        let ur_parts = parts
            .into_iter()
            .filter(|part| part.to_ascii_lowercase().starts_with("ur:"))
            .collect::<Vec<_>>();
        if !ur_parts.is_empty() {
            return ur_parts;
        }
    }

    vec![sanitize_part(trimmed)]
}

fn get_ur_type(part: &str) -> Result<String, String> {
    let lower = part.to_ascii_lowercase();
    let stripped = lower
        .strip_prefix("ur:")
        .ok_or_else(|| "ur is invalid".to_string())?;
    let (ur_type, _) = stripped
        .split_once('/')
        .ok_or_else(|| "ur type is invalid".to_string())?;
    Ok(ur_type.to_string())
}

fn decode_ur_parts(parts: Vec<String>) -> Result<(String, Vec<u8>), String> {
    if parts.is_empty() {
        return Err("ur is empty".to_string());
    }

    let normalized_parts = parts
        .into_iter()
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if normalized_parts.is_empty() {
        return Err("ur is empty".to_string());
    }

    let ur_type = get_ur_type(&normalized_parts[0])?;
    for part in normalized_parts.iter() {
        if !part.starts_with("ur:") {
            return Err("ur is invalid".to_string());
        }
        let current_type = get_ur_type(part)?;
        if current_type != ur_type {
            return Err("ur types are not the same across parts".to_string());
        }
    }

    if normalized_parts.len() == 1 {
        let only_part = normalized_parts[0].clone();
        if let Ok((kind, cbor)) = ur::decode(&only_part) {
            if kind == Kind::SinglePart {
                return Ok((ur_type, cbor));
            }
        }
    }

    let mut decoder = ur::Decoder::default();
    for part in normalized_parts.iter() {
        decoder
            .receive(part)
            .map_err(|e| format!("ur decode failed: {}", e))?;
    }

    if !decoder.complete() {
        return Err(format!(
            "multi-part ur is incomplete, progress: {}%",
            decoder.progress()
        ));
    }

    let cbor = decoder
        .message()
        .map_err(|e| format!("ur decode failed: {}", e))?
        .ok_or_else(|| "decoded cbor is empty".to_string())?;

    Ok((ur_type, cbor))
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_decodeUrToCborHex
    fn decode_ur_to_cbor_hex(ur_input: &str) -> String {
        let parts = split_ur_parts(ur_input);
        match decode_ur_parts(parts) {
            Ok((ur_type, cbor)) => json!({
                "type": ur_type,
                "cbor": hex::encode(cbor),
            }).to_string(),
            Err(err) => json!({
                "error": err,
            }).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::crypto_multi_accounts::parse_crypto_multi_accounts;
    use serde_json::Value;

    const CRYPTO_MULTI_ACCOUNTS_TYPE: &str = "crypto-multi-accounts";
    const CRYPTO_MULTI_ACCOUNTS_CBOR_HEX: &str = "a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30";
    const CRYPTO_MULTI_ACCOUNTS_CBOR_HEX_1_1: &str = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65";
    const CRYPTO_MULTI_ACCOUNTS_UR_1_1: &str = "ur:crypto-multi-accounts/1-1/lpadadcsgtcyeokkkgkthdgtotadcywlcscewfaolytaaddloeaxhdclaowdverokopdinhseeroisyalksaykctjshedprnuyjyfgrovawewftyghceglrpkgamtaaddyoyadlocsdwykcfadykykaeykaeykaxisjeihkkjkjyjljtihutltlrvo";
    const MULTI_PART_POOL_RAW: &str =
        include_str!("../../../../demo/fixtures/crypto_multi_accounts_parts_2_23.txt");

    fn decode_and_parse_ok(parts: &[&str]) -> Result<(), String> {
        let input = serde_json::to_string(parts).expect("must serialize");
        let decoded = decode_ur_to_cbor_hex(&input);
        let decoded_json: Value = match serde_json::from_str(&decoded) {
            Ok(v) => v,
            Err(_) => return Err(format!("decode json invalid: {}", decoded)),
        };
        let ur_type = match decoded_json.get("type").and_then(Value::as_str) {
            Some(v) => v,
            None => {
                return Err(format!(
                    "decode failed: {}",
                    decoded_json
                        .get("error")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown error")
                ))
            }
        };
        let cbor_hex = match decoded_json.get("cbor").and_then(Value::as_str) {
            Some(v) => v,
            None => return Err("decode returned empty cbor".to_string()),
        };
        let parsed = parse_crypto_multi_accounts(ur_type, cbor_hex);
        let parsed_json: Value = match serde_json::from_str(&parsed) {
            Ok(v) => v,
            Err(_) => return Err(format!("parse json invalid: {}", parsed)),
        };
        if parsed_json.get("error").is_some() {
            return Err(format!(
                "parse failed: {}",
                parsed_json
                    .get("error")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown error")
            ));
        }
        Ok(())
    }

    fn assert_decoded_ok(result: &str) {
        let json: Value = serde_json::from_str(result).expect("must be valid json");
        assert_eq!(
            json["type"].as_str().expect("type must exist"),
            CRYPTO_MULTI_ACCOUNTS_TYPE
        );
        assert_eq!(
            json["cbor"].as_str().expect("cbor must exist"),
            CRYPTO_MULTI_ACCOUNTS_CBOR_HEX
        );
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_single_part() {
        let cbor = hex::decode(CRYPTO_MULTI_ACCOUNTS_CBOR_HEX).unwrap();
        let ur = ur::encode(&cbor, CRYPTO_MULTI_ACCOUNTS_TYPE.to_string());

        let result = decode_ur_to_cbor_hex(&ur);
        assert_decoded_ok(&result);
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_one_of_one_multi_part() {
        let result = decode_ur_to_cbor_hex(CRYPTO_MULTI_ACCOUNTS_UR_1_1);
        let json: Value = serde_json::from_str(&result).expect("must be valid json");
        assert_eq!(
            json["type"].as_str().expect("type must exist"),
            CRYPTO_MULTI_ACCOUNTS_TYPE
        );
        assert_eq!(
            json["cbor"].as_str().expect("cbor must exist"),
            CRYPTO_MULTI_ACCOUNTS_CBOR_HEX_1_1
        );
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_multi_part_joined_by_newline() {
        let cbor = hex::decode(CRYPTO_MULTI_ACCOUNTS_CBOR_HEX).unwrap();
        let mut encoder = ur::Encoder::new(&cbor, 40, CRYPTO_MULTI_ACCOUNTS_TYPE.to_string())
            .expect("must build encoder");
        assert!(encoder.fragment_count() > 1);

        let mut parts = vec![];
        for _ in 0..encoder.fragment_count() {
            parts.push(encoder.next_part().expect("must have next part"));
        }

        let ur_input = parts.join("\n");
        let result = decode_ur_to_cbor_hex(&ur_input);
        assert_decoded_ok(&result);
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_multi_part_json_array() {
        let cbor = hex::decode(CRYPTO_MULTI_ACCOUNTS_CBOR_HEX).unwrap();
        let mut encoder = ur::Encoder::new(&cbor, 40, CRYPTO_MULTI_ACCOUNTS_TYPE.to_string())
            .expect("must build encoder");
        assert!(encoder.fragment_count() > 1);

        let mut parts = vec![];
        for _ in 0..encoder.fragment_count() {
            parts.push(encoder.next_part().expect("must have next part"));
        }

        let ur_input = serde_json::to_string(&parts).unwrap();
        let result = decode_ur_to_cbor_hex(&ur_input);
        assert_decoded_ok(&result);
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_incomplete_multi_part() {
        let cbor = hex::decode(CRYPTO_MULTI_ACCOUNTS_CBOR_HEX).unwrap();
        let mut encoder = ur::Encoder::new(&cbor, 40, CRYPTO_MULTI_ACCOUNTS_TYPE.to_string())
            .expect("must build encoder");
        assert!(encoder.fragment_count() > 1);
        let first_part = encoder.next_part().expect("must have first part");

        let result = decode_ur_to_cbor_hex(&first_part);
        let json: Value = serde_json::from_str(&result).expect("must be valid json");
        let error = json["error"].as_str().expect("error must exist");
        assert!(error.contains("incomplete"));
    }

    #[test]
    fn test_decode_ur_to_cbor_hex_multi_part_log_samples_batch() {
        let parts: Vec<&str> = MULTI_PART_POOL_RAW
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        assert!(parts.len() >= 20, "sample pool must have at least 20 parts");

        let mut groups: Vec<Vec<&str>> = vec![];
        for window in 12..=16 {
            if window > parts.len() {
                continue;
            }
            for start in 0..=(parts.len() - window) {
                groups.push(parts[start..start + window].to_vec());
            }
        }
        groups.push(parts.iter().step_by(2).copied().collect());
        groups.push(parts.iter().skip(1).step_by(2).copied().collect());

        let mut ok_count = 0usize;
        let mut failed = vec![];
        for (i, group) in groups.iter().enumerate() {
            match decode_and_parse_ok(group) {
                Ok(()) => ok_count += 1,
                Err(reason) => failed.push(format!("group {}: {}", i + 1, reason)),
            }
        }
        assert!(
            ok_count >= 8,
            "expected at least 8 groups to decode+parse, got {}. failed={:?}",
            ok_count,
            failed
        );
    }
}

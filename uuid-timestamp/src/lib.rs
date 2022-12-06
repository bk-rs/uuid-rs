use chrono::NaiveDateTime;

//
pub fn timestamp(bytes: &[u8; 16]) -> Option<NaiveDateTime> {
    // https://github.com/uuid-rs/uuid/blob/1.2.2/src/lib.rs#L521
    let version_num = bytes[6] >> 4;

    //
    match version_num {
        // https://github.com/uuid-rs/uuid/blob/1.2.2/src/lib.rs#L930
        7 => {
            let millis = u64::from_be_bytes([
                0, 0, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
            ]);

            Some(
                NaiveDateTime::from_timestamp_millis(millis as i64)
                    .expect("invalid or out-of-range datetime"),
            )
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    #[test]
    fn test_for_crate_uuid7() {
        //
        let v = "0184e596-ec2c-77a8-b4a9-9be4d1dcdb47"
            .parse::<uuid7::Uuid>()
            .unwrap();
        assert!(timestamp(v.as_bytes()).is_some());

        //
        let dt_start = Utc::now().naive_utc();
        let dt = timestamp(uuid7::uuid7().as_bytes()).unwrap();
        let dt_end = Utc::now().naive_utc();
        assert!(dt_start.timestamp_millis() <= dt.timestamp_millis());
        assert!(dt.timestamp_millis() <= dt_end.timestamp_millis());
    }

    #[cfg(uuid_unstable)]
    #[test]
    fn test_for_crate_uuid_v7() {
        //
        let v = "0184e596-ec2c-77a8-b4a9-9be4d1dcdb47"
            .parse::<uuid::Uuid>()
            .unwrap();
        assert!(timestamp(v.as_bytes()).is_some());

        //
        let dt_start = Utc::now().naive_utc();
        let dt = timestamp(uuid::Uuid::now_v7().as_bytes()).unwrap();
        let dt_end = Utc::now().naive_utc();
        assert!(dt_start.timestamp_millis() <= dt.timestamp_millis());
        assert!(dt.timestamp_millis() <= dt_end.timestamp_millis());
    }
}

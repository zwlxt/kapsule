pub mod encoding {
    pub trait DecoderExt {
        fn guess_and_decode(&self) -> String;
    }

    impl DecoderExt for [u8] {
        fn guess_and_decode(&self) -> String {
            let enc = guess_encoding(self, get_sys_encoding());
            let (s, _, _) = enc.decode(self);
            
            s.to_string()
        }
    }

    use std::env;

    use encoding_rs::Encoding;
    use tracing::warn;

    pub fn guess_encoding(
        content: &[u8],
        fallback: Option<&'static Encoding>,
    ) -> &'static Encoding {
        let mut det = chardetng::EncodingDetector::new();
        det.feed(content, true);

        let (mut enc, confident) = det.guess_assess(None, true);

        if !confident {
            warn!("not confident guessing, fallback");

            if let Some(fallback) = fallback {
                enc = fallback
            }
        }

        enc
    }

    pub fn get_sys_encoding() -> Option<&'static Encoding> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(lang) = env::var("LANG") {
                if let Some(enc_label) = lang.split('.').last() {
                    return Encoding::for_label(enc_label.as_bytes());
                }
            }
        }
        #[cfg(target_os = "windows")]
        {
            // unimplemented
            return None;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::util::encoding::get_sys_encoding;

    #[test]
    fn test_sys_encoding() {
        let enc = get_sys_encoding();
        println!("{:?}", &enc);
        assert!(enc.is_some());
    }
}

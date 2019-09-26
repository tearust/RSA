/// A generic trait that exposes the information that is needed for a hash function to be
/// used in `sign` and `verify.`.
pub trait Hash {
    /// Returns the length in bytes of a digest.
    fn size(&self) -> usize;

    /// Returns the ASN1 DER prefix for the the hash function.
    fn asn1_prefix(&self) -> &'static [u8];
}

/// A list of provided hashes, implementing `Hash`.
#[derive(Debug, Clone, Copy)]
pub enum Hashes {
    MD5,
    SHA1,
    SHA2_224,
    SHA2_256,
    SHA2_384,
    SHA2_512,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    MD5SHA1,
    RIPEMD160,
}

impl Hash for Hashes {
    fn size(&self) -> usize {
        match *self {
            Hashes::MD5 => 16,
            Hashes::SHA1 => 20,
            Hashes::SHA2_224 => 28,
            Hashes::SHA2_256 => 32,
            Hashes::SHA2_384 => 48,
            Hashes::SHA2_512 => 64,
            Hashes::SHA3_256 => 32,
            Hashes::SHA3_384 => 48,
            Hashes::SHA3_512 => 64,
            Hashes::MD5SHA1 => 36,
            Hashes::RIPEMD160 => 20,
        }
    }

    fn asn1_prefix(&self) -> &'static [u8] {
        match *self {
            Hashes::MD5 => &[
                0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x05,
                0x05, 0x00, 0x04, 0x10,
            ],
            Hashes::SHA1 => &[
                0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04,
                0x14,
            ],
            Hashes::SHA2_224 => &[
                0x30, 0x2d, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x04, 0x05, 0x00, 0x04, 0x1c,
            ],
            Hashes::SHA2_256 => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x01, 0x05, 0x00, 0x04, 0x20,
            ],
            Hashes::SHA2_384 => &[
                0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x02, 0x05, 0x00, 0x04, 0x30,
            ],

            Hashes::SHA2_512 => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x03, 0x05, 0x00, 0x04, 0x40,
            ],

            // A special TLS case which doesn't use an ASN1 prefix
            Hashes::MD5SHA1 => &[],
            Hashes::RIPEMD160 => &[
                0x30, 0x20, 0x30, 0x08, 0x06, 0x06, 0x28, 0xcf, 0x06, 0x03, 0x00, 0x31, 0x04, 0x14,
            ],

            Hashes::SHA3_256 => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x08, 0x05, 0x00, 0x04, 0x20,
            ],
            Hashes::SHA3_384 => &[
                30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x08, 0x05, 0x00, 0x04, 0x20,
            ],

            Hashes::SHA3_512 => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
                0x0a, 0x05, 0x00, 0x04, 0x40,
            ],
        }
    }
}

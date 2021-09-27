use super::super::public;

/// RSA key pair components.
#[derive(Clone, Copy)]
pub struct Components<Public, Private = Public> {
    /// The public key components.
    pub public_key: super::super::public::Components<Public>,

    /// The private exponent.
    pub d: Private,

    /// The first prime factor of `d`.
    pub p: Private,

    /// The second prime factor of `d`.
    pub q: Private,

    /// `p`'s public Chinese Remainder Theorem exponent.
    pub dP: Private,

    /// `q`'s public Chinese Remainder Theorem exponent.
    pub dQ: Private,

    /// `q**-1 mod p`.
    pub qInv: Private,
}

impl<Public, Private> core::fmt::Debug for Components<Public, Private>
where
    public::Components<Public>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        // Non-public components are intentionally skipped
        f.debug_struct("Components")
            .field("public_key", &self.public_key)
            .finish()
    }
}

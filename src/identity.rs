use anyhow::Result;
use ruint::aliases::U256;

pub struct Identity {
    private_key: Vec<u8>,
    secret_scalar: U256,
    public_key: Vec<u8>,
    commitment: U256,
}

impl Identity {
    fn sign_message(&self, message: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn generate_commitment(public_key: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

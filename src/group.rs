use anyhow::Result;

use ruint::aliases::U256;
use zk_kit_imt::imt::{IMTMerkleProof, IMT};

pub struct Group {
    pub lean_imt: IMT,
}

impl Group {
    fn new(mebmers: &[U256]) -> Self {
        unimplemented!()
    }

    fn add_member(&self, mebmer: U256) -> Result<()> {
        unimplemented!()
    }
    fn add_members(&self, mebmers: &[U256]) -> Result<()> {
        unimplemented!()
    }
    fn update_member(&self, index: U256, mebmer: U256) -> Result<()> {
        unimplemented!()
    }

    fn generate_merkle_proof(&self, index: U256) -> Result<IMTMerkleProof> {
        unimplemented!()
    }
}

use anyhow::Result;
use ruint::aliases::U256;

use crate::{group::Group, identity::Identity};

type PackedGroth16Proof = [U256; 8];

// Matches the private G1Tup type in ark-circom.
pub type G1 = (U256, U256);

// Matches the private G2Tup type in ark-circom.
pub type G2 = ([U256; 2], [U256; 2]);
pub struct Groth16Proof(pub G1, pub G2, pub G1);

pub struct SemaphorProof {
    merkle_tree_depth: u16,
    merkle_tree_root: U256,
    message: String,
    nullifier: String,
    scope: String,
    points: PackedGroth16Proof,
}

pub struct Proof {}

impl Proof {
    fn generate_proof(
        identity: Identity,
        group: Group,
        message: String,
        scope: String,
        merkle_tree_depth: u16,
    ) -> Result<SemaphorProof> {
        unimplemented!()
    }

    fn verify_proof(proof: SemaphorProof) -> bool {
        unimplemented!()
    }

    fn packGroth16Proof(proof: Groth16Proof) -> PackedGroth16Proof {
        unimplemented!()
    }

    fn unpackGroth16Proof(proof: PackedGroth16Proof) -> Groth16Proof {
        unimplemented!()
    }
}

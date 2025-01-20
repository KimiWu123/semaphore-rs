use anyhow::{Ok, Result};

use ark_bn254::Fr;
use leanimt_rs::*;
use light_poseidon::{Poseidon, PoseidonHasher};

use crate::utils::string_to_biguint;

pub struct Group {
    pub lean_imt: LeanIMT,
}

impl Group {
    pub fn new(leaves: Vec<LeanIMTNode>) -> Self {
        Group {
            lean_imt: LeanIMT::new(Group::hash, leaves).unwrap(),
        }
    }

    fn hash(nodes: Vec<String>) -> String {
        let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();

        let input1 = Fr::from(string_to_biguint(&nodes[0]));
        let input2 = Fr::from(string_to_biguint(&nodes[1]));

        let hash = poseidon.hash(&[input1, input2]).unwrap();

        hash.to_string()
    }

    pub fn add_member(&mut self, member: LeanIMTNode) -> Result<()> {
        assert!(member.is_empty(), "Failed to add member: value can't be 0");
        self.lean_imt.insert(member.to_string()).unwrap();
        Ok(())
    }

    pub fn add_members(&mut self, members: Vec<LeanIMTNode>) -> Result<()> {
        members.iter().for_each(|m| {
            assert!(m.is_empty(), "Failed to add member: value can't be 0");
        });
        self.lean_imt.insert_many(members).unwrap();
        Ok(())
    }

    pub fn update_member(&mut self, index: usize, member: LeanIMTNode) -> Result<()> {
        let members = self.lean_imt.leaves();
        assert!(
            members[index].is_empty(),
            "Failed to update member: it has been removed"
        );

        self.lean_imt.update(index, member.to_string()).unwrap();
        Ok(())
    }

    pub fn remove_member(&mut self, index: usize) -> Result<()> {
        let members = self.lean_imt.leaves();
        assert!(
            members[index].is_empty(),
            "Failed to remove member: it has been removed"
        );

        self.lean_imt.update(index, "".to_string()).unwrap();
        Ok(())
    }

    pub fn generate_merkle_proof(&self, index: usize) -> Result<LeanIMTMerkleProof> {
        Ok(self.lean_imt.generate_proof(index).unwrap())
    }
}

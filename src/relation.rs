use serde::{Deserialize, Serialize};
use crate::entity::Entity;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Relation {
  pub id: u32,
  pub in_use: bool,
  pub first_node_id: u32,
  pub second_node_id: u32,
  pub first_prev_relation_id: u32,
  pub first_next_relation_id: u32,
  pub second_prev_relation_id: u32,
  pub second_next_relation_id: u32,
  pub next_property_id: u32
}

impl Entity for Relation {
  const FILE_PATH: &'static str = "./tmp/relation.bin";
  const SIZE: u32 = 33;
  fn size(&self) -> u32 {
    return Self::SIZE;
  }
  fn get_id(&self) -> u32 {
    return self.id;
  }
  fn new(id: u32) -> Self {
    return Self {
      id,
      in_use: true,
      first_node_id: 0,
      second_node_id: 0,
      first_prev_relation_id: 0,
      first_next_relation_id: 0,
      second_prev_relation_id: 0,
      second_next_relation_id: 0,
      next_property_id: 0
    };
  }
  fn from_bytes(bytes: &[u8]) -> Self {
    return bincode::deserialize(bytes).unwrap();
  }
}

impl Relation {
  pub fn first_next_relation(&self) -> Result<Self, std::io::Error> {
    return Self::open(self.first_next_relation_id);
  }
  pub fn second_next_relation(&self) -> Result<Self, std::io::Error> {
    return Self::open(self.second_next_relation_id);
  }
}
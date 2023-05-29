use serde::{Deserialize, Serialize};
use crate::entity::Entity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
  pub id: u32,
  pub in_use: bool,
  pub next_relation_id: u32,
  pub next_property_id: u32
}

impl Entity for Node {
  const FILE_PATH: &'static str = "./tmp/node.bin";
  const SIZE: u32 = 13;
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
      next_relation_id: 0,
      next_property_id: 0
    };
  }
  fn from_bytes(bytes: &[u8]) -> Self {
    return bincode::deserialize(bytes).unwrap();
  }
}

impl Node {
  pub fn next_relation(&self) -> Result<crate::relation::Relation, std::io::Error> {
    let relation = crate::relation::Relation::open(self.next_relation_id)?;
    return Ok(relation);
  }
  pub fn all_relations(&self) -> Vec<crate::relation::Relation> {
    let mut result = Vec::new();
    let relation = self.next_relation();
    if let Err(_) = relation {
      return result;
    }
    loop {
      print!("{:?} ", relation);
      if let Ok(mut relation) = relation {
        if relation.id == 0 {
          break;
        }
        // TODO(ybabts): This loops infinitely
        result.push(relation.clone());
        if relation.first_node_id == self.id && relation.first_next_relation_id != 0 {
          relation = relation.first_next_relation().unwrap();
        } else if relation.second_next_relation_id == self.id && relation.second_next_relation_id != 0{
          relation = relation.second_next_relation().unwrap();
        }
      }
      if let Err(_) = relation {
        break;
      }
    }
    return result;
  }
}
use crate::entity::Entity;

#[derive(Debug)]
pub struct Node {
  pub id: u64,
  pub in_use: bool,
  pub next_relation_id: u32,
  pub next_property_id: u32
}

impl Entity for Node {
  const ENTITY_NAME: &'static str = "Node";
  const BYTE_LENGTH: usize = 9;
  fn get_byte_length() -> usize {
    return Self::BYTE_LENGTH;
  }
  fn get_id(&self) -> u64 {
    return self.id;
  }
  fn to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(self.in_use as u8);
    bytes.extend_from_slice(&self.next_relation_id.to_le_bytes());
    bytes.extend_from_slice(&self.next_property_id.to_le_bytes());
    return bytes;
  }
  fn from_bytes(id: u64, bytes: &[u8]) -> Result<Node, std::io::Error> {
    if bytes.len() != Self::BYTE_LENGTH {
      return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid byte length"));
    }
    let mut next_relation_id_bytes: [u8; 4] = [0; 4];
    next_relation_id_bytes.copy_from_slice(&bytes[1..5]);
    let mut next_property_id_bytes: [u8; 4] = [0; 4];
    next_property_id_bytes.copy_from_slice(&bytes[5..9]);
    return Ok(Node {
      id,
      in_use: bytes[0] == 1,
      next_relation_id: u32::from_le_bytes(next_relation_id_bytes),
      next_property_id: u32::from_le_bytes(next_property_id_bytes)
    });
  }
  fn get_name() -> &'static str {
    return Self::ENTITY_NAME;
  }
}

impl Node {

}
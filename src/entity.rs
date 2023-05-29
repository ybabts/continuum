use std::{fs::OpenOptions, io::{SeekFrom, Seek, Write, Read, self}};
use serde::{Serialize, Deserialize};

pub trait Entity: Serialize + Deserialize<'static> {
  const FILE_PATH: &'static str;
  const SIZE: u32;
  fn size(&self) -> u32;
  fn get_id(&self) -> u32;
  fn to_bytes(&self) -> Vec<u8> {
    bincode::serialize(&self).unwrap()
  }
  fn from_bytes(bytes: &[u8]) -> Self;
  fn write(&self) {
    let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(Self::FILE_PATH)
      .unwrap();
    file.seek(SeekFrom::Start((self.get_id() * Self::SIZE).into())).unwrap();
    file.write_all(&self.to_bytes()).unwrap();
  }
  fn open(id: u32) -> Result<Self, io::Error> {
    let mut file = OpenOptions::new()
      .read(true)
      .open(Self::FILE_PATH)
      .unwrap();
    file.seek(SeekFrom::Start((id * Self::SIZE).into())).unwrap();
    let mut buffer: Vec<u8> = vec![0; Self::SIZE as usize];
    file.read_exact(&mut buffer)?;
    let entity = Self::from_bytes(&buffer);
    return Ok(entity);
  }
  fn disable(&self) {
    let mut file = OpenOptions::new()
      .write(true)
      .open(Self::FILE_PATH)
      .unwrap();
    file.seek(SeekFrom::Start(((self.get_id() * Self::SIZE) + 4).into())).unwrap();
    let buffer: Vec<u8> = [0].to_vec();
    file.write_all(&buffer).unwrap();
  }
  fn enable(&self) {
    let mut file = OpenOptions::new()
      .write(true)
      .open(Self::FILE_PATH)
      .unwrap();
    file.seek(SeekFrom::Start(((self.get_id() * Self::SIZE) + 4).into())).unwrap();
    let buffer: Vec<u8> = [1].to_vec();
    file.write_all(&buffer).unwrap();
  }
  fn new(id: u32) -> Self;
  fn alloc() -> Self {
    let file = OpenOptions::new()
      .append(true)
      .create(true)
      .open(Self::FILE_PATH)
      .unwrap();
    let id = file.metadata().unwrap().len() / Self::SIZE as u64;
    let entity = Self::new(id as u32);
    entity.enable();
    entity.write();
    return entity;
  }
}
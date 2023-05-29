pub mod node;
use std::{fs::{OpenOptions, File}, io::{SeekFrom, Seek, Read, Write}};

pub trait Entity {
  const ENTITY_NAME: &'static str;
  const BYTE_LENGTH: usize;
  fn get_byte_length() -> usize;
  fn to_bytes(&self) -> Vec<u8>;
  fn from_bytes(id: u64, bytes: &[u8]) -> Result<Self, std::io::Error> where Self: Sized;
  fn get_id(&self) -> u64;
  fn get_name() -> &'static str;
  fn get_file_path() -> String {
    "./tmp/entities/".to_owned() + Self::ENTITY_NAME + ".bin"
  }
  fn create_file() -> Result<(), std::io::Error> {
    match File::create(Self::get_file_path()) {
      Err(e) => return Err(e),
      Ok(mut file) => {
        file.write_all(&vec![0; Self::BYTE_LENGTH])?;
        return Ok(());
      }
    }
  }
  fn file_exists() -> bool {
    return std::path::Path::new(&Self::get_file_path()).exists();
  }
  fn open_file() -> Result<std::fs::File, std::io::Error> {
    if Self::file_exists() == false {
      Self::create_file()?;
    }
    return Ok(OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(Self::get_file_path())?)
  }
  fn seek_to_id(id: u64, file: &mut std::fs::File) -> Result<(), std::io::Error> {
    file.seek(SeekFrom::Start((id * Self::BYTE_LENGTH as u64) as u64))?;
    Ok(())
  }
  fn get_bytes(id: u64) -> Result<Vec<u8>, std::io::Error> {
    let mut file = Self::open_file()?;
    Self::seek_to_id(id, &mut file)?;
    let mut bytes: Vec<u8> = vec![0; Self::BYTE_LENGTH];
    file.read_exact(&mut bytes)?;
    Ok(bytes)
  }
  fn exists(id: u64) -> Result<bool, std::io::Error> {
    let file = Self::open_file()?;
    if file.metadata()?.len() < (id + 1 * Self::BYTE_LENGTH as u64) as u64 {
      return Ok(false);
    }
    return Ok(true);
  }
  fn load(id: u64) -> Result<Self, std::io::Error> where Self: Sized {
    match Self::exists(id) {
      Err(e) => return Err(e),
      Ok(false) => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Entity not found")),
      Ok(true) => {
        let bytes = Self::get_bytes(id)?;
        return Self::from_bytes(id, &bytes)
      }
    }
  }
  fn save(&self) -> Result<(), std::io::Error> {
    let mut file = Self::open_file()?;
    Self::seek_to_id(self.get_id(), &mut file)?;
    file.write_all(&self.to_bytes()).unwrap();
    return Ok(());
  }
  fn alloc() -> Result<u64, std::io::Error> {
    let mut file = Self::open_file()?;
    let pos = file.seek(SeekFrom::End(0))?;
    file.write_all(&vec![0; Self::BYTE_LENGTH])?;
    return Ok(pos / Self::BYTE_LENGTH as u64);
  }
}
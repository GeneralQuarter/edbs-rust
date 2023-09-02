use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct InvalidMapPosition {
    position: MapPosition,
}

impl InvalidMapPosition {
    fn new(position: MapPosition) -> InvalidMapPosition {
        InvalidMapPosition { position }
    }
}

impl fmt::Display for InvalidMapPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid tile position {:?}", self.position)
    }
}

impl Error for InvalidMapPosition {}

pub struct Map {
  width: u32,
  height: u32,
  entity_tiles: Vec<u32>,
}

impl Map {
  pub fn new(width: u32, height: u32) -> Map {
      Map {
          width,
          height,
          entity_tiles: vec![0; (width * height).try_into().unwrap()],
      }
  }

  fn to_index(&self, position: &MapPosition) -> Result<usize, Box<dyn Error>> {
      if position.x >= self.width || position.y >= self.height {
          return Err(Box::new(InvalidMapPosition::new(position.clone())));
      }

      Ok((position.y * self.width + position.x).try_into()?)
  }

  fn to_position(&self, index: usize) -> Result<MapPosition, Box<dyn Error>>  {
    let index_u32 = TryInto::<u32>::try_into(index)?;
    let x = index_u32 % self.width;
    let y = index_u32 / self.width;
    Ok(MapPosition { x, y })
  }

  pub fn set(&mut self, position: &MapPosition, value: u32) -> Result<(), Box<dyn Error>> {
      let index = self.to_index(position)?;
      self.entity_tiles[index] = value;
      Ok(())
  }

  pub fn get(&self, position: &MapPosition) -> Result<u32, Box<dyn Error>> {
      Ok(self.entity_tiles[self.to_index(position)?])
  }

  pub fn swap(&mut self, from: &MapPosition, to: &MapPosition) -> Result<(), Box<dyn Error>> {
      let from_value = self.get(from)?;
      let to_value = self.get(to)?;
      self.set(from, to_value)?;
      self.set(to, from_value)?;
      Ok(())
  }

  fn get_entity_index(&self, entity_id: u32) -> Option<usize> {
    self.entity_tiles.iter().position(|&e| e == entity_id)
  }

  pub fn get_entity_position(&self, entity_id: u32) -> Option<Result<MapPosition, Box<dyn Error>>> {
    self.get_entity_index(entity_id).and_then(|index| Some(self.to_position(index)))
  }
}

impl fmt::Display for Map {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let str_map = self.entity_tiles.iter().enumerate().fold("".to_string(), |mut str, (i, j)| -> String {
          str.push_str(j.to_string().as_str());
          str.push(' ');

          let modulo: usize = i % TryInto::<usize>::try_into(self.width).unwrap();
          let end: usize = (self.width - 1).try_into().unwrap();

          if modulo == end {
              str.push('\n');
          }

          str
      });

      write!(f, "{str_map}")
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_position() {
        let map = Map::new(3, 4);
        let position = map.to_position(11).unwrap();
        assert_eq!(position, MapPosition { x: 2, y: 3})
    }
}

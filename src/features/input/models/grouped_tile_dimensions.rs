use serde::Serialize;

use super::tile_dimensions::TileDimensions;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct GroupedTileDimensions {
    pub group: u8,
    pub instance: TileDimensions,
}
impl GroupedTileDimensions {
    pub(crate) fn from_tile_dimension(tile_dimension: TileDimensions, group: u8) -> Self {
        Self {
            group,
            instance: tile_dimension,
        }
    }
}
impl fmt::Display for GroupedTileDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ТОЧНАЯ копия Java toString(): "id=" + this.id + ", gropup=" + this.group + "[" + this.width + "x" + this.height + ']'

        write!(
            f,
            "id={}, group={}[{}x{}]",
            self.instance.id, self.group, self.instance.width, self.instance.height
        )
    }
}

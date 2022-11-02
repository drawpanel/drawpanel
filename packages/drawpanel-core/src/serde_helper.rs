use geo::Coordinate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Coordinate")]
pub struct CoordinateRef {
    x: f64,
    y: f64,
}

pub mod option_coordinate {
    use super::{Coordinate, CoordinateRef};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Coordinate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "CoordinateRef")] &'a Coordinate);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Coordinate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "CoordinateRef")] Coordinate);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}

pub mod vec_coordinate {
    use super::{Coordinate, CoordinateRef};
    use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Vec<Coordinate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "CoordinateRef")] &'a Coordinate);

        let size = value.len();
        let mut seq = serializer.serialize_seq(Some(size))?;
        for v in value.iter() {
            let helper = Helper(v);
            seq.serialize_element(&helper)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Coordinate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "CoordinateRef")] Coordinate);

        let helper = Vec::deserialize(deserializer)?;
        Ok(helper.iter().map(|Helper(external)| *external).collect())
    }
}

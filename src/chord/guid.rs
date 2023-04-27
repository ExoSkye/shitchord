use serde::{Serialize, Deserialize};

#[cfg(feature = "large_n")]
type GUIDType = u128;

#[cfg(feature = "medium_n")]
type GUIDType = u64;

#[cfg(feature = "small_n")]
type GUIDType = u8;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GUID {
    value: GUIDType
}

impl GUID {
    pub fn new() -> GUID {
        Self {
            #[cfg(feature = "large_n")]
            value: rand::random(),

            #[cfg(feature = "medium_n")]
            value: rand::random(),

            #[cfg(feature = "small_n")]
            value: rand::random(),
        }
    }

    pub fn distance(&self, other: &GUID) -> GUIDType {
        return if self.value < other.value {
            other.value - self.value
        } else {
            other.value.overflowing_add(GUIDType::max_value()).0.overflowing_sub(self.value).0
        }
    }

    pub fn get_length() -> u16 {
        #[cfg(feature = "massive_n")]
        return 512;

        #[cfg(feature = "large_n")]
        return 256;

        #[cfg(feature = "medium_n")]
        return 64;

        #[cfg(feature = "small_n")]
        return 8;
    }
}

impl From<GUIDType> for GUID {
    fn from(value: GUIDType) -> Self {
        Self {
            value
        }
    }
}
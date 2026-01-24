use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use validator::{Validate, ValidationErrors};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, JsonSchema,
)]
#[serde(transparent)]
pub struct Validated<T>(T);

impl<T> Validated<T>
where
    T: Validate,
{
    pub fn new(value: T) -> Result<Self, ValidationErrors> {
        value.validate()?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn new_unchecked(value: T) -> Self {
        Self::new(value).expect("Validation error.")
    }

    #[must_use]
    pub fn get(&self) -> &T {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'de, T> Deserialize<'de> for Validated<T>
where
    T: Deserialize<'de> + Validate,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unvalidated = T::deserialize(deserializer)?;
        Self::new(unvalidated).map_err(Error::custom)
    }
}

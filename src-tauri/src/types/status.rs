/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceStatus {
    /// Instance is active (user is in the world)
    Active,
    /// Instance completed normally (user left intentionally)
    Completed,
    /// Instance was interrupted (VRChat crashed or unexpected termination)
    Interrupted,
    /// Instance encountered sync failure (network synchronization error)
    SyncFailed,
}

impl InstanceStatus {
    /// Convert to database string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Interrupted => "interrupted",
            Self::SyncFailed => "sync_failed",
        }
    }

    /// Parse from database string representation
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "interrupted" => Ok(Self::Interrupted),
            "sync_failed" => Ok(Self::SyncFailed),
            _ => Err(format!("Unknown instance status: {}", s)),
        }
    }
}

impl Default for InstanceStatus {
    fn default() -> Self {
        Self::Active
    }
}

// Serialize for sending to frontend
impl serde::Serialize for InstanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

// Deserialize for receiving from frontend (if needed)
impl<'de> serde::Deserialize<'de> for InstanceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

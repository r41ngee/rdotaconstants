/// Ability properties
impl super::Ability {
    /// Returns true if ability has shard upgrade.
    pub fn has_shard_upgrade(&self) -> bool {
        self.data.get("HasShardUpgrade").map(|v| v.as_str().unwrap_or("0") == "1").unwrap_or(false)
    }

    /// Returns true if ability has scepter upgrade.
    pub fn has_scepter_upgrade(&self) -> bool {
        self.data.get("HasScepterUpgrade").map(|v| v.as_str().unwrap_or("0") == "1").unwrap_or(false)
    }

    /// Returns ability type.
    pub fn ability_type(&self) -> Option<AbilityType> {
        self.data.get("AbilityType").and_then(|v| v.as_str()).and_then(AbilityType::from_str)
    }

    /// Returns true if ability is marked as breakable.
    pub fn is_breakable(&self) -> bool {
        self.data.get("IsBreakable").map(|v| v.as_str().unwrap_or("0") == "1").unwrap_or(false)
    }

    /// Returns true if ability is granted by Aghanim's Shard.
    pub fn is_granted_by_shard(&self) -> bool {
        self.data.get("IsGrantedByShard").map(|v| v.as_str().unwrap_or("0") == "1").unwrap_or(false)
    }

    /// Returns true if ability is granted by Aghanim's Scepter.
    pub fn is_granted_by_scepter(&self) -> bool {
        self.data.get("IsGrantedByScepter").map(|v| v.as_str().unwrap_or("0") == "1").unwrap_or(false)
    }
}

pub enum AbilityType {
    Basic,
    Ultimate,
    Attributes,
    Hidden,
}
impl AbilityType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "DOTA_ABILITY_TYPE_BASIC" => Some(AbilityType::Basic),
            "DOTA_ABILITY_TYPE_ULTIMATE" => Some(AbilityType::Ultimate),
            "DOTA_ABILITY_TYPE_ATTRIBUTES" => Some(AbilityType::Attributes),
            "DOTA_ABILITY_TYPE_HIDDEN" => Some(AbilityType::Hidden),
            _ => None,
        }
    }
}
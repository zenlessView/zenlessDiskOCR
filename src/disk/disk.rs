use {
  crate::source::source::{DISK_AFFIX_NAMES, DISK_SETS},
  std::fmt::Display
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AffixValueType {
  Flat,
  Percentage
}

impl TryFrom<&str> for AffixValueType {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "flat" => Ok(Self::Flat),
      "percentage" => Ok(Self::Percentage),
      _ => Err(format!("Invalid affix value type: {}", value))
    }
  }
}

impl Display for AffixValueType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::Flat => "flat".to_string(),
      Self::Percentage => "percentage".to_string()
    };
    write!(f, "{}", str)
  }
}

#[derive(Clone)]
pub struct Affix {
  pub name: &'static str,
  pub value: f64,
  pub value_type: AffixValueType
}

impl Affix {
  pub fn new(name: &str, value: f64, value_type: AffixValueType) -> Result<Self, String> {
    let name: &'static str = match DISK_AFFIX_NAMES.iter().find(|&&str| str == name) {
      Some(str) => str,
      None => return Err(format!("Invalid affix name: {}", name))
    };

    Ok(Self { name, value, value_type })
  }

  pub fn new_simple(name: &str, value: f64) -> Result<Self, String> {
    Self::new(name, value, AffixValueType::Flat)
  }
}

impl TryFrom<(&str, f64, &str)> for Affix {
  type Error = String;

  fn try_from(value: (&str, f64, &str)) -> Result<Self, Self::Error> {
    let (name, value, value_type) = value;
    let value_type = AffixValueType::try_from(value_type)?;
    Self::new(name, value, value_type)
  }
}

impl PartialEq for Affix {
  fn eq(&self, other: &Self) -> bool {
    self as *const _ == other as *const _
      || (self.name == other.name
        && self.value == other.value
        && self.value_type == other.value_type)
  }
}

impl Eq for Affix {
}

pub struct Disk {
  pub set: &'static str,
  pub slot: u8,
  pub primary_affix: Affix,
  pub secondary_affixes: [Option<Affix>; 4]
}

impl Disk {
  pub fn new(
    set: &str,
    slot: u8,
    primary_affix: Affix,
    secondary_affix_1: Option<Affix>,
    secondary_affix_2: Option<Affix>,
    secondary_affix_3: Option<Affix>,
    secondary_affix_4: Option<Affix>
  ) -> Result<Self, String> {
    let set: &'static str = match DISK_SETS.iter().find(|&&str| str == set) {
      Some(str) => str,
      None => return Err(format!("Invalid disk set: {}", set))
    };

    Ok(Self {
      set,
      slot,
      primary_affix,
      secondary_affixes: [
        secondary_affix_1,
        secondary_affix_2,
        secondary_affix_3,
        secondary_affix_4
      ]
    })
  }

  pub fn new_simple(
    set: &str,
    slot: u8,
    primary_affix: Affix,
    secondary_affix_1: Affix,
    secondary_affix_2: Affix,
    secondary_affix_3: Affix
  ) -> Result<Self, String> {
    Self::new(
      set,
      slot,
      primary_affix,
      Some(secondary_affix_1),
      Some(secondary_affix_2),
      Some(secondary_affix_3),
      None
    )
  }

  pub fn potential_eq(&self, other: &Self) -> bool {
    self == other
      || (self.set == other.set
        && self.slot == other.slot
        && self.primary_affix == other.primary_affix
        && self.secondary_affixes[0] == other.secondary_affixes[0]
        && self.secondary_affixes[1] == other.secondary_affixes[1]
        && self.secondary_affixes[2] == other.secondary_affixes[2]
        && self.secondary_affixes[3] == other.secondary_affixes[3])
  }

  pub fn get_static_set_name(set: &str) -> Option<&'static str> {
    DISK_SETS.iter().find(|&&str| str == set).map(|&str| str)
  }

  pub fn get_static_affix_name(name: &str) -> Option<&'static str> {
    DISK_AFFIX_NAMES.iter().find(|&&str| str == name).map(|&str| str)
  }
}

impl PartialEq for Disk {
  fn eq(&self, other: &Self) -> bool {
    self as *const _ == other as *const _
  }
}

impl Eq for Disk {
}

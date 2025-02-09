use crate::source::source::{DISK_AFFIX_NAMES, DISK_SETS};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AffixValueType {
  Flat,
  Percentage,
}

pub struct Affix {
  pub name: &'static str,
  pub value: f64,
  pub value_type: AffixValueType,
}

impl Affix {
  pub fn new(name: &str, value: f64, value_type: AffixValueType) -> Result<Self, String> {
    let name: &'static str = match DISK_AFFIX_NAMES.iter().find(|&&str| str == name) {
      Some(str) => str,
      None => return Err(format!("Invalid affix name: {}", name)),
    };

    Ok(Self {
      name,
      value,
      value_type,
    })
  }

  pub fn new_simple(name: &str, value: f64) -> Result<Self, String> {
    Self::new(name, value, AffixValueType::Flat)
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

impl Eq for Affix {}

pub struct Disk {
  pub set: &'static str,
  pub slot: u8,
  pub primary_affix: Affix,
  pub secondary_affixes: [Option<Affix>; 4],
}

impl Disk {
  pub fn new(
    set: &str,
    slot: u8,
    primary_affix: Affix,
    secondary_affix_1: Option<Affix>,
    secondary_affix_2: Option<Affix>,
    secondary_affix_3: Option<Affix>,
    secondary_affix_4: Option<Affix>,
  ) -> Result<Self, String> {
    let set: &'static str = match DISK_SETS.iter().find(|&&str| str == set) {
      Some(str) => str,
      None => return Err(format!("Invalid disk set: {}", set)),
    };

    Ok(Self {
      set,
      slot,
      primary_affix,
      secondary_affixes: [
        secondary_affix_1,
        secondary_affix_2,
        secondary_affix_3,
        secondary_affix_4,
      ],
    })
  }

  pub fn new_simple(
    set: &str,
    slot: u8,
    primary_affix: Affix,
    secondary_affix_1: Affix,
    secondary_affix_2: Affix,
    secondary_affix_3: Affix,
  ) -> Result<Self, String> {
    Self::new(
      set,
      slot,
      primary_affix,
      Some(secondary_affix_1),
      Some(secondary_affix_2),
      Some(secondary_affix_3),
      None,
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
}

impl PartialEq for Disk {
  fn eq(&self, other: &Self) -> bool {
    self as *const _ == other as *const _
  }
}

impl Eq for Disk {}
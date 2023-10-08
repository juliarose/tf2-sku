//! Container for holding up to 2 spells.

use std::fmt;
use std::hash::{Hash, Hasher};
use tf2_enum::Spell;

/// Contains up to 2 spells.
/// 
/// # Examples
/// ```
/// use tf2_sku::Spells;
/// use tf2_enum::Spell;
/// 
/// let mut spells = Spells::new([
///     Some(Spell::HeadlessHorseshoes),
///     None,
/// ]);
/// 
/// // Check that spells contains Headless Horseshoes.
/// assert!(spells.contains(&Spell::HeadlessHorseshoes));
/// assert_eq!(spells.len(), 1);
/// 
/// // Add a spell.
/// spells.push(Spell::VoicesFromBelow);
/// 
/// assert_eq!(spells.len(), 2);
/// 
/// // If a spell is added when spells are full, it will be ignored.
/// spells.push(Spell::PumpkinBombs);
/// 
/// assert!(!spells.contains(&Spell::PumpkinBombs));
/// ```
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct Spells {
    inner: [Option<Spell>; 2]
}

impl Spells {
    /// Creates a container for spells.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::Spells;
    /// use tf2_enum::Spell;
    /// 
    /// let spells = Spells::new([
    ///     Some(Spell::HeadlessHorseshoes),
    ///     None,
    /// ]);
    /// ```
    pub fn new(inner: [Option<Spell>; 2]) -> Self {
        let mut inner = inner;
        
        // remove duplicates
        for i in 0..=1 {
            for j in 0..=1 {
                if i == j {
                    continue;
                }
                
                if let Some(si) = inner[i] {
                    if let Some(sj) = inner[j] {
                        if si.attribute_defindex() == sj.attribute_defindex() {
                            inner[i] = None;
                        }
                    }
                }
            }
        }
        
        Self {
            inner,
        }
    }
    
    /// Clears spells.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::Spells;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = Spells::new([
    ///     Some(Spell::HeadlessHorseshoes),
    ///     None,
    /// ]);
    /// 
    /// spells.clear();
    /// 
    /// assert_eq!(spells.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.inner = [None, None];
    }
    
    /// Adds a spell to the first available slot. If no slots are available or a spell of the same 
    /// type already exists, the new spell will be ignored.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::Spells;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = Spells::new([
    ///     Some(Spell::HeadlessHorseshoes),
    ///     None,
    /// ]);
    /// 
    /// assert_eq!(spells.len(), 1);
    /// 
    /// spells.push(Spell::VoicesFromBelow);
    /// 
    /// assert_eq!(spells.len(), 2);
    /// ```
    pub fn push(&mut self, spell: Spell) {
        let attribute_defindex = spell.attribute_defindex();
        
        for s in self.inner.iter_mut() {
            if let Some(ss) = s {
                if ss.attribute_defindex() == attribute_defindex {
                    *s = Some(spell);
                    
                    return;
                }
            }
        }
        
        for index in 0..=1usize {
            if self.inner[index].is_none() {
                self.inner[index] = Some(spell);
                return;
            }
        }
    }
    
    /// Removes a spell.
    pub fn remove(&mut self, spell: Spell) {
        if self.inner[0] == Some(spell) {
            self.inner[0] = None;
        } else if self.inner[1] == Some(spell) {
            self.inner[1] = None;
        }
    }
    
    /// Checks if spells are empty.
    pub fn is_empty(&self) -> bool {
        self.inner
            .iter()
            .all(|s| s.is_none())
    }
    
    /// Checks if contains spell.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::Spells;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = Spells::new([
    ///     Some(Spell::HeadlessHorseshoes),
    ///     None,
    /// ]);
    /// 
    /// assert!(spells.contains(&Spell::HeadlessHorseshoes));
    /// ```
    pub fn contains(&self, spell: &Spell) -> bool {
        self.inner.contains(&Some(*spell))
    }
    
    /// Gets the length of contained spells.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::Spells;
    /// use tf2_enum::Spell;
    /// 
    /// let spells = Spells::new([
    ///     Some(Spell::HeadlessHorseshoes),
    ///     None,
    /// ]);
    /// 
    /// assert_eq!(spells.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.inner
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
}

impl PartialEq<Self> for Spells {
    fn eq(&self, other: &Self) -> bool {
        self.inner[0] == other.inner[0] && self.inner[1] == other.inner[1] || 
        self.inner[0] == other.inner[1] && self.inner[1] == other.inner[0]
    }
}

impl Hash for Spells {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.inner[0] <= self.inner[1] {
            self.inner[0].hash(state);
            self.inner[1].hash(state);
        } else {
            self.inner[1].hash(state);
            self.inner[0].hash(state);
        }
    }
}

impl IntoIterator for Spells {
    type Item = Spell;
    type IntoIter = SpellsIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        SpellsIterator {
            inner: self,
            index: 0,
        }
    }
}

impl IntoIterator for &Spells {
    type Item = Spell;
    type IntoIter = SpellsIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        SpellsIterator {
            inner: *self,
            index: 0,
        }
    }
}

/// Iterates over spells.
#[derive(Debug)]
pub struct SpellsIterator {
    inner: Spells,
    index: usize,
}

impl Iterator for SpellsIterator {
    type Item = Spell;
    
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(s) = self.inner.inner.get(self.index) {
            self.index += 1;
            
            if let Some(s) = s {
                // stop at first filled slot
                return Some(*s);
            }
        }
        
        None
    }
}

impl fmt::Display for Spells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
    }
}

pub(crate) fn spell_label(spell: &Spell) -> &'static str {
    match spell.attribute_defindex() {
        Spell::DEFINDEX_VOICES_FROM_BELOW => "voices",
        Spell::DEFINDEX_EXORCISM => "exorcism",
        Spell::DEFINDEX_PUMPKIN_BOMBS => "pumpkinbombs",
        Spell::DEFINDEX_HALLOWEEN_FIRE => "halloweenfire",
        Spell::DEFINDEX_PAINT => "paintspell-",
        Spell::DEFINDEX_FOOTPRINTS => "footprints-",
        _ => "",
    }
}
    
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn spells_equals() {
        assert_eq!(Spells::new([
            Some(Spell::Exorcism),
            Some(Spell::HalloweenFire),
        ]), Spells::new([
            Some(Spell::HalloweenFire),
            Some(Spell::Exorcism),
        ]));
    }
    
    #[test]
    fn spells_no_duplicates() {
        assert_eq!(Spells::new([
            Some(Spell::Exorcism),
            Some(Spell::Exorcism),
        ]), Spells::new([
            Some(Spell::Exorcism),
            None,
        ]));
        
        assert_eq!(Spells::new([
            Some(Spell::TeamSpiritFootprints),
            Some(Spell::HeadlessHorseshoes),
        ]), Spells::new([
            Some(Spell::HeadlessHorseshoes),
            None,
        ]));
    }
}
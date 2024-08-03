//! Set for holding up to 2 spells.

use crate::error::InsertError;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Sub;
use tf2_enum::Spell;

const SPELL_COUNT: usize = 2;

/// Contains up to 2 spells. Although the underlying data structure is an array, it behaves like a 
/// set. Most methods mimic that of [`HashSet`](std::collections::HashSet).
/// 
/// This struct solves the following problems:
/// - An item can only hold up to 2 spells.
/// - An item cannot have duplicate spells.
/// - Comparing spells for equality is order-agnostic.
/// - Hashing is order-agnostic.
/// 
/// Since the number of spells is fixed, the struct uses zero heap allocations and is therefore 
/// [`Copy`].
/// 
/// # Examples
/// ```
/// use tf2_sku::SpellSet;
/// use tf2_enum::Spell;
/// 
/// // Create a set for spells with one spell.
/// let mut spells = SpellSet::single(Spell::HeadlessHorseshoes);
/// 
/// // Check that spells contains Headless Horseshoes.
/// assert!(spells.contains(&Spell::HeadlessHorseshoes));
/// assert_eq!(spells.len(), 1);
/// 
/// // Add a spell.
/// spells.insert(Spell::VoicesFromBelow).ok();
/// assert_eq!(spells.len(), 2);
/// 
/// // If a spell is added when spells are full, an error is returned.
/// assert!(spells.insert(Spell::PumpkinBombs).is_err());
/// assert!(!spells.contains(&Spell::PumpkinBombs));
/// 
/// // Iterate over spells.
/// for spell in spells {
///     println!("{}", spell);
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct SpellSet {
    inner: [Option<Spell>; SPELL_COUNT]
}

impl SpellSet {
    /// Creates a set for spells.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// 
    /// let spells = SpellSet::new();
    /// 
    /// assert!(spells.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a set for spells with one spell.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// assert_eq!(spells.len(), 1);
    /// ```
    pub fn single(spell: Spell) -> Self {
        Self {
            inner: [
                Some(spell),
                None,
            ],
        }
    }
    
    /// Creates a set for spells with two spells.
    /// 
    /// If the same spell is added multiple times, only one will be kept.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells = SpellSet::double(Spell::HeadlessHorseshoes, Spell::VoicesFromBelow);
    /// 
    /// assert_eq!(spells.len(), 2);
    /// ```
    pub fn double(spell1: Spell, spell2: Spell) -> Self {
        Self::from([
            Some(spell1),
            Some(spell2),
        ])
    }
    
    /// Clears the set, removing all spells.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// spells.clear();
    /// 
    /// assert_eq!(spells.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.inner = [None, None];
    }
    
    /// Adds a spell to the first available slot.
    /// 
    /// # Errors
    /// - If the set is full.
    /// - If the spell is already in the set.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// assert_eq!(spells.len(), 1);
    /// 
    /// spells.insert(Spell::VoicesFromBelow).ok();
    /// 
    /// assert_eq!(spells.len(), 2);
    /// 
    /// // Spells are full.
    /// assert!(spells.insert(Spell::PumpkinBombs).is_err());
    /// ```
    pub fn insert(&mut self, spell: Spell) -> Result<(), InsertError> {
        let attribute_defindex = spell.attribute_defindex();
        
        for s in self.inner.iter_mut().flatten() {
            if s.attribute_defindex() == attribute_defindex {
                return Err(InsertError::Duplicate);
            }
        }
        
        for index in 0..SPELL_COUNT {
            if self.inner[index].is_none() {
                self.inner[index] = Some(spell);
                return Ok(());
            }
        }
        
        Err(InsertError::Full)
    }
    
    /// Removes a spell from the set. Returns whether the value was present in the set.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// assert!(spells.remove(&Spell::HeadlessHorseshoes));
    /// assert!(!spells.contains(&Spell::HeadlessHorseshoes));
    /// ```
    pub fn remove(&mut self, spell: &Spell) -> bool {
        if self.inner[0] == Some(*spell) {
            self.inner[0] = None;
            true
        } else if self.inner[1] == Some(*spell) {
            self.inner[1] = None;
            true
        } else {
            false
        }
    }
    
    /// Removes and returns the spell in the set, if any, that is equal to the given one.
    pub fn take(&mut self, spell: &Spell) -> Option<Spell> {
        if self.inner[0] == Some(*spell) {
            self.inner[0] = None;
            return Some(*spell);
        } else if self.inner[1] == Some(*spell) {
            self.inner[1] = None;
            return Some(*spell);
        }
        
        None
    }
    
    /// Returns `true` if the set contains no spells.
    pub fn is_empty(&self) -> bool {
        self.inner
            .iter()
            .all(|s| s.is_none())
    }
    
    /// Returns `true` if the set contains a spell.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let mut spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// assert!(spells.contains(&Spell::HeadlessHorseshoes));
    /// ```
    pub fn contains(&self, spell: &Spell) -> bool {
        self.inner.contains(&Some(*spell))
    }
    
    /// Returns the number of spells in the set.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells = SpellSet::single(Spell::HeadlessHorseshoes);
    /// 
    /// assert_eq!(spells.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.inner
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
    
    /// Returns the spells that are in `self` but not in `other`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells1 = SpellSet::double(Spell::HalloweenFire, Spell::Exorcism);
    /// let spells2 = SpellSet::double(Spell::HalloweenFire, Spell::VoicesFromBelow);
    /// let difference = spells1.difference(&spells2);
    /// 
    /// assert_eq!(difference, SpellSet::single(Spell::Exorcism));
    /// 
    /// let difference = spells2.difference(&spells1);
    /// 
    /// assert_eq!(difference, SpellSet::single(Spell::VoicesFromBelow));
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let mut inner = [None, None];
        
        for (i, s_option) in inner.iter_mut().enumerate() {
            if let Some(s) = self.inner[i] {
                if !other.contains(&s) {
                    *s_option = Some(s);
                }
            }
        }
        
        Self {
            inner,
        }
    }
    
    /// Returns the spells that are both in `self` and `other`.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells1 = SpellSet::double(Spell::HalloweenFire, Spell::Exorcism);
    /// let spells2 = SpellSet::double(Spell::HalloweenFire, Spell::VoicesFromBelow);
    /// let intersection = spells1.intersection(&spells2);
    /// 
    /// assert_eq!(intersection, SpellSet::single(Spell::HalloweenFire));
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let mut inner = [None, None];
        
        for (i, s_option) in inner.iter_mut().enumerate() {
            if let Some(s) = self.inner[i] {
                if other.contains(&s) {
                    *s_option = Some(s);
                }
            }
        }
        
        Self {
            inner,
        }
    }
    
    /// Returns `true` if `self` has no spells in common with `other`. This is equivalent to 
    /// checking for an empty intersection.
    /// 
    /// # Examples
    /// ```
    /// use tf2_sku::SpellSet;
    /// use tf2_enum::Spell;
    /// 
    /// let spells1 = SpellSet::double(Spell::HalloweenFire, Spell::Exorcism);
    /// let spells2 = SpellSet::double(Spell::HalloweenFire, Spell::VoicesFromBelow);
    /// 
    /// assert!(!spells1.is_disjoint(&spells2));
    /// ```
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }
}

impl From<[Option<Spell>; SPELL_COUNT]> for SpellSet {
    fn from(inner: [Option<Spell>; SPELL_COUNT]) -> Self {
        let mut inner = inner;
        
        // remove duplicates
        for i in 0..SPELL_COUNT {
            for j in 0..SPELL_COUNT {
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
}

// Only Sub is implemented because Add wouldn't make much sense with spells being limited to 2.
impl Sub for SpellSet {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        self.difference(&other)
    }
}

impl PartialEq<Self> for SpellSet {
    fn eq(&self, other: &Self) -> bool {
        (self.inner[0] == other.inner[0] && self.inner[1] == other.inner[1]) || 
        (self.inner[0] == other.inner[1] && self.inner[1] == other.inner[0])
    }
}

impl Hash for SpellSet {
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

impl FromIterator<Spell> for SpellSet {
    fn from_iter<I: IntoIterator<Item = Spell>>(iter: I) -> Self {
        let mut spell_set = Self::new();
        
        for spell in iter {
            spell_set.insert(spell).ok();
        }
        
        spell_set
    }
}

impl IntoIterator for SpellSet {
    type Item = Spell;
    type IntoIter = SpellSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        SpellSetIterator {
            inner: self,
            index: 0,
        }
    }
}

impl IntoIterator for &SpellSet {
    type Item = Spell;
    type IntoIter = SpellSetIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        SpellSetIterator {
            inner: *self,
            index: 0,
        }
    }
}

/// Iterates over spells.
#[derive(Debug)]
pub struct SpellSetIterator {
    inner: SpellSet,
    index: usize,
}

impl Iterator for SpellSetIterator {
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

impl fmt::Display for SpellSet {
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
    use std::collections::HashSet;
    
    #[test]
    fn spell_set_equals() {
        assert_eq!(SpellSet::from([
            Some(Spell::Exorcism),
            Some(Spell::HalloweenFire),
        ]), SpellSet::from([
            Some(Spell::HalloweenFire),
            Some(Spell::Exorcism),
        ]));
    }
    
    #[test]
    fn spell_set_hashes() {
        let mut set = HashSet::new();
        
        set.insert(SpellSet::from([
            Some(Spell::Exorcism),
            Some(Spell::HalloweenFire),
        ]));
        
        assert!(set.contains(&SpellSet::from([
            Some(Spell::HalloweenFire),
            Some(Spell::Exorcism),
        ])));
    }
    
    #[test]
    fn spell_set_no_duplicates() {
        assert_eq!(SpellSet::from([
            Some(Spell::Exorcism),
            Some(Spell::Exorcism),
        ]), SpellSet::from([
            Some(Spell::Exorcism),
            None,
        ]));
        
        assert_eq!(SpellSet::from([
            Some(Spell::TeamSpiritFootprints),
            Some(Spell::HeadlessHorseshoes),
        ]), SpellSet::from([
            Some(Spell::HeadlessHorseshoes),
            None,
        ]));
    }
    
    #[test]
    fn iterates_spells() {
        let spells = SpellSet::from([
            Some(Spell::Exorcism),
            Some(Spell::HalloweenFire),
        ]);
        let mut count = 0;
        
        for _spell in spells {
            count += 1;
        }
        
        assert_eq!(count, 2);
        
        let spells = spells.into_iter().collect::<Vec<_>>();
        
        assert_eq!(spells, vec![Spell::Exorcism, Spell::HalloweenFire]);
        
        let spells = SpellSet::from_iter(spells);
        
        assert_eq!(spells, SpellSet::from([
            Some(Spell::Exorcism),
            Some(Spell::HalloweenFire),
        ]));
    }
    
    #[test]
    fn sub() {
        let spells1 = SpellSet::from([
            Some(Spell::HalloweenFire),
            Some(Spell::Exorcism),
        ]);
        let spells2 = SpellSet::from([
            Some(Spell::HalloweenFire),
            Some(Spell::VoicesFromBelow),
        ]);
        
        let difference = spells1 - spells2;
        
        assert_eq!(difference, SpellSet::from([
            Some(Spell::Exorcism),
            None,
        ]));
    }
}

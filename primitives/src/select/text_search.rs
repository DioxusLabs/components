//! Text search and matching algorithms for the select component.

use crate::select::context::OptionState;
use std::collections::HashMap;

/// Find the best matching option based on typeahead input
pub(super) fn best_match<T: Clone + PartialEq + 'static>(
    keyboard_layout: &KeyboardLayout,
    typeahead: &str,
    options: &[OptionState<T>],
) -> Option<usize> {
    if typeahead.is_empty() {
        return None;
    }

    let typeahead_characters: Box<[_]> = typeahead.chars().collect();

    options
        .iter()
        .map(|opt| {
            let value = &opt.text_value;
            let value_characters: Box<[_]> = value.chars().collect();
            let distance =
                normalized_distance(&typeahead_characters, &value_characters, keyboard_layout);
            (distance, opt.tab_index)
        })
        .min_by(|(d1, _), (d2, _)| f32::total_cmp(d1, d2))
        .map(|(_, value)| value)
}

/// Calculate normalized distance between typeahead and value characters
pub(super) fn normalized_distance(
    typeahead_characters: &[char],
    value_characters: &[char],
    keyboard_layout: &KeyboardLayout,
) -> f32 {
    // Only use the the start of the value characters
    let value_characters =
        &value_characters[..value_characters.len().min(typeahead_characters.len())];
    // Only use the end of the typeahead characters
    let typeahead_characters = &typeahead_characters[typeahead_characters
        .len()
        .saturating_sub(value_characters.len())..];

    levenshtein_distance(typeahead_characters, value_characters, |a, b| {
        keyboard_layout.substitution_cost(a, b)
    })
}

/// The recency bias of the levenshtein distance function
pub(super) fn recency_bias(char_index: usize, total_length: usize) -> f32 {
    ((char_index as f32 + 1.5).ln() / (total_length as f32 + 1.5).ln()).powi(4)
}

/// Weighted Levenshtein distance to account for the recency of characters
/// More recent characters have a higher weight, while older characters have a lower weight
/// The first few characters in the value are weighted more heavily
///
/// When substitution is required, the substitution is cheaper for characters that are closer together on the keyboard
pub(super) fn levenshtein_distance(
    typeahead: &[char],
    value: &[char],
    substitution_cost: impl Fn(char, char) -> f32,
) -> f32 {
    let mut dp = vec![vec![0.0; value.len() + 1]; typeahead.len() + 1];

    let mut prev = 0.0;
    for j in 0..=value.len() {
        let weight = recency_bias(j, value.len());
        dp[0][j] = prev + weight;
        prev = dp[0][j];
    }

    for i in 1..=typeahead.len() {
        dp[i][0] = dp[i - 1][0] + 1.0;
        for j in 1..=value.len() {
            let weight = recency_bias(j - 1, value.len());
            let cost = if typeahead[i - 1] == value[j - 1] {
                0.0
            } else {
                substitution_cost(typeahead[i - 1], value[j - 1]) * weight
            };

            dp[i][j] = (dp[i - 1][j] + weight)
                .min(dp[i][j - 1] + weight)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[typeahead.len()][value.len()]
}

/// Adaptive keyboard learning system for multi-language support
#[derive(Debug, Clone)]
pub struct AdaptiveKeyboard {
    /// Learned substitution costs between characters
    confusion_matrix: HashMap<(char, char), f32>,
    /// Physical key position mappings learned from events
    physical_mappings: HashMap<String, char>,
}

impl Default for AdaptiveKeyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveKeyboard {
    /// Create a new adaptive keyboard system
    pub fn new() -> Self {
        Self {
            confusion_matrix: HashMap::new(),
            physical_mappings: HashMap::new(),
        }
    }

    /// Learn from a keyboard event mapping physical key to logical character
    pub fn learn_from_event(&mut self, physical_code: &str, logical_char: char) {
        self.physical_mappings
            .insert(physical_code.to_string(), logical_char);
    }

    /// Record a user correction to improve future matching
    #[allow(dead_code)]
    pub fn record_correction(&mut self, typed: char, intended: char) {
        if typed == intended {
            return;
        }

        let current = self
            .confusion_matrix
            .get(&(typed, intended))
            .copied()
            .unwrap_or(1.0);
        // Gradually reduce substitution cost for this pair
        self.confusion_matrix
            .insert((typed, intended), (current * 0.9).max(0.1));
    }

    /// Get learned substitution cost between two characters
    pub fn get_learned_cost(&self, a: char, b: char) -> Option<f32> {
        self.confusion_matrix
            .get(&(a, b))
            .or_else(|| self.confusion_matrix.get(&(b, a)))
            .copied()
    }

    /// Calculate hybrid substitution cost using multiple strategies
    pub fn substitution_cost(&self, a: char, b: char) -> f32 {
        if a == b {
            return 0.0;
        }

        // 1. Check learned patterns first (highest priority)
        if let Some(cost) = self.get_learned_cost(a, b) {
            return cost;
        }

        // 2. Try physical key distance if we have mappings
        if let Some(cost) = self.physical_key_cost(a, b) {
            return cost * 0.3; // Physical proximity is a strong signal
        }

        // 3. Use Unicode codepoint similarity
        let unicode_cost = self.unicode_similarity_cost(a, b);

        // 4. Check phonetic similarity
        let phonetic_cost = self.phonetic_similarity_cost(a, b);

        // Return the minimum of unicode and phonetic costs
        unicode_cost.min(phonetic_cost)
    }

    /// Calculate cost based on physical key positions
    fn physical_key_cost(&self, a: char, b: char) -> Option<f32> {
        // Find physical keys that produce these characters
        let key_a = self.physical_mappings.iter().find(|(_, &ch)| ch == a)?.0;
        let key_b = self.physical_mappings.iter().find(|(_, &ch)| ch == b)?.0;

        let distance = physical_key_distance(key_a, key_b)?;
        Some((distance / 10.0).min(1.0).max(0.1))
    }

    /// Calculate similarity based on Unicode codepoint proximity
    fn unicode_similarity_cost(&self, a: char, b: char) -> f32 {
        let diff = (a as u32).abs_diff(b as u32) as f32;

        // Characters close in Unicode are often similar
        // Scale: adjacent codepoints get ~0.1 cost, distant ones approach 1.0
        (diff / 100.0).min(1.0).max(0.1)
    }

    /// Check phonetic similarity using small lookup groups
    fn phonetic_similarity_cost(&self, a: char, b: char) -> f32 {
        // Small groups of phonetically similar characters across scripts
        const PHONETIC_GROUPS: &[&[char]] = &[
            // "A" sounds
            &['a', 'а', 'α', 'ا', 'আ'],
            // "B" sounds
            &['b', 'б', 'β', 'ب', 'ব'],
            // "S" sounds
            &['s', 'с', 'σ', 'س', 'স'],
            // "T" sounds
            &['t', 'т', 'τ', 'ت', 'ত'],
            // "N" sounds
            &['n', 'н', 'ν', 'ن', 'ন'],
            // "R" sounds
            &['r', 'р', 'ρ', 'ر', 'র'],
            // "L" sounds
            &['l', 'л', 'λ', 'ل', 'ল'],
            // "M" sounds
            &['m', 'м', 'μ', 'م', 'ম'],
            // "K" sounds
            &['k', 'к', 'κ', 'ك', 'ক'],
            // "P" sounds
            &['p', 'п', 'π', 'پ', 'প'],
            // "F" sounds
            &['f', 'ф', 'φ'],
        ];

        for group in PHONETIC_GROUPS {
            if group.contains(&a) && group.contains(&b) {
                return 0.2; // Low cost for phonetically similar characters
            }
        }

        1.0 // Default high cost
    }
}

/// Calculate distance between physical keys based on QWERTY layout
fn physical_key_distance(code1: &str, code2: &str) -> Option<f32> {
    let pos1 = physical_key_position(code1)?;
    let pos2 = physical_key_position(code2)?;

    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;
    Some((dx * dx + dy * dy).sqrt())
}

/// Get universal physical position of a key (layout-independent)
fn physical_key_position(code: &str) -> Option<(f32, f32)> {
    match code {
        // Top row
        "KeyQ" => Some((0.0, 0.0)),
        "KeyW" => Some((1.0, 0.0)),
        "KeyE" => Some((2.0, 0.0)),
        "KeyR" => Some((3.0, 0.0)),
        "KeyT" => Some((4.0, 0.0)),
        "KeyY" => Some((5.0, 0.0)),
        "KeyU" => Some((6.0, 0.0)),
        "KeyI" => Some((7.0, 0.0)),
        "KeyO" => Some((8.0, 0.0)),
        "KeyP" => Some((9.0, 0.0)),

        // Home row
        "KeyA" => Some((0.5, 1.0)),
        "KeyS" => Some((1.5, 1.0)),
        "KeyD" => Some((2.5, 1.0)),
        "KeyF" => Some((3.5, 1.0)),
        "KeyG" => Some((4.5, 1.0)),
        "KeyH" => Some((5.5, 1.0)),
        "KeyJ" => Some((6.5, 1.0)),
        "KeyK" => Some((7.5, 1.0)),
        "KeyL" => Some((8.5, 1.0)),

        // Bottom row
        "KeyZ" => Some((1.0, 2.0)),
        "KeyX" => Some((2.0, 2.0)),
        "KeyC" => Some((3.0, 2.0)),
        "KeyV" => Some((4.0, 2.0)),
        "KeyB" => Some((5.0, 2.0)),
        "KeyN" => Some((6.0, 2.0)),
        "KeyM" => Some((7.0, 2.0)),

        // Number row
        "Digit1" => Some((0.0, -1.0)),
        "Digit2" => Some((1.0, -1.0)),
        "Digit3" => Some((2.0, -1.0)),
        "Digit4" => Some((3.0, -1.0)),
        "Digit5" => Some((4.0, -1.0)),
        "Digit6" => Some((5.0, -1.0)),
        "Digit7" => Some((6.0, -1.0)),
        "Digit8" => Some((7.0, -1.0)),
        "Digit9" => Some((8.0, -1.0)),
        "Digit0" => Some((9.0, -1.0)),

        _ => None,
    }
}

/// Supported keyboard layouts for optimized text matching
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum KeyboardLayout {
    Qwerty,
    ColemakDH,
    Colemak,
    Dvorak,
    Workman,
    Azerty,
    Qwertz,
    Adaptive(AdaptiveKeyboard), // New adaptive option
    Unknown,
}

impl KeyboardLayout {
    #[allow(dead_code)]
    const KNOWN_KEYBOARD_LAYOUTS: &'static [KeyboardLayout] = &[
        KeyboardLayout::Qwerty,
        KeyboardLayout::ColemakDH,
        KeyboardLayout::Colemak,
        KeyboardLayout::Dvorak,
        KeyboardLayout::Workman,
        KeyboardLayout::Azerty,
        KeyboardLayout::Qwertz,
    ];

    /// Guess the keyboard layout based on observed key positions
    #[allow(dead_code)]
    pub fn guess(known_key_positions: &HashMap<char, char>) -> KeyboardLayout {
        let mut best_layout = KeyboardLayout::Unknown;
        let mut best_score = 0;

        for layout in Self::KNOWN_KEYBOARD_LAYOUTS {
            let mut score = 0;
            for (from, to) in known_key_positions {
                if let (Some(from_pos), Some(to_pos)) =
                    (layout.char_position(*from), layout.char_position(*to))
                {
                    // If the positions are the same, it's a match
                    if from_pos == to_pos {
                        score += 1;
                    }
                }
            }
            if score > best_score {
                best_score = score;
                best_layout = layout.clone();
            }
        }

        best_layout
    }

    /// Calculate substitution cost between two characters based on keyboard distance
    pub fn substitution_cost(&self, a: char, b: char) -> f32 {
        match self {
            KeyboardLayout::Adaptive(adaptive) => adaptive.substitution_cost(a, b),
            _ => self.legacy_substitution_cost(a, b),
        }
    }

    /// Legacy substitution cost for hardcoded layouts
    fn legacy_substitution_cost(&self, a: char, b: char) -> f32 {
        if a == b {
            return 0.0;
        }

        let (a_pos, b_pos) = match (self.char_position(a), self.char_position(b)) {
            (Some(a_pos), Some(b_pos)) => (a_pos, b_pos),
            _ => return 1.0,
        };

        let dx = (a_pos.0 as f32 - b_pos.0 as f32).abs();
        let dy = (a_pos.1 as f32 - b_pos.1 as f32).abs();
        let distance = (dx * dx + dy * dy).sqrt();

        // Scale the distance to be between 0.1 and 1.0
        (distance / 10.0).min(1.0).max(0.1)
    }

    /// Get the position of a character on the keyboard layout
    fn char_position(&self, c: char) -> Option<(usize, usize)> {
        let layout = match self {
            KeyboardLayout::Qwerty => &QWERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::ColemakDH => &COLEMACK_DH_KEYBOARD_LAYOUT,
            KeyboardLayout::Colemak => &COLEMAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Dvorak => &DVORAK_KEYBOARD_LAYOUT,
            KeyboardLayout::Workman => &WORKMAN_KEYBOARD_LAYOUT,
            KeyboardLayout::Azerty => &AZERTY_KEYBOARD_LAYOUT,
            KeyboardLayout::Qwertz => &QWERTZ_KEYBOARD_LAYOUT,
            KeyboardLayout::Adaptive(_) => return None, // Use hybrid approach instead
            KeyboardLayout::Unknown => return None,
        };

        for (row_idx, row) in layout.iter().enumerate() {
            for (col_idx, &ch) in row.iter().enumerate() {
                if ch == c {
                    return Some((col_idx, row_idx));
                }
            }
        }
        None
    }
}

/// Convert a key code to a character
#[allow(dead_code)]
pub(super) fn code_to_char(code: &str) -> Option<char> {
    match code {
        "KeyA" => Some('a'),
        "KeyB" => Some('b'),
        "KeyC" => Some('c'),
        "KeyD" => Some('d'),
        "KeyE" => Some('e'),
        "KeyF" => Some('f'),
        "KeyG" => Some('g'),
        "KeyH" => Some('h'),
        "KeyI" => Some('i'),
        "KeyJ" => Some('j'),
        "KeyK" => Some('k'),
        "KeyL" => Some('l'),
        "KeyM" => Some('m'),
        "KeyN" => Some('n'),
        "KeyO" => Some('o'),
        "KeyP" => Some('p'),
        "KeyQ" => Some('q'),
        "KeyR" => Some('r'),
        "KeyS" => Some('s'),
        "KeyT" => Some('t'),
        "KeyU" => Some('u'),
        "KeyV" => Some('v'),
        "KeyW" => Some('w'),
        "KeyX" => Some('x'),
        "KeyY" => Some('y'),
        "KeyZ" => Some('z'),
        "Digit0" => Some('0'),
        "Digit1" => Some('1'),
        "Digit2" => Some('2'),
        "Digit3" => Some('3'),
        "Digit4" => Some('4'),
        "Digit5" => Some('5'),
        "Digit6" => Some('6'),
        "Digit7" => Some('7'),
        "Digit8" => Some('8'),
        "Digit9" => Some('9'),
        _ => None,
    }
}

impl PartialEq for AdaptiveKeyboard {
    fn eq(&self, other: &Self) -> bool {
        self.confusion_matrix == other.confusion_matrix
            && self.physical_mappings == other.physical_mappings
    }
}

// Keyboard layout definitions
static QWERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';'],
    ['z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static COLEMACK_DH_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['q', 'w', 'f', 'p', 'b', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'g', 'm', 'n', 'e', 'i', 'o'],
    ['x', 'c', 'd', 'v', 'z', 'k', 'h', ',', '.', '/'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static COLEMAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['q', 'w', 'f', 'p', 'g', 'j', 'l', 'u', 'y', ';'],
    ['a', 'r', 's', 't', 'd', 'h', 'n', 'e', 'i', 'o'],
    ['z', 'x', 'c', 'v', 'b', 'k', 'm', ',', '.', '/'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static DVORAK_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['\'', ',', '.', 'p', 'y', 'f', 'g', 'c', 'r', 'l'],
    ['a', 'o', 'e', 'u', 'i', 'd', 'h', 't', 'n', 's'],
    [';', 'q', 'j', 'k', 'x', 'b', 'm', 'w', 'v', 'z'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static WORKMAN_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['q', 'd', 'r', 'w', 'b', 'j', 'f', 'u', 'p', ';'],
    ['a', 's', 'h', 't', 'g', 'y', 'n', 'e', 'o', 'i'],
    ['z', 'x', 'm', 'c', 'v', 'k', 'l', ',', '.', '/'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static AZERTY_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    ['q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm'],
    ['w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '!'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

static QWERTZ_KEYBOARD_LAYOUT: [[char; 10]; 4] = [
    ['q', 'w', 'e', 'r', 't', 'z', 'u', 'i', 'o', 'p'],
    ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ö'],
    ['y', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-'],
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select::context::OptionState;
    use std::collections::HashMap;

    #[test]
    fn test_levenshtein_distance() {
        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b', 'c'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance < 0.01); // Very small but not exactly 0 due to recency bias

        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b', 'd'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        let typeahead = ['a', 'b'];
        let value = ['a', 'b', 'c'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        let typeahead = ['a', 'b', 'c'];
        let value = ['a', 'b'];
        let distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);
        assert!(distance > 0.0);

        // Test with keyboard-aware substitution costs
        let typeahead = ['q', 'w'];
        let value = ['q', 'e'];
        let qwerty_distance = levenshtein_distance(&typeahead, &value, |a, b| {
            KeyboardLayout::Qwerty.substitution_cost(a, b)
        });
        let uniform_distance = levenshtein_distance(&typeahead, &value, |_, _| 1.0);

        // 'w' and 'e' are adjacent on QWERTY, so should have lower cost than uniform
        assert!(qwerty_distance < uniform_distance);
    }

    #[test]
    fn test_normalized_distance() {
        let typeahead_chars = ['a', 'b', 'c'];
        let value_chars = ['a', 'b', 'c'];
        let keyboard_layout = KeyboardLayout::Qwerty;

        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard_layout);
        assert!(distance < 0.01); // Very small but not exactly 0 due to recency bias

        let typeahead_chars = ['a', 'b', 'c'];
        let value_chars = ['a', 'b', 'd'];
        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard_layout);
        assert!(distance > 0.0);

        // Test truncation behavior
        let typeahead_chars = ['a', 'b', 'c', 'd', 'e'];
        let value_chars = ['x', 'y', 'z'];
        let distance = normalized_distance(&typeahead_chars, &value_chars, &keyboard_layout);
        assert!(distance > 0.0);

        // Test with keyboard-aware costs
        let typeahead_chars = ['q'];
        let value_chars = ['w'];
        let qwerty_distance =
            normalized_distance(&typeahead_chars, &value_chars, &KeyboardLayout::Qwerty);

        let typeahead_chars = ['q'];
        let value_chars = ['p'];
        let far_distance =
            normalized_distance(&typeahead_chars, &value_chars, &KeyboardLayout::Qwerty);

        // Adjacent keys should have lower distance than distant keys
        assert!(qwerty_distance < far_distance);
    }

    #[test]
    fn test_detect_keyboard_layout() {
        // Test QWERTY detection
        let mut known_positions = HashMap::new();
        known_positions.insert('q', 'q');
        known_positions.insert('w', 'w');
        known_positions.insert('e', 'e');

        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::Qwerty);

        // Test with mostly QWERTY matches (layout detection is based on position matches)
        let mut known_positions = HashMap::new();
        known_positions.insert('q', 'q');
        known_positions.insert('w', 'w');
        known_positions.insert('e', 'e');
        known_positions.insert('r', 'r');

        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::Qwerty);

        // Test empty input
        let known_positions = HashMap::new();
        let detected = KeyboardLayout::guess(&known_positions);
        assert_eq!(detected, KeyboardLayout::Unknown);
    }

    #[test]
    fn test_keyboard_layout_substitution_cost() {
        let layout = KeyboardLayout::Qwerty;

        // Same character should have zero cost
        assert_eq!(layout.substitution_cost('a', 'a'), 0.0);

        // Adjacent characters should have lower cost than distant ones
        let adjacent_cost = layout.substitution_cost('q', 'w');
        let distant_cost = layout.substitution_cost('q', 'p');
        assert!(adjacent_cost < distant_cost);

        // Unknown characters should have cost of 1.0
        let unknown_cost = layout.substitution_cost('α', 'β');
        assert_eq!(unknown_cost, 1.0);
    }

    #[test]
    fn test_code_to_char() {
        assert_eq!(code_to_char("KeyA"), Some('a'));
        assert_eq!(code_to_char("KeyZ"), Some('z'));
        assert_eq!(code_to_char("Digit0"), Some('0'));
        assert_eq!(code_to_char("Digit9"), Some('9'));
        assert_eq!(code_to_char("Unknown"), None);
        assert_eq!(code_to_char(""), None);
    }

    #[test]
    fn test_best_match() {
        let options = vec![
            OptionState {
                tab_index: 0,
                value: "apple",
                text_value: "Apple".to_string(),
                id: "apple".to_string(),
            },
            OptionState {
                tab_index: 1,
                value: "banana",
                text_value: "Banana".to_string(),
                id: "banana".to_string(),
            },
            OptionState {
                tab_index: 2,
                value: "cherry",
                text_value: "Cherry".to_string(),
                id: "cherry".to_string(),
            },
        ];

        let layout = KeyboardLayout::Qwerty;

        // Exact prefix match
        let result = best_match(&layout, "App", &options);
        assert_eq!(result, Some(0));

        // Partial match
        let result = best_match(&layout, "ban", &options);
        assert_eq!(result, Some(1));

        // Empty typeahead should return None
        let result = best_match(&layout, "", &options);
        assert_eq!(result, None);

        // No match should return closest option
        let result = best_match(&layout, "xyz", &options);
        assert!(result.is_some());
    }

    #[test]
    fn test_recency_bias() {
        // Later characters should have higher bias (recency bias favors recent chars)
        let early_bias = recency_bias(0, 10);
        let late_bias = recency_bias(9, 10);
        assert!(late_bias > early_bias);

        // Single character should have maximum bias
        let single_bias = recency_bias(0, 1);
        assert!(single_bias > 0.0);
        assert!(single_bias <= 1.0);
    }

    #[test]
    fn test_adaptive_keyboard_learning() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Test learning from keyboard events
        adaptive.learn_from_event("KeyA", 'ф'); // Russian 'f' sound on A key
        adaptive.learn_from_event("KeyS", 'ы'); // Russian 'y' sound on S key

        // Should have learned the mappings
        assert_eq!(adaptive.physical_mappings.get("KeyA"), Some(&'ф'));
        assert_eq!(adaptive.physical_mappings.get("KeyS"), Some(&'ы'));
    }

    #[test]
    fn test_adaptive_keyboard_corrections() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Initially, substitution cost should be high
        let initial_cost = adaptive.substitution_cost('a', 'ф');
        assert!(initial_cost > 0.5);

        // Record multiple corrections
        adaptive.record_correction('a', 'ф');
        adaptive.record_correction('a', 'ф');
        adaptive.record_correction('a', 'ф');

        // Cost should be reduced
        let learned_cost = adaptive.substitution_cost('a', 'ф');
        assert!(learned_cost < initial_cost);
        assert!(learned_cost >= 0.1); // But not below minimum
    }

    #[test]
    fn test_unicode_similarity() {
        let adaptive = AdaptiveKeyboard::new();

        // Close Unicode codepoints should have low cost
        let close_cost = adaptive.unicode_similarity_cost('a', 'b'); // U+0061 vs U+0062
        let far_cost = adaptive.unicode_similarity_cost('a', '中'); // U+0061 vs U+4E2D

        assert!(close_cost < far_cost);
        assert!(close_cost >= 0.1);
        assert!(far_cost <= 1.0);

        // Same characters should have zero cost handled by main function
        assert_eq!(adaptive.substitution_cost('a', 'a'), 0.0);
    }

    #[test]
    fn test_phonetic_similarity() {
        let adaptive = AdaptiveKeyboard::new();

        // Test Latin and Cyrillic 'a' sounds
        let phonetic_cost = adaptive.phonetic_similarity_cost('a', 'а');
        assert_eq!(phonetic_cost, 0.2); // Should be low cost

        // Test Latin and Arabic 'b' sounds
        let phonetic_cost = adaptive.phonetic_similarity_cost('b', 'ب');
        assert_eq!(phonetic_cost, 0.2);

        // Test unrelated characters
        let unrelated_cost = adaptive.phonetic_similarity_cost('x', 'ж');
        assert_eq!(unrelated_cost, 1.0); // Should be high cost
    }

    #[test]
    fn test_physical_key_distance() {
        // Test adjacent keys
        let adjacent_distance = physical_key_distance("KeyA", "KeyS");
        assert!(adjacent_distance.is_some());
        assert!(adjacent_distance.unwrap() < 2.0);

        // Test distant keys
        let distant_distance = physical_key_distance("KeyA", "KeyP");
        assert!(distant_distance.is_some());
        assert!(distant_distance.unwrap() > adjacent_distance.unwrap());

        // Test same key
        let same_distance = physical_key_distance("KeyA", "KeyA");
        assert_eq!(same_distance, Some(0.0));

        // Test unknown key
        let unknown_distance = physical_key_distance("KeyA", "UnknownKey");
        assert!(unknown_distance.is_none());
    }

    #[test]
    fn test_physical_key_positions() {
        // Test some known positions
        assert_eq!(physical_key_position("KeyQ"), Some((0.0, 0.0)));
        assert_eq!(physical_key_position("KeyA"), Some((0.5, 1.0)));
        assert_eq!(physical_key_position("Digit1"), Some((0.0, -1.0)));

        // Test unknown key
        assert_eq!(physical_key_position("UnknownKey"), None);
    }

    #[test]
    fn test_hybrid_substitution_cost() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Set up some learned mappings and corrections
        adaptive.learn_from_event("KeyA", 'ф');
        adaptive.learn_from_event("KeyS", 'ы');
        adaptive.record_correction('a', 'ф');

        // Test learned cost takes priority
        let learned_cost = adaptive.substitution_cost('a', 'ф');
        assert!(learned_cost < 1.0);

        // Test physical key cost for mapped characters
        let physical_cost = adaptive.substitution_cost('ф', 'ы');
        assert!(physical_cost < 1.0); // Should use physical distance

        // Test phonetic similarity fallback
        let phonetic_cost = adaptive.substitution_cost('b', 'ب');
        assert_eq!(phonetic_cost, 0.2);

        // Test unicode similarity fallback
        let unicode_cost = adaptive.substitution_cost('x', 'y');
        assert!(unicode_cost < 1.0);
        assert!(unicode_cost >= 0.1);
    }

    #[test]
    fn test_multilingual_matching() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Russian
        adaptive.learn_from_event("KeyA", 'ф');
        adaptive.learn_from_event("KeyB", 'и');

        // Arabic
        adaptive.learn_from_event("KeyS", 'س');
        adaptive.learn_from_event("KeyT", 'ت');

        // Chinese characters that are far apart in Unicode get high cost
        let distant_chinese_cost = adaptive.substitution_cost('中', '国');
        assert_eq!(distant_chinese_cost, 1.0); // Distant characters should have max cost

        // But closer Chinese characters should have lower cost
        let close_chinese_cost = adaptive.substitution_cost('中', '丰'); // U+4E2D vs U+4E30
        assert!(close_chinese_cost < 1.0); // Close Unicode characters should work

        // Mixed script matching - test f/ф which are phonetically similar
        let mixed_cost = adaptive.substitution_cost('f', 'ф');
        assert!(mixed_cost < 1.0); // Should work through phonetic similarity
    }

    #[test]
    fn test_adaptive_keyboard_layout_enum() {
        let mut adaptive = AdaptiveKeyboard::new();
        adaptive.record_correction('q', 'й'); // Russian

        let layout = KeyboardLayout::Adaptive(adaptive);

        // Test that adaptive layout uses hybrid approach
        let cost = layout.substitution_cost('q', 'й');
        assert!(cost < 1.0);
        assert!(cost >= 0.1);
    }

    #[test]
    fn test_best_match_with_adaptive() {
        let mut adaptive = AdaptiveKeyboard::new();

        // Learn some Cyrillic mappings
        adaptive.record_correction('a', 'а');
        adaptive.record_correction('p', 'р');

        let layout = KeyboardLayout::Adaptive(adaptive);

        let options = vec![
            OptionState {
                tab_index: 0,
                value: "apple",
                text_value: "Apple".to_string(),
                id: "apple".to_string(),
            },
            OptionState {
                tab_index: 1,
                value: "арple", // Mixed Cyrillic/Latin
                text_value: "Арple".to_string(),
                id: "arple".to_string(),
            },
        ];

        // Typing with mixed scripts should still find matches
        let result = best_match(&layout, "аp", &options);
        assert!(result.is_some());
    }
}

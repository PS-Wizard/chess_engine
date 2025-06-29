# -> Chess Engine
---

> ### Step 1: Board & Peice Representation

* [x] Bit Boards
* [x] Peice Representation
* [x] Ability to move a peice some place else


```rust
~

#![allow(dead_code)]


pub fn notation_to_idx(notation: &str) -> Option<usize> {
    let mut chars = notation.chars();
    let file_char = chars.next()?; // 'e'
    let rank_char = chars.next()?; // '2'

    let file = (file_char as u8).checked_sub(b'a')?;
    let rank = (rank_char as u8).checked_sub(b'1')?;

    if file > 7 || rank > 7 {
        return None;
    }

    Some((rank as usize) * 8 + (file as usize))
}

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_notation_to_idx() {
        assert_eq!(notation_to_idx("a1"), Some(0));
        assert_eq!(notation_to_idx("h8"), Some(63));
        assert_eq!(notation_to_idx("e2"), Some(12));
    }
}
```

> ## Step 2: Move Generation
* [ ] Generate Moves For:
    * [ ] Pawns
        * [ ] Enpassant
    * [ ] Knights
    * [ ] Bishops
    * [ ] Rooks
    * [ ] Queen
    * [ ] King
        * [ ] Castling
* [ ] Filter Out Valid Moves

#### Step 3: Make / Unmake Moves

#### Step 4: Evaluation Function

#### Step 5: Search

#### Step 6: UCI Protocol



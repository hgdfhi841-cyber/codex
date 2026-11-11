// Modified by the unofficial codex-ar fork (2026): Arabic localization. See CODEX-AR.md.
//
// Right-to-left display for terminals without bidi support (for example Windows Terminal).
//
// The terminal draws cells strictly left to right. To make Arabic readable we do, per run of text:
//  1. shaping: replace each Arabic letter with its contextual presentation form (isolated / final /
//     initial / medial, Unicode block "Arabic Presentation Forms-B"), so no terminal shaping is
//     needed;
//  2. reordering: a simplified Unicode bidi pass (paragraph direction from the first strong
//     character, neutrals resolved from their neighbours, numbers and Latin kept left-to-right)
//     that produces visual order;
//  3. mirroring of brackets inside right-to-left runs.
//
// Everything works on "cells" (one grapheme each, as ratatui stores them), so styles move with
// their text and the width of every row is unchanged.

/// Joining behaviour of a letter and the first code point of its presentation forms.
/// `forms` is 1 (non-joining), 2 (joins to the previous letter only) or 4 (joins both ways).
/// Forms are stored in the order isolated, final, initial, medial.
struct Letter {
    base: u32,
    forms: u8,
}

const fn l(base: u32, forms: u8) -> Letter {
    Letter { base, forms }
}

/// Indexed by `ch - 0x0621` for U+0621..=U+064A. `None` for code points that are not letters.
const LETTERS: [Option<Letter>; 42] = [
    Some(l(0xFE80, 1)), // 0621 HAMZA
    Some(l(0xFE81, 2)), // 0622 ALEF WITH MADDA ABOVE
    Some(l(0xFE83, 2)), // 0623 ALEF WITH HAMZA ABOVE
    Some(l(0xFE85, 2)), // 0624 WAW WITH HAMZA ABOVE
    Some(l(0xFE87, 2)), // 0625 ALEF WITH HAMZA BELOW
    Some(l(0xFE89, 4)), // 0626 YEH WITH HAMZA ABOVE
    Some(l(0xFE8D, 2)), // 0627 ALEF
    Some(l(0xFE8F, 4)), // 0628 BEH
    Some(l(0xFE93, 2)), // 0629 TEH MARBUTA
    Some(l(0xFE95, 4)), // 062A TEH
    Some(l(0xFE99, 4)), // 062B THEH
    Some(l(0xFE9D, 4)), // 062C JEEM
    Some(l(0xFEA1, 4)), // 062D HAH
    Some(l(0xFEA5, 4)), // 062E KHAH
    Some(l(0xFEA9, 2)), // 062F DAL
    Some(l(0xFEAB, 2)), // 0630 THAL
    Some(l(0xFEAD, 2)), // 0631 REH
    Some(l(0xFEAF, 2)), // 0632 ZAIN
    Some(l(0xFEB1, 4)), // 0633 SEEN
    Some(l(0xFEB5, 4)), // 0634 SHEEN
    Some(l(0xFEB9, 4)), // 0635 SAD
    Some(l(0xFEBD, 4)), // 0636 DAD
    Some(l(0xFEC1, 4)), // 0637 TAH
    Some(l(0xFEC5, 4)), // 0638 ZAH
    Some(l(0xFEC9, 4)), // 0639 AIN
    Some(l(0xFECD, 4)), // 063A GHAIN
    None,               // 063B
    None,               // 063C
    None,               // 063D
    None,               // 063E
    None,               // 063F
    None,               // 0640 TATWEEL (handled separately: joins both ways, no forms)
    Some(l(0xFED1, 4)), // 0641 FEH
    Some(l(0xFED5, 4)), // 0642 QAF
    Some(l(0xFED9, 4)), // 0643 KAF
    Some(l(0xFEDD, 4)), // 0644 LAM
    Some(l(0xFEE1, 4)), // 0645 MEEM
    Some(l(0xFEE5, 4)), // 0646 NOON
    Some(l(0xFEE9, 4)), // 0647 HEH
    Some(l(0xFEED, 2)), // 0648 WAW
    Some(l(0xFEEF, 2)), // 0649 ALEF MAKSURA
    Some(l(0xFEF1, 4)), // 064A YEH
];

const TATWEEL: char = '\u{0640}';

fn letter(ch: char) -> Option<&'static Letter> {
    let c = ch as u32;
    if (0x0621..=0x064A).contains(&c) {
        LETTERS[(c - 0x0621) as usize].as_ref()
    } else {
        None
    }
}

/// True for code points written right to left (Arabic and Hebrew blocks, presentation forms).
fn is_rtl_char(ch: char) -> bool {
    matches!(ch as u32,
        0x0590..=0x05FF   // Hebrew
        | 0x0600..=0x065F // Arabic (letters, punctuation, harakat)
        | 0x066A..=0x06EF // Arabic extended letters (digits 0660-0669 / 06F0-06F9 excluded)
        | 0x06FA..=0x06FF
        | 0x0750..=0x077F
        | 0x08A0..=0x08FF
        | 0xFB1D..=0xFDFF // Hebrew / Arabic presentation forms A
        | 0xFE70..=0xFEFF // Arabic presentation forms B
    )
}

fn is_strong_ltr_char(ch: char) -> bool {
    ch.is_alphanumeric() && !is_rtl_char(ch)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    L,
    R,
    N,
}

fn cell_dir(cell: &str) -> Dir {
    match cell.chars().next() {
        Some(ch) if is_rtl_char(ch) => Dir::R,
        Some(ch) if is_strong_ltr_char(ch) => Dir::L,
        // Arabic-Indic digits read left to right like other numbers.
        Some(ch) if ('\u{0660}'..='\u{0669}').contains(&ch) || ('\u{06F0}'..='\u{06F9}').contains(&ch) => Dir::L,
        _ => Dir::N,
    }
}

/// True when the cell contains Arabic or Hebrew text that needs processing.
pub(crate) fn needs_processing(cell: &str) -> bool {
    cell.chars().any(is_rtl_char)
}

/// Can the letter in this cell connect to the letter that follows it?
fn joins_forward(cell: &str) -> bool {
    match cell.chars().next() {
        Some(TATWEEL) => true,
        Some(ch) => letter(ch).is_some_and(|l| l.forms == 4),
        None => false,
    }
}

/// Can the letter in this cell connect to the letter before it?
fn joins_backward(cell: &str) -> bool {
    match cell.chars().next() {
        Some(TATWEEL) => true,
        Some(ch) => letter(ch).is_some_and(|l| l.forms >= 2),
        None => false,
    }
}

/// Replace the first code point of `cell` (an Arabic letter) with its contextual form.
fn shape_cell(cell: &str, prev_joins: bool, next_joins: bool) -> Option<String> {
    let mut chars = cell.chars();
    let first = chars.next()?;
    let lt = letter(first)?;
    let offset = match (lt.forms, prev_joins, next_joins) {
        (1, _, _) => 0,
        (2, true, _) => 1,
        (2, false, _) => 0,
        (4, true, true) => 3,
        (4, false, true) => 2,
        (4, true, false) => 1,
        _ => 0,
    };
    let shaped = char::from_u32(lt.base + offset)?;
    let mut out = String::with_capacity(cell.len());
    out.push(shaped);
    out.extend(chars);
    Some(out)
}

fn mirror(cell: &str) -> Option<&'static str> {
    Some(match cell {
        "(" => ")",
        ")" => "(",
        "[" => "]",
        "]" => "[",
        "{" => "}",
        "}" => "{",
        "<" => ">",
        ">" => "<",
        "«" => "»",
        "»" => "«",
        _ => return None,
    })
}

/// Result of processing one run of cells.
pub(crate) struct Reordered {
    /// `order[visual_index] = logical_index`.
    pub order: Vec<usize>,
    /// Replacement symbol per logical cell (shaped letter or mirrored bracket), if changed.
    pub symbols: Vec<Option<String>>,
    /// The run reads right to left (its first strong character is Arabic or Hebrew).
    pub rtl: bool,
}

/// Shape and reorder one run of cells given in logical order.
/// Returns `None` when the run has no right-to-left text (nothing to do).
pub(crate) fn reorder_run(cells: &[&str]) -> Option<Reordered> {
    if !cells.iter().any(|c| needs_processing(c)) {
        return None;
    }
    let n = cells.len();

    // 1. Shaping (logical order).
    let mut symbols: Vec<Option<String>> = vec![None; n];
    for i in 0..n {
        let first = cells[i].chars().next();
        if first.and_then(letter).is_none() {
            continue;
        }
        let prev = i > 0 && joins_forward(cells[i - 1]);
        let next = i + 1 < n && joins_backward(cells[i + 1]);
        symbols[i] = shape_cell(cells[i], prev, next);
    }

    // 2. Directions and levels.
    let mut dirs: Vec<Dir> = cells.iter().map(|c| cell_dir(c)).collect();
    // Number punctuation follows the number it belongs to ("25%", "$5", "1.5", "10:30").
    let is_digit = |c: &str| {
        c.chars().next().is_some_and(|ch| {
            ch.is_ascii_digit() || ('\u{0660}'..='\u{0669}').contains(&ch) || ('\u{06F0}'..='\u{06F9}').contains(&ch)
        })
    };
    let terminator = |c: &str| matches!(c, "%" | "$" | "€" | "+" | "-" | "#" | "°" | "‰" | "٪");
    let separator = |c: &str| matches!(c, "." | "," | ":" | "/" | "٫" | "٬");
    for i in 0..n {
        if dirs[i] != Dir::N {
            continue;
        }
        let prev_digit = i > 0 && is_digit(cells[i - 1]);
        let next_digit = i + 1 < n && is_digit(cells[i + 1]);
        if (terminator(cells[i]) && (prev_digit || next_digit))
            || (separator(cells[i]) && prev_digit && next_digit)
        {
            dirs[i] = Dir::L;
        }
    }
    let para_rtl = dirs.iter().find(|d| **d != Dir::N).is_some_and(|d| *d == Dir::R);
    let base = if para_rtl { Dir::R } else { Dir::L };
    let mut resolved = dirs.clone();
    let mut i = 0;
    while i < n {
        if resolved[i] != Dir::N {
            i += 1;
            continue;
        }
        let start = i;
        while i < n && resolved[i] == Dir::N {
            i += 1;
        }
        let before = if start == 0 { base } else { resolved[start - 1] };
        let after = if i == n { base } else { resolved[i] };
        let d = if before == after { before } else { base };
        for r in resolved.iter_mut().take(i).skip(start) {
            *r = d;
        }
    }
    let levels: Vec<u8> = resolved
        .iter()
        .map(|d| match (para_rtl, d) {
            (false, Dir::R) => 1,
            (false, _) => 0,
            (true, Dir::L) => 2,
            (true, _) => 1,
        })
        .collect();

    // 3. Mirroring inside right-to-left runs.
    for i in 0..n {
        if levels[i] % 2 == 1
            && let Some(m) = mirror(cells[i])
        {
            symbols[i] = Some(m.to_string());
        }
    }

    // 4. Reorder: reverse every maximal run at or above each level, highest level first.
    let mut order: Vec<usize> = (0..n).collect();
    let max_level = levels.iter().copied().max().unwrap_or(0);
    for level in (1..=max_level).rev() {
        let mut j = 0;
        while j < n {
            if levels[order[j]] < level {
                j += 1;
                continue;
            }
            let start = j;
            while j < n && levels[order[j]] >= level {
                j += 1;
            }
            order[start..j].reverse();
        }
    }
    Some(Reordered {
        order,
        symbols,
        rtl: para_rtl,
    })
}

/// Split a row of cells into runs that are processed independently: runs are separated by box
/// drawing characters and by gaps of two or more spaces, so frames and columns stay in place.
/// Returns half-open `(start, end)` ranges that exclude leading and trailing spaces.
pub(crate) fn row_runs(cells: &[&str]) -> Vec<(usize, usize)> {
    let is_space = |c: &str| c.is_empty() || c == " ";
    let is_frame = |c: &str| {
        c.chars().next().is_some_and(|ch| ('\u{2500}'..='\u{259F}').contains(&ch))
    };
    let mut runs = Vec::new();
    let n = cells.len();
    let mut i = 0;
    while i < n {
        while i < n && (is_space(cells[i]) || is_frame(cells[i])) {
            i += 1;
        }
        if i >= n {
            break;
        }
        let start = i;
        let mut last_text = i;
        while i < n && !is_frame(cells[i]) {
            if is_space(cells[i]) {
                let gap_start = i;
                while i < n && is_space(cells[i]) {
                    i += 1;
                }
                if i - gap_start >= 2 || i >= n {
                    break;
                }
                continue;
            }
            last_text = i;
            i += 1;
        }
        runs.push((start, last_text + 1));
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(s: &str) -> Vec<String> {
        s.chars().map(|c| c.to_string()).collect()
    }

    fn visual(s: &str) -> String {
        let owned = cells(s);
        let refs: Vec<&str> = owned.iter().map(String::as_str).collect();
        match reorder_run(&refs) {
            None => s.to_string(),
            Some(r) => r
                .order
                .iter()
                .map(|&i| r.symbols[i].clone().unwrap_or_else(|| owned[i].clone()))
                .collect(),
        }
    }

    #[test]
    fn latin_is_untouched() {
        assert_eq!(visual("hello world"), "hello world");
    }

    #[test]
    fn arabic_word_is_shaped_and_reversed() {
        // "سلام": seen initial, lam medial, alef final, meem isolated (alef does not join forward).
        let v: Vec<char> = visual("سلام").chars().collect();
        assert_eq!(v, vec!['\u{FEE1}', '\u{FE8E}', '\u{FEE0}', '\u{FEB3}']);
    }

    #[test]
    fn numbers_and_latin_keep_their_order_inside_arabic() {
        // "متبقٍ 25%" -> the number stays "25%" and sits to the left of the Arabic word.
        let v = visual("ابدأ codex الآن");
        assert!(v.contains("codex"), "{v}");
        let v = visual("بقي 25% فقط");
        assert!(v.contains("25%"), "{v}");
    }

    #[test]
    fn brackets_are_mirrored_in_arabic() {
        let v = visual("(نص)");
        assert!(v.starts_with('(') && v.ends_with(')'), "{v}");
    }

    #[test]
    fn runs_split_on_frames_and_wide_gaps() {
        let owned = cells("│ abc  مرحبا │");
        let refs: Vec<&str> = owned.iter().map(String::as_str).collect();
        assert_eq!(row_runs(&refs), vec![(2, 5), (7, 12)]);
    }
}

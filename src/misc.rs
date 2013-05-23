//! Various utility functions.
//!
//! Clients should not need to use these.

use core::str::CharRange;

pub static EOT: char = '\u0003';

pub fn is_print(ch: char) -> bool {
    ch >= ' ' && ch <= '~'
}

pub fn at_connect(v: &[@~str], sep: &str) -> ~str {
    let mut s = ~"", first = true;
    for vec::each(v) |ss| {
        if first {
            first = false;
        } else { unsafe { str::push_str(&mut s, sep); } }
        unsafe { str::push_str(&mut s, **ss) };
    }
    return s;
}

/// Converts a string to an array of char and appends an EOT character.
pub fn chars_with_eot(s: &str) -> @[char] {
    do at_vec::build_sized(s.len() + 1) |push| {
        let mut i = 0u;
        let len = str::len(s);
        while i < len {
            let CharRange{ch: ch, next: next} = str::char_range_at(s, i);
            assert!(next > i);
            push(ch);
            i = next;
        }
        push(EOT);
    }
}

/// Returns ch as lower case.
pub fn lower_char(ch: char) -> char {
    if ch >= 'A' && ch <= 'Z' {
        ('a' as uint + (ch as uint - 'A' as uint)) as char
    } else { ch }
}

/// Returns a string with count ch characters.
pub fn repeat_char(ch: char, count: uint) -> ~str {
    let mut value = ~"";
    str::reserve(&mut value, count);
    for uint::range(0u, count) |_i| { str::push_char(&mut value, ch); }
    return value;
}

#[doc(hidden)]
pub fn get_col(text: @[char], index: uint) -> uint {
    let mut i = index;
    while i > 0u && text[i - 1u] != '\n' && text[i - 1u] != '\r' { i -= 1u; }
    return index - i + 1u;
}

// Note that we don't want to escape control characters here because we need
// one code point to map to one printed character (so our log_ok arrows point to
// the right character).

/// Replaces non-is_print characters with '.'."
pub fn munge_chars(chars: @[char]) -> ~str {
    // TODO: I'd like to use bullet here, but while io::println handles it correctly
    // the logging subsystem does not. See issue 2154.
    //let bullet = '\u2022';
    let bullet = '.';
    let mut value = ~"";
    str::reserve(&mut value, vec::len(chars));
    for vec::each(chars) |ch| {
        str::push_char(&mut value, if is_print(*ch) { *ch } else { bullet });
    }
    return value;
}

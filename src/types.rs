//! Types used by all parse functions.

// TODO: should be able to get rid of all the Durable bounds once
// https://github.com/mozilla/rust/issues/2992 is fixed

/// Type for parse functions.
pub type Parser<T> = @fn (State) -> Status<T>;

/// Input argument for parse functions. File is not interpreted and need 
/// not be a path. Text is assumed to end with EOT. Lines are 1-based.
pub struct State {file: @~str, text: @[char], index: uint, line: int}

/// Return type of parse functions.
pub type Status<T> = Result<Succeeded<T>, Failed>;

/// new_state will be like the input state except that index and line may 
/// advance. Value is an arbitrary value associated with the parse.
pub struct Succeeded<T> {new_state: State, value: T}

/// old_state should be identical to the input state. err_state is where 
/// the error happened.
pub struct Failed {old_state: State, err_state: State, mesg: @~str}

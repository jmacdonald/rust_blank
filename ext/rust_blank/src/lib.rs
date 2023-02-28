use magnus::{
    class,
    define_class,
    encoding::{CType, RbEncoding},
    method,
    prelude::*,
    Error, RString,
};

fn is_blank(rb_self: RString) -> Result<bool, Error> {
    // RString::as_str is unsafe as it's possible for Ruby to invalidate the
    // str as we hold a reference to it, but here we're only ever using the
    // &str before Ruby is invoked again, so it doesn't get a chance to mess
    // with it and this is safe.
    unsafe {
        // fast path, string is valid utf-8 and we can use Rust's stdlib
        if let Some(s) = rb_self.test_as_str() {
            return Ok(!s.contains(|c: char| !c.is_whitespace()));
        }
    }

    // slow path, use Ruby's API to iterate the codepoints and check for blanks
    let enc = RbEncoding::from(rb_self.enc_get());
    // Similar to ::as_str above, RString::codepoints holds a reference to the
    // underlying string data and we can't let Ruby mutate or invalidate the
    // string while we hold a reference to the codepoints iterator. Here we
    // don't invoke any Ruby methods that could modify the string, so this is
    // safe.
    unsafe {
        for cp in rb_self.codepoints() {
            if !enc.is_code_ctype(cp?, CType::Blank) {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let class = define_class("String", class::object())?;
    class.define_method("blank?", method!(is_blank, 0))?;
    Ok(())
}

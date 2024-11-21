use super::base;

#[inline]
pub fn ok<S: AsRef<str>>(msg: S) -> String {
    base::ok(msg)
}

#[inline]
pub fn error<S: AsRef<str>>(msg: S) -> String {
    base::error(msg)
}

#[inline]
pub fn info<S: AsRef<str>>(msg: S) -> String {
    base::info(msg)
}

#[inline]
pub fn debug<S: AsRef<str>>(msg: S) -> String {
    base::debug(msg)
}

#[inline]
pub fn warn<S: AsRef<str>>(msg: S) -> String {
    base::warn(msg)
}

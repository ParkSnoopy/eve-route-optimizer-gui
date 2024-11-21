use nu_ansi_term::Color;



#[inline]
fn paint_on_debug<S: AsRef<str>>(msg: S, code: u8) -> String {
    match crate::config::DEBUG {
        false => msg.as_ref().to_string(),
        true => {
            Color::Fixed(code).paint(msg.as_ref()).to_string()
        }
    }
}

#[inline]
pub fn ok(msg: impl AsRef<str>) -> String {
    format!("{} {}",
        paint_on_debug("[  O K  ]", 070),
        msg.as_ref(),
    )
}

#[inline]
pub fn error(msg: impl AsRef<str>) -> String {
    format!("{} {}",
        paint_on_debug("[  ERR  ]", 160),
        msg.as_ref(),
    )
}

#[inline]
pub fn info(msg: impl AsRef<str>) -> String {
    format!("{} {}",
        paint_on_debug("[  INF  ]", 039),
        msg.as_ref(),
    )
}

#[inline]
pub fn debug(msg: impl AsRef<str>) -> String {
    format!("{} {}",
        paint_on_debug("[  DBG  ]", 190),
        msg.as_ref(),
    )
}

#[inline]
pub fn warn(msg: impl AsRef<str>) -> String {
    format!("{} {}",
        paint_on_debug("[  WRN  ]", 199),
        msg.as_ref(),
    )
}

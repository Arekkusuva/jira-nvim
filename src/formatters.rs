use std::fmt::Write;

use chrono::{DateTime, Utc};

pub fn diff_ago(dt: DateTime<Utc>) -> String {
    let diff = Utc::now().signed_duration_since(dt);
    let diff_days = diff.num_days();
    let diff_hours = diff.num_hours();
    let diff_seconds = diff.num_seconds();

    if diff_days != 0 {
        format!("{} days ago", diff_days)
    } else if diff_hours != 0 {
        format!("{} hours ago", diff_hours)
    } else if diff_seconds > 0 {
        format!("{} seconds ago", diff_seconds)
    } else {
        "less than second ago".into()
    }
}

pub fn list(list: &Vec<String>) -> String {
    if list.is_empty() {
        return "()".into();
    }

    let mut len = 0;
    for item in list {
        len += item.len()
    }
    // `2` for parentheses, `list.len() - 1` for commas
    len = 2 + len + (list.len() - 1);

    let mut buf = String::with_capacity(len);
    buf.write_char('(').unwrap();
    for item in list {
        buf.write_str(item).unwrap();
        buf.write_char(',').unwrap();
    }
    // SAFETY: It is safe to replace one ASCII character with another
    unsafe { buf.as_bytes_mut()[len - 1] = ')' as u8 }
    buf
}

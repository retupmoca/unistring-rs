#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(temporary_cstring_as_ptr)]

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub trait UnicodeStr {
    fn width(&self) -> i32;
    fn wrap_to_width(&self, width: i32) -> Vec<&str>;
    fn cut_to_width(&self, width: i32) -> (&str, &str);
    fn cut_to_width_hard(&self, width: i32) -> (&str, &str);
}

use std::ffi::CString;
impl UnicodeStr for str {
    fn width(&self) -> i32 {
        unsafe {
            sys::u8_width(
                self.as_bytes().as_ptr(),
                self.len() as u64,
                CString::new("UTF-8").unwrap().as_ptr()
            )
        }
    }

    fn wrap_to_width(&self, width: i32) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let mut cur = self;
        loop {
            if cur.width() <= width {
                result.push(cur);
                return result;
            }
            let (line, rest) = cur.cut_to_width(width);
            cur = rest;
            result.push(line);
        };
    }

    fn cut_to_width(&self, width: i32) -> (&str, &str) {
        if self.width() <= width {
            return (self, "");
        }

        let mut bp: Vec<i8> = vec![0; self.len()];
        unsafe {
            sys::u8_wordbreaks(
                self.as_bytes().as_ptr(),
                self.len() as u64,
                bp.as_mut_ptr()
            );
        }
        // start at 2: we don't want a single space at the beginning of a line
        // to be picked
        for i in (2..self.len()).rev() {
            if bp[i] != 0 && self[..i].width() <= width {
                return (&self[..i], &self[i..]);
            }
        }

        self.cut_to_width_hard(width)
    }

    fn cut_to_width_hard(&self, width: i32) -> (&str, &str) {
        if self.width() <= width {
            return (self, "");
        }

        let mut last = 0;
        let mut current = 0;
        while current < self.len() {
            let next;
            unsafe {
                next = sys::u8_mblen(
                    self[current..].as_bytes().as_ptr(), 
                    (self.len() - current) as u64
                ) as usize;
            }
            last = current;
            current += next;
            if self[..current].width() > width {
                break;
            }
        }

        (&self[..last], &self[last..])
    }
}

#[cfg(test)]
mod tests {
    use super::UnicodeStr;

    #[test]
    fn width() {
        let s1 = "Hello";
        assert_eq!(s1.width(), 5);
    }

    #[test]
    fn cut_to_width_hard() {
        let s1 = "Hello";
        let (a, b) = s1.cut_to_width_hard(3);
        assert_eq!(a, "Hel");
        assert_eq!(b, "lo");
    }

    #[test]
    fn cut_to_width() {
        let s1 = "Hello World";
        let (a, b) = s1.cut_to_width(3);
        assert_eq!(a, "Hel");
        assert_eq!(b, "lo World");

        let (a, b) = s1.cut_to_width(10);
        assert_eq!(a, "Hello ");
        assert_eq!(b, "World");
    }

    #[test]
    fn wrap_to_width() {
        let s1 = "Hello There, World";
        let result = s1.wrap_to_width(3);
        assert_eq!(result, vec!["Hel", "lo ", "The", "re,", " Wo", "rld"]);

        let result = s1.wrap_to_width(10);
        assert_eq!(result, vec!["Hello ", "There, ", "World"]);
    }
}

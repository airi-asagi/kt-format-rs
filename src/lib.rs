pub mod kt {
    pub use proc::*;
}

#[cfg(test)]
mod test {
    use crate::kt;
    #[test]
    fn it_works() {
        assert_eq!(format!("{{{{{}", 1), r"{{1");
        assert_eq!(kt::format!("{{${}", 1), r"{{1");
        assert_eq!(kt::format!($$ "{{$${}", 1), r"{{1");
        assert_eq!(kt::format!($$ "${${$${}", 1), r"${${1");

        assert_eq!(
            format!(r"\${{{}}}\{{\{{([^{{}}]*?)\}}\}}", 3),
            r"\${3}\{\{([^{}]*?)\}\}"
        );
        assert_eq!(
            kt::format!($$ r"\${$${}}\{\{([^{}]*?)\}\}", 3),
            r"\${3}\{\{([^{}]*?)\}\}"
        );

        assert_eq!(format!("hex = {:#x}", 255), "hex = 0xff");
        assert_eq!(kt::format!("hex = ${:#x}", 255), "hex = 0xff");

        assert_eq!(format!("value = {value}", value = 42), "value = 42");
        assert_eq!(kt::format!("value = ${value}", value = 42), "value = 42");
    }
}

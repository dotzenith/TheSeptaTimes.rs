use  crate::ScheduleMode;

pub trait Parse {
    fn parse(&self) -> Vec<String>;
}

pub trait ParseWithMode {
    fn parse(&self, mode: &ScheduleMode) -> Vec<String>;
}

pub trait PrettyPrint {
    fn print(&self)
    where
        Self: Sized;
}

pub trait PrettyPrintWithMode {
    fn print(&self, mode: &ScheduleMode)
    where
        Self: Sized;
}

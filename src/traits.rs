pub trait Parse {
    fn parse(&self) -> Vec<String>;
}

pub trait PrettyPrint {
    fn print(&self)
    where
        Self: Sized;
}

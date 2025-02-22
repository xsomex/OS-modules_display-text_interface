#![no_std]

pub trait DisplayText
where
    Self: core::fmt::Write + Default,
{
    fn set_foreground_color(&mut self, color: (u8, u8, u8));
    fn set_background_color(&mut self, color: (u8, u8, u8));
    fn fill(&mut self, color: (u8, u8, u8));
}

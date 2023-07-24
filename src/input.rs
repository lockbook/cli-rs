/// parser complexities:
///
/// compound flags (-am)
///
/// optional flags
///
/// out of order flags
pub trait Input {
    fn parse(&mut self, args: &[String], offset: usize) -> usize;
    fn display_name(&self) -> String;
    fn type_name(&self) -> String;
}

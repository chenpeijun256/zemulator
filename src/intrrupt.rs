
#[derive(Clone, Copy)]
pub enum IntrType {
    None,
    IntrUserSoft(u32),
    IntrByExt(u32),
    ExceInstruction(u32),
    ExceMem(u32),
}

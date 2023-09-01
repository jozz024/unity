#[repr(C)]
#[crate::class("App", "Mess")]
pub struct AppMess;

#[crate::from_offset("App", "ScriptSystem", "Log")]
pub fn scriptsystem_log(args: *const u8);

#[crate::from_offset("App", "MainMenuSequence", "JumpToNextSequence")]
pub fn mainmenusequence_jumptonextsequence(this: *const u8);

// 此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

mod base;
mod check;
mod crates;
mod status;
mod util;

pub use check::check_architecture;
pub use status::{CheckStatus, finish};

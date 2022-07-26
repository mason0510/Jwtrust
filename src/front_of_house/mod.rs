mod add;

// 这些是重导出函数
pub use add::add;

const DEBUG: bool = true;

//如果在A文件中使用B
// super::add

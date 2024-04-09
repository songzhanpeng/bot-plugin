const { execFile } = require('child_process');

// Rust 可执行文件路径
const rustExecutable = './target/release/bot-plugin';

// 连击数量
const comboCount = 5; // 你可以根据需要设置连击的数量

// 执行 Rust 可执行文件
execFile(rustExecutable, [comboCount.toString()], (error, stdout, stderr) => {
  if (error) {
    console.error('执行 Rust 程序时出错：', error);
    return;
  }
  // 输出 Rust 程序的结果
  console.log('Rust 程序的输出：', stdout);
});

//FOR PM2

const { spawn } = require('child_process');

// Path to your Rust program's binary - for PM2
const rustProgramPath = '/home/rocky/rust-projects/discord-bots/bot01/target/debug/bot01';

// Start the Rust program as a child process 
const rustProcess = spawn(rustProgramPath, [], { stdio: 'inherit' });

rustProcess.on('exit', (code) => {
  console.log(`Rust program exited with code ${code}`);
});
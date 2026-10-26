import { spawn } from "node:child_process";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cargoBin = path.join(os.homedir(), ".cargo", "bin");
const npmBin = path.join(root, "node_modules", ".bin");

process.env.PATH = [cargoBin, npmBin, process.env.PATH ?? ""].join(path.delimiter);

const tauriBin =
  process.platform === "win32"
    ? path.join(npmBin, "tauri.cmd")
    : path.join(npmBin, "tauri");

const args = process.argv.slice(2);
const child = spawn(tauriBin, args, {
  stdio: "inherit",
  cwd: root,
  env: process.env,
  shell: process.platform === "win32",
});

child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 0);
});

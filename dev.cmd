@echo off
setlocal
cd /d "%~dp0"

rem Ensure rustup cargo is on PATH (needed if the terminal was opened before installing Rust)
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

where cargo >nul 2>&1
if errorlevel 1 (
  echo ERROR: cargo not found. Install Rust from https://rust-lang.org/tools/install
  echo Then close and reopen this terminal.
  exit /b 1
)

call npm run tauri -- dev
exit /b %ERRORLEVEL%

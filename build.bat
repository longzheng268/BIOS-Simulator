@echo off
REM ============================================
REM  BIOS Simulator - Windows Build Script
REM ============================================
REM  Usage: build.bat
REM  Output: bios-simulator.exe
REM ============================================

echo.
echo === BIOS Simulator Build ===
echo.

REM Check if Rust is installed
where cargo >nul 2>&1
if errorlevel 1 (
    echo ERROR: cargo not found. Please install Rust:
    echo   https://rustup.rs
    echo.
    pause
    exit /b 1
)

echo [1/2] Building release...
cargo build --release
if errorlevel 1 (
    echo.
    echo ERROR: Build failed.
    pause
    exit /b 1
)

echo.
echo [2/2] Copying assets...
if not exist "dist" mkdir dist
copy /Y "target\release\bios-simulator.exe" "dist\" >nul
xcopy /E /I /Y "assets" "dist\assets" >nul
xcopy /E /I /Y "audio_gen\scripts" "dist\audio_gen\scripts" >nul
xcopy /E /I /Y "audio_gen\output" "dist\audio_gen\output" >nul

echo.
echo ============================================
echo  Build complete!
echo.
echo  Run the game:
echo    dist\bios-simulator.exe
echo.
echo  Or run directly:
echo    cargo run --release
echo ============================================
echo.
pause

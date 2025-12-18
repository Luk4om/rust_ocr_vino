@echo off
setlocal

:: --- 1. ตั้งค่า Path ของ OpenVINO ---
:: แก้ไขบรรทัดข้างล่างนี้ให้ตรงกับโฟลเดอร์ที่คุณแตกไฟล์ OpenVINO ไว้
:: ตัวอย่าง: set "OPENVINO_INSTALL_DIR=C:\Users\Luk4om\Downloads\openvino_2025.4"
set "OPENVINO_INSTALL_DIR=C:\Program Files (x86)\Intel\openvino"

:: ตรวจสอบว่ามีโฟลเดอร์อยู่จริงไหม
if not exist "%OPENVINO_INSTALL_DIR%" (
    echo [ERROR] Could not find OpenVINO directory at: %OPENVINO_INSTALL_DIR%
    echo.
    echo Please edit this file (run_ocr.bat) and change OPENVINO_INSTALL_DIR
    echo to match where you extracted the OpenVINO zip file.
    echo.
    pause
    exit /b 1
)

:: --- 2. ตั้งค่า System Path ---
echo Setting environment variables...
set "PATH=%OPENVINO_INSTALL_DIR%\runtime\bin\intel64\Release;%OPENVINO_INSTALL_DIR%\runtime\3rdparty\tbb\bin;%PATH%"

:: --- 3. รันโปรแกรม ---
echo Running Rust OCR...
cargo run

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Program exited with error code %ERRORLEVEL%.
    pause
)

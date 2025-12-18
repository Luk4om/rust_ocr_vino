# Rust OCR Detector (OpenVINO)

โปรเจกต์นี้เป็นตัวอย่างการใช้ Rust ร่วมกับ OpenVINO เพื่อทำ OCR Detection โดยใช้โมเดล PaddleOCR

## Requirements (สิ่งที่ต้องมี)

1.  **Rust & Cargo**: ติดตั้งจาก [rustup.rs](https://rustup.rs/)
2.  **OpenVINO 2025.4 Runtime (Windows)**
    - *Note:* โปรเจกต์นี้ใช้ `openvino 0.9` ซึ่ง **บังคับ** ใช้อินสแตนซ์ 2025.x ขึ้นไป

## Installation & Setup (การติดตั้ง)

### 1. ดาวน์โหลด OpenVINO

ให้ดาวน์โหลด OpenVINO Runtime 2025.4.0 สำหรับ Windows จากลิงก์นี้:
[https://storage.openvinotoolkit.org/repositories/openvino/packages/2025.4/windows/openvino_toolkit_windows_2025.4.0.20398.8fdad55727d_x86_64.zip](https://storage.openvinotoolkit.org/repositories/openvino/packages/2025.4/windows/openvino_toolkit_windows_2025.4.0.20398.8fdad55727d_x86_64.zip)

### 2. แตกไฟล์

แตกไฟล์ zip ที่ดาวน์โหลดมา ไปไว้ที่ path ที่ต้องการ (แนะนำ `C:\Drivers\openvino_2025.4`)

### 3. ตั้งค่า Environment Variables (Set Env)

ก่อน Run หรือ Build โปรเจกต์ ต้องตั้งค่า Environment variables เพื่อให้ Rust หา Library เจอ

เปิด **Command Prompt (cmd)** และรันคำสั่งต่อไปนี้ (เปลี่ยน Path ให้ตรงกับที่คุณแตกไฟล์ไว้):

```cmd
:: 1. กำหนด Path หลักของ OpenVINO
set OPENVINO_INSTALL_DIR=C:\Drivers\openvino_2025.4

:: 2. เพิ่ม Path ของไฟล์ DLL (.dll) เข้าไปใน System Path
set PATH=%OPENVINO_INSTALL_DIR%\runtime\bin\intel64\Release;%PATH%
set PATH=%OPENVINO_INSTALL_DIR%\runtime\3rdparty\tbb\bin;%PATH%
```

> **Tip:** หากต้องการเช็คว่า path ถูกต้องหรือไม่ ให้ลองพิมพ์ `where openvino_c.dll` ใน cmd ถ้าขึ้น Path ไฟล์มาแสดงว่าถูกต้อง

## Usage (การใช้งาน)

**วิธีที่แนะนำ (ง่ายสุด):**
1. เปิดไฟล์ `run_ocr.bat` ด้วย Text Editor (เช่น Notepad)
2. แก้ไขบรรทัด `set "OPENVINO_INSTALL_DIR=..."` ให้ตรงกับที่ที่คุณแตกไฟล์ OpenVINO
3. ดับเบิ้ลคลิก `run_ocr.bat` เพื่อรันโปรแกรม

**วิธี Manual:**
เมื่อตั้งค่า Environment แล้ว สามารถรันคำสั่ง:

```bash
cargo run
```

### การปรับแต่ง
- **รูปภาพ:** แก้ไขตัวแปร `IMAGE_PATH` ในไฟล์ `src/main.rs`
- **โมเดล:** ตรวจสอบไฟล์โมเดลในโฟลเดอร์ `modelvino/` (ต้องมี `model.xml` และ `model.bin`)

## Troubleshooting

error: `OpenVINO version is too old ... 2024.6.0...`
- สาเหตุ: คลัง `openvino` ของ Rust เวอร์ชัน 0.9 บังคับใช้ OpenVINO 2025 ขึ้นไป
- วิธีแก้: ให้ดาวน์โหลดและติดตั้ง **OpenVINO 2025.4** ตามลิงก์ด้านบนแทน

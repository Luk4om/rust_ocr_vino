use anyhow::Result;
use image::{GenericImageView, Rgba, Pixel};
use image::imageops::FilterType;
use ndarray::Array4;
use openvino::{Core, Tensor};

// --- ตั้งค่า Parameter ---
const MODEL_XML: &str = "modelvino/model.xml";
const MODEL_BIN: &str = "modelvino/model.bin"; // จำเป็นต้องระบุ path bin ใน Rust บางเวอร์ชัน
const IMAGE_PATH: &str = "image/image.png";
const OUTPUT_PATH: &str = "result2_rust.png";
const INPUT_SIZE: u32 = 640;
const THRESHOLD: f32 = 0.3;

fn main() -> Result<()> {
    // --- Pre-setup for OpenVINO DLLs on Windows ---
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::path::PathBuf;

        let openvino_dir = env::var("INTEL_OPENVINO_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(r"C:\Program Files (x86)\Intel\openvino"));

        let bin_dir = openvino_dir.join("runtime/bin/intel64/Release");
        let tbb_dir = openvino_dir.join("runtime/3rdparty/tbb/bin");

        let current_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{};{};{}", bin_dir.display(), tbb_dir.display(), current_path);
        
        env::set_var("PATH", new_path);
    }

    println!("--- Starting Rust OpenVINO OCR Detection ---");

    // 1. Setup OpenVINO Core
    // 1. Setup OpenVINO Core
    let mut core = Core::new()?;
    
    // โหลดโมเดล (ต้องใช้ไฟล์ .xml)
    // หมายเหตุ: Rust binding บางตัวจะหา .bin เอง แต่ระบุไปเลยชัวร์กว่าถ้า API รองรับ
    // ในที่นี้เราใช้ read_model_from_file แบบมาตรฐาน
    let model = core.read_model_from_file(MODEL_XML, MODEL_BIN)?;
    
    // Compile โมเดลลง CPU
    let mut compiled_model = core.compile_model(&model, "CPU".into())?;
    let mut infer_request = compiled_model.create_infer_request()?;

    // 2. Pre-processing รูปภาพ
    println!("Loading and processing image...");
    let img = image::open(IMAGE_PATH)?;
    let (orig_width, orig_height) = img.dimensions();

    // Resize เป็น 640x640
    let resized_img = img.resize_exact(INPUT_SIZE, INPUT_SIZE, FilterType::Triangle);
    
    // เตรียมข้อมูล Tensor (NCHW Format)
    // Mean & Std ของ PaddleOCR
    let mean = [0.485, 0.456, 0.406];
    let std = [0.229, 0.224, 0.225];

    let mut input_data = Array4::<f32>::zeros((1, 3, INPUT_SIZE as usize, INPUT_SIZE as usize));

    for (x, y, pixel) in resized_img.pixels() {
        let rgb = pixel.to_rgb();
        // Normalize: (pixel/255 - mean) / std
        input_data[[0, 0, y as usize, x as usize]] = ((rgb[0] as f32 / 255.0) - mean[0]) / std[0]; // R
        input_data[[0, 1, y as usize, x as usize]] = ((rgb[1] as f32 / 255.0) - mean[1]) / std[1]; // G
        input_data[[0, 2, y as usize, x as usize]] = ((rgb[2] as f32 / 255.0) - mean[2]) / std[2]; // B
    }

    // 3. Inference
    // แปลง ndarray เป็น OpenVINO Tensor
    // หมายเหตุ: การสร้าง Tensor ใน Rust อาจแตกต่างกันตามเวอร์ชัน crate
    // โค้ดนี้อิงตาม concept ทั่วไปของการส่ง pointer
    let tensor_data = input_data.as_slice().unwrap();
    
    let input = compiled_model.get_input_by_index(0)?;
    let input_type = input.get_element_type()?;
    
    // Model has dynamic shape, so we specify the shape explicitly for the input tensor
    // openvino::Shape::new takes &[i64] and returns Result<Shape>
    let input_shape = openvino::Shape::new(&[1, 3, INPUT_SIZE as i64, INPUT_SIZE as i64])?;
    
    let mut input_tensor = Tensor::new(input_type, &input_shape)?;
    
    input_tensor.get_data_mut::<f32>()?.copy_from_slice(tensor_data);
    
    infer_request.set_tensor(&input.get_name()?, &input_tensor)?;

    println!("Running inference...");
    infer_request.infer()?;

    // 4. Post-processing (Get Output)
    let output = compiled_model.get_output_by_index(0)?;
    let output_tensor = infer_request.get_tensor(&output.get_name()?)?;
    
    // ดึงข้อมูลออกมาเป็น Vector<f32>
    // Output shape ปกติคือ [1, 1, 640, 640]
    let output_data: Vec<f32> = output_tensor.get_data()?.to_vec(); 
    
    // 5. Visualization (Plot Heatmap)
    println!("Visualizing output...");
    
    // สร้างภาพผลลัพธ์โดยก๊อปปี้จากภาพต้นฉบับ
    let mut output_image = img.to_rgba8();

    // คำนวณ Scale ratio เพื่อ map จาก 640x640 กลับไปภาพเดิม
    let scale_x = orig_width as f32 / INPUT_SIZE as f32;
    let scale_y = orig_height as f32 / INPUT_SIZE as f32;

    let mut detected_count = 0;

    // วนลูป Heatmap (640x640)
    for y in 0..INPUT_SIZE {
        for x in 0..INPUT_SIZE {
            // คำนวณ index ใน output_data (flat array)
            let index = (y * INPUT_SIZE + x) as usize;
            let confidence = output_data[index];

            if confidence > THRESHOLD {
                detected_count += 1;

                // คำนวณพิกัดบนภาพจริง
                let real_x = (x as f32 * scale_x) as u32;
                let real_y = (y as f32 * scale_y) as u32;

                // วาดจุดสีแดงลงบนภาพจริง (ระวัง index out of bound)
                if real_x < orig_width && real_y < orig_height {
                    let pixel = output_image.get_pixel_mut(real_x, real_y);
                    // ผสมสีแดง (Red Overlay)
                    // R=255, G=เดิม/2, B=เดิม/2, A=255
                    *pixel = Rgba([255, pixel[1]/2, pixel[2]/2, 255]);
                }
            }
        }
    }

    println!("Detected {} pixels above threshold.", detected_count);

    if detected_count > 0 {
        output_image.save(OUTPUT_PATH)?;
        println!("Saved result image to: {}", OUTPUT_PATH);
    } else {
        println!("No text detected.");
    }

    Ok(())
}
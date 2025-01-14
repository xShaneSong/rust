use image::{ImageFormat, DynamicImage, ImageBuffer, Rgba};
use std::path::PathBuf;
use std::result::Result;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{fmt, fs, io};

use super::error::ImagixError;
struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}
#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}
impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}
impl FromStr for SizeOption {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small),
            //default
        }
    }
}

impl FromStr for Mode {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError(
                "Wrong value for mode".to_string(),
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Single,
    All,
}

pub fn process_resize_request(
    size: SizeOption,
    mode: Mode,
    src_folder: &mut PathBuf,
) -> Result<(), ImagixError> {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };
    let _ = match mode {
        Mode::All => resize_all(size, src_folder)?,
        Mode::Single => resize_single(size, src_folder)?,
    };
    Ok(())
}

fn resize_single(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    let mut src_folder = src_folder;
    // Get file stem from src_folder
    resize_image(size, &mut src_folder)?;
    Ok(())
}

fn resize_all(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    if let Ok(entries) = get_image_files(src_folder.to_path_buf()) {
        for mut entry in entries {
            resize_image(size, &mut entry)?;
        }
    };
    Ok(())
}
////

pub fn resize_image(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    // 1.接收带有完整源文件夹路径的源图像文件名，将其调整为.png文件，并将调整后的文件存储在源文件夹的tmp子文件夹中。
    // 2.从完整路径中提取源文件名。文件扩展名为被更改为.png。
    // 3.使用/tmp构建目标文件路径，因为调整大小后的图像将存储在源文件夹的tmp子文件夹中。
    // 4.调整图像大小并将其保存到目标文件路径。
    // 5.使用Instant::now()和Elapsed::from()计算调整图像大小所需的时间。

    let new_file_name = src_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));

    // 构造目标文件夹路径
    // 例如，如果在源文件中不存/tmp，则创建它。
    let mut dest_folder = src_folder.clone();
    dest_folder.pop();
    dest_folder.push("tmp");
    // let dest_folder = src_folder.join("tmp");
    if !dest_folder.exists() {
        fs::create_dir(&dest_folder).unwrap();
    }

    dest_folder.pop();
    dest_folder.push("tmp/tmp.png");
    dest_folder.set_file_name(new_file_name?.as_str());

    let timer = Instant::now();
    let img = image::open(&src_folder)?;
    let scaled = img.thumbnail(size, size);
    let mut output = fs::File::create(&dest_folder)?;
    scaled.write_to(&mut output, ImageFormat::Png)?;
    println!(
        "Thumbnailed file: {:?} to size {}x{} in {}. Output file in {:?}",
        src_folder,
        size,
        size,
        timer.elapsed().as_millis(),
        dest_folder
    );
    Ok(())
}

pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    // 检索源文件来中的目录条目，将将它们收集在一个向量中。
    // 迭代向量中的条止并过滤图像文件。请注意，本项目中只关注 PNG和 JPG文件，但也可以将其轻松扩展到其他 类型的图像文件中。
    // 从该方法中返回图像文件列表。

    // code from github copilot
    // let mut image_files = Vec::new();
    // for entry in fs::read_dir(src_folder).unwrap() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path.is_file() {
    //         let extension = path.extension().unwrap().to_str().unwrap();
    //         if extension == "png" || extension == "jpg" {
    //             image_files.push(path);
    //         }
    //     }
    // }
    // Ok(image_files)

    let entries = fs::read_dir(src_folder)
        .map_err(|e| ImagixError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
                || r.extension() == Some("PNG".as_ref())
                || r.extension() == Some("jpg".as_ref())
                || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(entries)
}

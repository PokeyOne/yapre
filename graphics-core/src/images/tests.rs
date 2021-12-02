use super::*;

#[test]
fn test_generate_random_temp_path() {
    let a = generate_random_temp_path();
    let b = generate_random_temp_path();

    assert_ne!(a, b);
}

#[test]
fn test_save_image() {
    let mut image = RawImage::new(256, 256);
    for x in 0..256 {
        for y in 0..256 {
            image.set_pixel(Color::new(x as u8, y as u8, 255, 255), x, y);
        }
    }
    let path = generate_random_temp_path();

    let path_prefix = Path::new(&path).parent().unwrap();
    std::fs::create_dir_all(path_prefix).unwrap();

    match image.save_image_to_path(&path) {
        Ok(_) => {},
        Err(e) => panic!("err: {:?}", e)
    }

    assert!(Path::new(&path).exists());
}

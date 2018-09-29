extern crate freenectrs;
extern crate image;
extern crate glium;


mod imgwin;

use freenectrs::freenect;

fn get_image() {

}

fn depth_to_img(data: &[u16]) -> image::RgbaImage {
    image::RgbaImage::from_fn(640, 480, |x, y| {
        let idx = y * 640 + x;
        let val = data[idx as usize];
        // let val = val / 2048;
        // let val = val / 10;
        let val = if val > 600 { val - 600 } else { 0 };
        let val = val / 2;
        let val = if val > 255 { 255 } else { val };
        let val = val as u8;
        image::Rgba([val, val, val, 255])
    })
}

fn main() {
    let dwin = imgwin::ImgWindow::new("Live Depth");
    let context = freenect::FreenectContext::init_with_video().unwrap();
    let device = context.open_device(0).unwrap();

    device.set_depth_mode(freenect::FreenectResolution::Medium, freenect::FreenectDepthFormat::MM).unwrap();
    device.set_video_mode(freenect::FreenectResolution::Medium, freenect::FreenectVideoFormat::Rgb).unwrap();

    let dstream = device.depth_stream().unwrap();
    //let vstream = device.video_stream().unwrap();

    ctx.spawn_process_thread().unwrap();

    loop {
        if let Ok((data, timestamp)) = dstream.receiver.try_recv() {
            let image = depth_to_img(data);
            dwin
        }
        //if let Ok((data, timestamp)) = vstream.receiver.try_recv() {
        //}
    }
}

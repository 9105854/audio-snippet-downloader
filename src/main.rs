use std::io;
use std::process::Command;
use youtube_dl::YoutubeDl;
fn main() {
    let mut video_url : String = String::new();
    println!("Enter the url link: ");
    
    io::stdin()
        .read_line(&mut video_url)
        .expect("An error occured while reading the URL. Please try again.");
    
    let output = YoutubeDl::new(&video_url)
    .youtube_dl_path("./yt-dlp_linux/yt-dlp_linux")
        .socket_timeout("15")
        .download(true)
        .extract_audio(true)
        .format(String::from("bestaudio[ext=m4a]"))
        .extra_arg("-o")
        .extra_arg("video.%(ext)s")
        .extra_arg("--restrict-filenames")
        .run()
        .expect("Failed to download video :(");
    let title = output
        .into_single_video()
        .expect("Sorry, something went wrong parsing the video data. ")
        .title;
    println!("Downloaded Video title: {}", title);
    let final_file = title + ".m4a";
    println!("Enter the start time in seconds: ");
    let mut start_time: String = String::new();
    io::stdin()
        .read_line(&mut start_time)
        .expect("An error occured reading the time. Please try again. ");
    let start_time = start_time.trim();
    println!("Cutting audio...");
    let mut ffmpeg = Command::new("./ffmpeg/ffmpeg");
    ffmpeg
        .arg("-ss")
        .arg(start_time)
        .arg("-i")
        .arg("video.m4a")
        .arg("-t")
        .arg("15")
        .arg("-c")
        .arg("copy")
        .arg(final_file)
        .spawn()
        .expect("Something went wrong. Please try again.");
        
    println!("Finished!");
}

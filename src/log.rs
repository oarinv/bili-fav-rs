pub fn init_logging(){
    wd_log::set_level(wd_log::INFO);
    //Set the log output prefix, default:"wd_log"
    wd_log::set_prefix("main");
    //Whether to display the print time, default:true
    wd_log::show_time(true);
    //Whether to display the location, default:true
    wd_log::show_file_line(false);
    //Set output to a file, default:stdout
    wd_log::output_to_file("./log.txt").expect("file open failed");
}
use daemonize::Daemonize;
use clap::{Arg, Command};
use fps_clock::FpsClock;

use std::{fs::{File, self}, thread, time::Duration, sync::mpsc};

mod display;
use display::Display;

mod input;
use input::Input;

mod event;
use event::{Event, EventType};

mod conf;
use conf::Config;

fn main() {
    let args = Command::new("smartusb")
        .version("0.1.0")
        .about("Enables the RPi Zero to be a very smart USB device")
        .arg(Arg::with_name("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .default_value("/etc/smartusb/config.toml")
                .help("The path to the main config file"))
        .arg(Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .help("More verbose logging"))
        .arg(Arg::with_name("daemon")
                .short('d')
                .long("daemon")
                .help("Run the smartusb daemon"))
        .get_matches();
    
    let config_path: &str = args.get_one::<String>("config").unwrap();
    let config: Config = match fs::read_to_string(config_path) {
        Ok(s) => Config::from_str(&s).expect("Error loading config"),
        Err(_) => {
            eprintln!("WARN: Config file does not exist, using defaults.");
            Config::default()
        },
    };
    
    let is_raspi = fs::read_to_string("/proc/device-tree/model").map_or(false, |text| text.contains("Raspberry"));
    
    if args.is_present("daemon") {
        if !is_raspi {
            panic!("Refusing to start daemon (Unknown device model)");
        }
        
        let stdout = File::create("/tmp/smartusb.log").expect("Unable to open logfile");
        let stderr = stdout.try_clone().expect("Unable to open logfile");
        let daemonize = Daemonize::new()
            .pid_file("/tmp/smartusb.pid")
            .working_directory("/tmp")
            .stdout(stdout)
            .stderr(stderr);
        
        match daemonize.start() {
            Err(e) => eprintln!("Error, {}", e),
            Ok(_) => main_loop(&config),
        }
    }

    println!("Running smartusb in foreground mode.");
    main_loop(&config);
}

fn main_loop(conf: &Config) {
    let (queue_tx, queue_rx) = mpsc::channel::<Event>();

    let mut oled = Display::new(conf.flip_screen).expect("Unable to create display");
    oled.init();

    let mut input = Input::new(conf.flip_buttons).expect("Unable to initialize input");
    input.connect_interrupts(queue_tx).expect("Error connecting input interrupts");

    oled.draw_text("  Hello world 1!", 0, 0);
    oled.draw_text("  Hello world 2!", 0, 8);
    oled.draw_text("  Hello world 3!", 0, 16);
    oled.draw_text("  Hello world 4!", 0, 24);
    oled.draw_text("  Hello world 5!", 0, 32);
    oled.draw_text("  Hello world 6!", 0, 40);
    oled.draw_text("  Hello world 7!", 0, 48);
    oled.draw_text("  Hello world 8!", 0, 56);
    oled.flush();

    println!("test success");

    let mut timer = FpsClock::new(10);
    loop {
        // process events
        while let Ok(msg) = queue_rx.try_recv() {
            println!("{:?}", msg);
        }

        // update display
        oled.flush();
        
        // wait for remainder of a frame
        // thread::sleep(Duration::from_millis(1));
        timer.tick();
    }
}

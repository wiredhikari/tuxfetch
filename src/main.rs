use ansi_term::Colour::{Red, RGB};
use std::process::Command;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};
fn main() {
    //iinitialize
    let mut sys = System::new_all();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    sys.refresh_all();

    //tux
    println!("{}",RGB(70, 130, 180).paint("         .LINUS.\r\n        $GENTOO$     ,\"\"\"\"\"\"\"\"\"\"\"\"\"\".\r\n       @portage@|    | Linux Rules! |\r\n       M|@||@) M|   _;..............\'\r\n       @,-----,@| -\'\r\n      C ^\\__/^ rust\r\n     aur        arch\r\n    rpm          suse\r\n   deb            enos\r\n   MMM            MMMM\r\n   MMM            MMMM\r\n __| \".        |\\OS\"vim\r\n |    `.       | `\' \\al\r\n/     \\.______.|     .\'\r\n\\____  |Windows|   .\'Imma\r\n     `-\'       `--\' god") );
    //coordinates
    let x_coord = 45;
    let mut y_coord = stdout.cursor_pos().unwrap().1;

    //1]User
    y_coord = y_coord - 14;
    println!(
        "{}{}      : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|User"),
        whoami::username()
    );

    //2]Host
    y_coord = y_coord + 1;
    println!(
        "{}{}      : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|Host"),
        sys.host_name().unwrap()
    );

    //3}OS;
    y_coord = y_coord + 1;
    println!(
        "{}{}        : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|OS"),
        sys.name().unwrap()
    );

    //4]Kernel
    y_coord = y_coord + 1;
    println!(
        "{}{}    : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|Kernel"),
        sys.kernel_version().unwrap()
    );

    // //5]Up-Time
    y_coord = y_coord + 1;
    println!(
        "{}{}    : {} \r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|UpTime"),
        nixinfo::uptime().unwrap()
    );

    //6]terminal
    y_coord = y_coord + 1;
    println!(
        "{}{}  : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|Terminal"),
        nixinfo::terminal().unwrap()
    );

    // //8]wm
    y_coord = y_coord + 1;
    let wm = Command::new("sh").arg("wm.sh").output().expect(" ");
    println!(
        "{}{}        : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|WM"),
        //nixinfo::environment().unwrap()
        String::from_utf8_lossy(&wm.stdout)
    );

    //10]cpu
    y_coord = y_coord + 1;
    println!(
        "{}{}       : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|CPU"),
        sys.processors()[0].brand()
    );

    y_coord = y_coord + 1;
    println!(
        "{}{}       : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|GPU"),
        nixinfo::gpu().unwrap()
    );

    //15]Usage
    y_coord = y_coord + 1;
    let mut usage = 0;

    for processor in sys.processors() {
        usage = usage + processor.cpu_usage() as usize;
    }
    println!(
        "{}{}     : {}%\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|Usage"),
        usage / sys.processors().len()
    );
    //11]Cores
    y_coord = y_coord + 1;
    println!(
        "{}{}     : {}\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|Cores"),
        sys.processors().len()
    );

    //12]Temperature
    y_coord = y_coord + 1;
    for component in sys.components() {
        if component.label().contains("Package id") {
            println!(
                "{}{}      : {}°C\r",
                termion::cursor::Goto(x_coord, y_coord),
                Red.bold().paint("|Temp"),
                component.temperature()
            );

            break;
        }
    }

    //13]RAM
    y_coord = y_coord + 1;
    println!(
        "{}{}       : {}MB/{}MB\r",
        termion::cursor::Goto(x_coord, y_coord),
        Red.bold().paint("|RAM"),
        sys.used_memory() / 1024,
        sys.total_memory() / 1024
    );

    //14]Swap
    y_coord = y_coord + 1;
    if sys.total_swap() == 0 {
        println!(" ");
    } else {
        println!(
            "{}{}      : {}MB/{}MB\r",
            termion::cursor::Goto(x_coord, y_coord),
            Red.bold().paint("|Swap"),
            sys.used_swap() / 1024,
            sys.total_swap() / 1024
        );
    }

    //let the tux rule
}

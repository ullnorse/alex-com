



fn main() {
    println!("Crossbeam test");

    let (send, recv) = crossbeam_channel::bounded::<String>(10);
    let q = crossbeam::queue::ArrayQueue::<String>::new(10);

    crossbeam::scope(|s| {
        s.spawn(|_| {
            let mut i = 1;

            let mut port = serialport::new("/dev/ttyUSB0", 9600)
                .open()
                .unwrap();
    
            port.clear(serialport::ClearBuffer::All).unwrap();

            let mut buffer = [0u8; 1];

            let mut q = Vec::new();

            loop {
                match port.read_exact(&mut buffer) {
                    Ok(_) => {
                        let c = buffer[0] as char;
                        match c {
                            '\n' => {
                                q.push(c);
                                let s: String = q.iter().collect();
                                print!("{s}");
                                send.send(s).unwrap();
                            },
                            _ => q.push(c),
                        }
                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e)
                }
            }

            // drop(send);
        });

        s.spawn(|_| {
            loop {
                // if let Ok(i) = recv.recv() {
                //     println!("{i}");
                // }
                if let Some(i) = q.pop() {
                    println!("{i} {:?}", std::thread::current().id());
                }
            }
        });
        
    }).unwrap();
}
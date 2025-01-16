use signal_hook::iterator::Signals;

// revice signals
fn listen_signals() {
    let signals = Signals::new(&[signal_hook::SIGTERM, signal_hook::SIGINT])?;
    'signal_loop: loop {
        for signal in signals.pending() {
            match signal {
                signal_hook::SIGTERM => {
                    println!("SIGTERM signal received");
                    break 'signal_loop;
                }
                signal_hook::SIGINT => {
                    println!("SIGINT signal received");
                    break 'signal_loop;
                }
                _ => unreachable!(),
            }
        }
    }
}
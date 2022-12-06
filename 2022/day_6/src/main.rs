use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let t = Instant::now();
    let packet_marker = find_marker(input, 4);
    let pm = t.elapsed();
    println!("Packet marker: {packet_marker} ({pm:?})");

    let message_marker = find_marker(input, 14);
    let mm = t.elapsed() - pm;
    println!("Message marker: {message_marker} ({mm:?})");
}

fn all_different(window: &&[char], window_size: usize) -> bool {
    window
        .iter()
        .take_while(|c| window.iter().filter(|sc| sc == c).count() == 1)
        .count()
        == window_size
}

fn find_marker(stream: &str, marker_size: usize) -> usize {
    stream
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_size)
        .enumerate()
        .find(|(_, w)| all_different(w, marker_size))
        .unwrap()
        .0
        + marker_size
}

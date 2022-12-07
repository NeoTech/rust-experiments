#[cfg(target_os = "linux")]
extern crate ncurses;
use ncurses::*;
pub fn ncurses(graph: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: ncurses <value,value,value,value,value>");
        return;
    }
    let graph: Vec<&str> = graph.split(",").collect();
    let mut data: Vec<i32> = Vec::new();

    for arg in &graph[1..] {
        data.push(arg.parse::<i32>().unwrap());
    }

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut max_value = 0;
    for value in &data {
        if *value > max_value {
            max_value = *value;
        }
    }

    let mut x = 0;
    for value in &data {
        let height = (*value as f32 / max_value as f32) * (LINES() - 1) as f32;
        for y in 0..height as i32 {
            mvaddch(LINES() - y - 1, x, '#' as u32);
        }
        x += 1;
    }

    refresh();
    getch();
    endwin();
}
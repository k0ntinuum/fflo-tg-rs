mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use std::env;
use std::process::exit;   
fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 8 {
        print!("To use Kontinnum-Textgraphics, add parameters in the following order :\n");
        print!("kont-tg [rows] [cols] [filters] [filterspan] [flux] [updates per step] [steps]\n");
        reset();
        exit(0);

    }
    clear_screen();
    let mut default_settings: Vec<usize> =vec![25,50,7,5,20,3000,500];
    for i in 0..default_settings.len() {
        if i < args.len() - 1 {
            let parsed = match args[i+1].parse::<usize>() {
                Ok(u) => u,
                Err(_) => 0,
            };
            if parsed > 0 && i < default_settings.len() {
                default_settings[i] = parsed;
            }
        }
    }
    let rows = default_settings[0];
    let cols = default_settings[1];
    let filters = default_settings[2];
    let span = default_settings[3] as i32;
    let flux = default_settings[4];
    let updates = default_settings[5];
    let duration = default_settings[6];
    let right = 10;
    let down = 0;
    clear_screen();
    hide_cursor();
    let mut filter_system = simple_random_filters(filters,span,span,2.0);
    let mut state = random_state(rows, cols);
    
    
    for i in 0..duration {
        for _ in 0..updates {

            let row =  rand::random::<usize>() % rows;
            let col =  rand::random::<usize>() % cols;
            for f in &filter_system {
                filter_state_mutate_cell(&f, &mut state, row, col, rows, cols);
            }
        }
        
        display(&state, rows, cols, right, down+1);
        set_color(255,255,255);
        cursor_to(down,right);
        print!("Kontinuum [ Textgraphics Version ] {} {} {} {} {} {} {}", rows, cols, filters, span, flux, updates, duration-i);
        if rand::random::<usize>()%1000 < flux {
            filter_system = simple_random_filters(filters,span,span,0.5 + 0.5*rand::random::<f64>());
        }

        

    }
    reset();

}

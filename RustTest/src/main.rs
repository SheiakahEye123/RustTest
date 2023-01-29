use std::{fmt, thread, time::Duration};

fn main() {
    let mut state = [[Cell::Dead; 50]; 50];
    let mut next = [[Cell::Dead; 50]; 50];
    state[7][10] = Cell::Alive;
    state[6][10] = Cell::Alive;
    state[5][10] = Cell::Alive;
    state[7][11] = Cell::Alive;
    state[6][12] = Cell::Alive;
    while true {
        println!("Hello, newest world!");
        let mut x = 0;
        let mut y = 0;

        while x < 50 {
            y = 0;
            while y < 50 {
                if x > 0 && x < 49 && y > 0 && y < 49 {
                    let mut lupe = 0;
                    let neighbors = [state[x-1][y],state[x+1][y],state[x-1][y-1],state[x+1][y-1],state[x-1][y+1],state[x+1][y+1],state[x][y-1],state[x][y+1],];
                    let mut liveneighbors = 0;
                    while lupe < 8 {
                        if neighbors[lupe] == Cell::Alive{
                            liveneighbors += 1;
                        }
                        lupe += 1
                    }

                    if state[x][y] == Cell::Alive && liveneighbors >= 2 && liveneighbors <= 3 {
                        next[x][y] = Cell::Alive
                    }
                    else if state[x][y] == Cell::Dead && liveneighbors == 3 {
                        next[x][y] = Cell::Alive
                    }
                    else if state[x][y] == Cell::Alive && liveneighbors < 2 {
                        next[x][y] = Cell::Dead
                    }
                    else if state[x][y] == Cell::Alive && liveneighbors > 3 {
                        next[x][y] = Cell::Dead
                    }
                }
                else {
                }
                y += 1
            }
            x += 1
        }
        println!("Hello, new world!");
        for line in state {
            println!("{:?}", format!("{:?}", line));
        }
        state = next;
        thread::sleep(Duration::from_millis(400));
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]

pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl std::fmt::Debug for Cell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Dead => write!(f,"⎔"),
            Cell::Alive => write!(f,"⬢"),
        }
    }
}

impl Default for Cell{
    fn default() -> Self {
        Cell::Dead
    }
}

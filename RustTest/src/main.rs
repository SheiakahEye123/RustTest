use std::{fmt};

fn main() {
    println!("Hello, world!");
    let mut state = [[Cell::Dead; 50]; 50];
    let mut next = [[Cell::Dead; 50]; 50];
    state[0][1] = Cell::Alive;
    state[0][2] = Cell::Alive;
    state[1][1] = Cell::Alive;
    state[3][0] = Cell::Alive;
    loop {

        let x = 0;
        let y = 0;

        while x < 50 {
            while y < 50 {
                if x > 0 && x < 50 && y > 0 && y < 50 {
                    let mut lupe = 0;
                    let neighbors = [state[x-1][y],state[x+1][y],state[x-1][y-1],state[x+1][y-1],state[x-1][y+1],state[x+1][y+1],state[x][y-1],state[x][y+1],];
                    while lupe < 8 {
                        let mut liveneighbors = 0;
                        if neighbors[lupe] == Cell::Alive{
                            liveneighbors += 1
                        }
                        if liveneighbors >= 2 && liveneighbors <= 3 {
                            next[y][x] = Cell::Alive
                        }
                    }
                }
            }
        }
        println!("Hello, new world!");
        println!("{:?}", format!("{:?}", state));
        state = next
    }
}

#[derive(Copy, Clone)]

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

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true
        }
        else {
            return false
        }
    }
    fn ne(&self, other: &Self) -> bool {
        if self != other {
            return true
        }
        else {
            return false
        }
    }
}

impl Default for Cell{
    fn default() -> Self {
        Cell::Dead
    }
}

// #![allow(dead_code, unused)]

// TODO: definitely refactor this using impl
pub fn print_generation(generation: &[Cell]) {
    for cell in generation {
        match cell.state {
            State::Alive => print!("█"),
            State::Dead  => print!(" "),
        }
    }
    println!();
}

pub fn determine_next_generation(generation: &[Cell]) -> Vec<Cell> {
    let mut next_generation: Vec<Cell> = vec![Cell::new(State::Dead)];

    for i in 0..generation.len() - 1 {
        // TODO: maybe refactor this using methods?
        next_generation.push(Cell::new(matcher(take_three_items(i, generation))))
    }

    next_generation
}

pub fn initialize_first_generation(length: usize) -> Vec<Cell> {
    let mut first_generation: Vec<Cell> = Vec::new();

    for _i in 0..length - 1 {
        first_generation.push(Cell::new(State::Dead))
    }

    first_generation.push(Cell::new(State::Alive));

    first_generation
}

pub fn take_three_items(index: usize, generation: &[Cell]) -> (State, State, State) {
    assert!(index <= generation.len() - 2);

    // TODO: a more elegant wraparound should be possible.
    if index == generation.len() - 2 {
        (generation[index].state, generation[index + 1].state, generation[0].state)
    } else {
        (generation[index].state, generation[index + 1].state, generation[index + 2].state)
    }
}

pub fn matcher(states: (State, State, State)) -> State {
    match states {
        (State::Alive,  State::Alive,   State::Alive)   => State::Dead,
        (State::Alive,  State::Alive,   State::Dead)    => State::Alive,
        (State::Alive,  State::Dead,    State::Alive)   => State::Alive,
        (State::Alive,  State::Dead,    State::Dead)    => State::Dead,
        (State::Dead,   State::Alive,   State::Alive)   => State::Alive,
        (State::Dead,   State::Alive,   State::Dead)    => State::Alive,
        (State::Dead,   State::Dead,    State::Alive)   => State::Alive,
        (State::Dead,   State::Dead,    State::Dead)    => State::Dead,
    }
}

#[derive(Debug)]
pub struct Cell {
    pub state: State,
}


impl Cell {
    fn new(state: State) -> Cell {
        Cell {
            state,
        }
    }
}
    
#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive,
    Dead,
}



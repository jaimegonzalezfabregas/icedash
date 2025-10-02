use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread::spawn,
};

use rand::{
    random,
    seq::{IndexedRandom, IteratorRandom},
    Rng,
};
use sorted_vec::partial::ReverseSortedVec;

use crate::logic::{
    creature::Creature,
    noise_reduction::{asthetic_cleanup, is_board_valid, map_noise_cleanup},
    solver::step,
};

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (-1, 0),
            Direction::West => (1, 0),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum Tile {
    Entrance,
    Gate,
    Wall,
    Ice,
    ThinIce(u8),
    WeakBox(u8),
    Outside,
}

impl Tile {
    fn simbol(&self) -> &str {
        match self {
            Tile::Entrance => "E",
            Tile::Gate => "G",
            Tile::Wall => "#",
            Tile::Ice => " ",
            Tile::ThinIce(_) => "t",
            Tile::WeakBox(_) => "b",
            Tile::Outside => " ",
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub map: Vec<Vec<Tile>>,
    pub start: (isize, isize),
    pub end: (isize, isize),
    pub reset_pos: (isize, isize),
    pub start_direction: Direction,
    pub end_direction: Direction,
}

impl Board {
    pub fn mutate(&self, factor: f32) -> Option<Self> {
        return Some(self.clone());
        let mut rng = rand::rng();

        let mut ret = self.clone();
        let height = ret.map.len();
        let width = ret.map[0].len();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.random::<f32>() < factor {
                    match ret.map[y as usize][x as usize] {
                        Tile::Wall => ret.map[y as usize][x as usize] = Tile::Ice,
                        Tile::Ice => ret.map[y as usize][x as usize] = Tile::Wall,
                        _ => {}
                    }
                }
            }
        }

        map_noise_cleanup(
            &mut ret.map,
            &mut ret.start,
            ret.start_direction,
            &mut ret.end,
            ret.end_direction,
        );

        if is_board_valid(&ret) {
            Some(ret)
        } else {
            None
        }
    }

    pub fn new_random() -> Option<Self> {
        let mut rng = rand::rng();
        let width = (5..15).choose(&mut rng)?;
        let height = (5..15).choose(&mut rng)?;

        let start_side = (0..3).choose(&mut rng)?;
        let end_side = ((1..3).choose(&mut rng)? + start_side) % 4;

        let (start, start_direction) = match start_side {
            0 => ((0, (2..height - 2).choose(&mut rng)?), Direction::West),
            1 => (
                (width - 1, (2..height - 2).choose(&mut rng)?),
                Direction::East,
            ),
            2 => (((2..width - 2).choose(&mut rng)?, 0), Direction::South),
            _ => (
                ((2..width - 2).choose(&mut rng)?, height - 1),
                Direction::North,
            ),
        };

        let (end, end_direction) = match end_side {
            0 => ((0, (2..height - 2).choose(&mut rng)?), Direction::West),
            1 => (
                (width - 1, (2..height - 2).choose(&mut rng)?),
                Direction::East,
            ),
            2 => (((2..width - 2).choose(&mut rng)?, 0), Direction::South),
            _ => (
                ((2..width - 2).choose(&mut rng)?, height - 1),
                Direction::North,
            ),
        };

        let mut map = vec![vec![Tile::Wall; width as usize]; height as usize];

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                map[y as usize][x as usize] = Tile::Ice;
            }
        }

        let columns = ((width * height) / 10..(width * height) / 5).choose(&mut rng)?;

        for _ in 0..columns {
            let x = (1..(width - 1) as usize).choose(&mut rng)?;
            let y = (1..(height - 1) as usize).choose(&mut rng)?;

            map[y][x] = Tile::Wall;
        }

        let vignet = ((width * height) / 10..(width * height) / 5).choose(&mut rng)?;

        for _ in 0..vignet {
            let x = (1..(width - 1) as usize).choose(&mut rng)?;
            let y = (1..(height - 1) as usize).choose(&mut rng)?;

            let normal_x = (x as f32 / width as f32) - 0.5;
            let normal_y = (y as f32 / height as f32) - 0.5;

            let normal_d = normal_x * normal_x + normal_y * normal_y;

            if random::<f32>() > normal_d {
                map[y][x] = Tile::Wall;
            }

            map[y][x] = Tile::Wall;
        }
        let mut start = start;
        let mut end = end;

        map_noise_cleanup(
            &mut map,
            &mut start,
            start_direction,
            &mut end,
            end_direction,
        );

        let ret = Board {
            reset_pos: step(&map, &start, start_direction),
            map,
            start,
            start_direction,
            end,
            end_direction,
        };

        if is_board_valid(&ret) {
            Some(ret)
        } else {
            None
        }
    }

    pub(crate) fn print(&self, highlight: Vec<(isize, isize)>) {
        println!(
            "printing start {:?} {:?} end {:?} {:?}",
            self.start, self.start_direction, self.end, self.end_direction
        );

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if highlight.contains(&(x as isize, y as isize)) {
                    print!(". ");
                } else {
                    print!("{} ", tile.simbol());
                }
            }
            println!("");
        }
    }
}

static G_RET_CHANNEL: Mutex<Option<mpsc::Receiver<Board>>> = Mutex::new(None);
static G_KILL_CHANNEL: Mutex<Option<mpsc::Sender<()>>> = Mutex::new(None);

pub fn search_board() -> Board {
    {
        let mut thread = G_KILL_CHANNEL.lock().unwrap();
        let thread = &mut (*thread);
        thread
            .take()
            .unwrap()
            .send(())
            .expect("could not send kill signal to child worker");
    };

    let ret = {
        let mut recv = G_RET_CHANNEL.lock().unwrap();
        let thread = &mut (*recv);

        let rx = thread.take().unwrap();

        let mut ret = rx.recv().unwrap();
        while let Ok(x) = rx.try_recv() {
            ret = x;
        }
        ret
    };

    start_search();
    asthetic_cleanup(ret)
}

fn start_search() {
    let (kill_tx, kill_rx) = mpsc::channel();
    let (ret_tx, ret_rx) = mpsc::channel();

    let mut ret = G_RET_CHANNEL.lock().unwrap();
    let mut kill = G_KILL_CHANNEL.lock().unwrap();

    *ret = Some(ret_rx);
    *kill = Some(kill_tx);

    spawn(|| genetic_search_thread(ret_tx, kill_rx));
}

fn genetic_search_thread(returns: Sender<Board>, kill: Receiver<()>) {
    let mut rng = rand::rng();

    let mut population: ReverseSortedVec<Creature> = ReverseSortedVec::new();

    let mut generations = 0;

    let mut best_so_far = 0.;

    loop {
        while population.len() < generations * 3 {
            if let Some(new_creature) = Creature::board_to_creature(Board::new_random()) {
                population.insert(new_creature);
                if population[0].fitness > best_so_far {
                    best_so_far = population[0].fitness;
                    returns
                        .send(population[0].board.clone())
                        .expect("unable to send best so far");
                }
            }
        }

        while population.len() < generations * 9 {
            let creature = population[0..generations * 2].choose(&mut rng).unwrap();

            if let Some(new_creature) = creature.mutate(0.3) {
                population.insert(new_creature);
                if population[0].fitness > best_so_far {
                    best_so_far = population[0].fitness;
                    returns
                        .send(population[0].board.clone())
                        .expect("unable to send best so far");
                }
            }
        }

        let fitness = population
            .iter()
            .map(|e| e.analisis.fitness())
            .collect::<Vec<_>>();
        println!("generations: {} decicisons: {fitness:?}", generations);

        population = population[0..generations * 2]
            .into_iter()
            .cloned()
            .collect();

        generations += 1;

        if let Ok(()) = kill.try_recv() {
            return;
        }
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
    start_search();
}

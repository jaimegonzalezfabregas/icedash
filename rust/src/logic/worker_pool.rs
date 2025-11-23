use std::{
    sync::{
        mpsc::{self},
        Mutex,
    },
    thread::{self, available_parallelism, spawn, JoinHandle},
    time::{self},
};

use sorted_vec::partial::{SortedSet, SortedVec};

use crate::{
    api::{
        board_description::BoardDescription,
        dart_board::DartBoard,
        direction::Direction,
        main::{AutoGenOutput, LeftRotatable},
    },
    logic::{
        board::Board,
        noise_reduction::asthetic_cleanup,
        solver::{analyze, Analysis},
    },
};

pub enum CtrlMsg {
    Kill,
    Halt(usize),
}

struct Candidate {
    pub fitness: f32,
    pub board: Board,
    pub analysis: Analysis,
}

impl std::fmt::Debug for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Candidate")
            .field("fitness", &self.fitness)
            .finish()
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

struct Worker {
    crtl_channel: mpsc::Sender<CtrlMsg>,
    join: JoinHandle<()>,
}

static G_WORKER: Mutex<Option<Vec<Worker>>> = Mutex::new(None);
static G_RESULT_QUEUE: Mutex<SortedSet<Candidate>> = Mutex::new(SortedSet::new());
static G_BOARD_DESC: Mutex<Option<BoardDescription>> = Mutex::new(None);
static G_RESULT_MAX_SIZE: Mutex<usize> = Mutex::new(0);

fn submit(candidate: Candidate) -> f32 {
    // println!("submiting a candidate with fitness {}", candidate.fitness);
    let mut result = G_RESULT_QUEUE.lock().unwrap();

    result.insert(candidate);

    let mut ret = 0.;

    if result.len() != 0 {
        ret = result[0].fitness;
    }

    while result.len() > *(G_RESULT_MAX_SIZE.lock().unwrap()) {
        result.remove_index(0);
        if result.len() != 0 {
            ret = result[0].fitness;
        }
    }

    // println!("  >  new fitness goal is {ret} {result:?}");

    ret
}

pub fn get_new_room(entry_direction: Direction) -> AutoGenOutput {
    let mut ret = G_RESULT_QUEUE.lock().unwrap();

    match ret.pop() {
        None => {
            if *G_RESULT_MAX_SIZE.lock().unwrap() == 0 {
                AutoGenOutput::NoMoreBufferedBoards
            } else {
                AutoGenOutput::NotReady
            }
        }

        Some(candidate) => {
            let mut max_size = G_RESULT_MAX_SIZE.lock().unwrap();

            *max_size -= 1;

            if *max_size == 0 {
                stop_search();
            }

            candidate.board.print(
                candidate.analysis.routes[0][0]
                    .solution
                    .iter()
                    .map(|e| e.1)
                    .collect(),
            );
            let mut board = candidate.board;
            while board.gates[0].inwards_direction != entry_direction {
                board = board.rotate_left();
            }

            let board = asthetic_cleanup(board, &candidate.analysis, 0);

            let board_desc = G_BOARD_DESC.lock().unwrap();

            AutoGenOutput::Ok(DartBoard::new(
                board,
                Some((candidate.analysis, board_desc.clone().unwrap().game_mode)),
            ))
        }
    }
}

pub fn start_search(board_desc: BoardDescription, max_buffered_boards: isize) {
    {
        *(G_RESULT_MAX_SIZE.lock().unwrap()) = max_buffered_boards as usize;
    }
    {
        *(G_BOARD_DESC.lock().unwrap()) = Some(board_desc.clone());
    }

    let mut g_worker_queue = G_WORKER.lock().unwrap();

    if let None = *g_worker_queue {
        // let paralelism = available_parallelism()
        //     .expect("couldnt get available parallelism")
        //     .get()
        //     / 2;
        let paralelism = 1;

        *g_worker_queue = Some(
            (0..paralelism)
                .map(|_| {
                    let (ctrl_tx, ctrl_rx) = mpsc::channel();

                    let board_desc = board_desc.clone();

                    Worker {
                        crtl_channel: ctrl_tx,
                        join: spawn(move || worker_thread(ctrl_rx, board_desc.clone())),
                    }
                })
                .collect(),
        );
    }
}

pub fn stop_search() {
    let mut g_worker_queue = G_WORKER.lock().unwrap();

    if let Some(workers) = g_worker_queue.take() {
        workers.iter().for_each(|e| {
            e.crtl_channel
                .send(CtrlMsg::Kill)
                .expect("worker leak: Worker comunications failed on kill")
        });
    }
}

pub fn halt_search(millis: usize) {
    let mut g_worker_queue = G_WORKER.lock().unwrap();

    let worker_queue = g_worker_queue.take();

    if let Some(ref workers) = worker_queue {
        workers.iter().for_each(|e| {
            e.crtl_channel
                .send(CtrlMsg::Halt(millis))
                .expect("worker leak: Worker comunications failed on kill")
        });
    }

    *g_worker_queue = worker_queue;
}

pub fn worker_thread(messenger: mpsc::Receiver<CtrlMsg>, board_desc: BoardDescription) {
    let mut fitness_filter = 0.;
    let mut iter = 0;

    loop {
        iter += 1;
        match messenger.try_recv() {
            Ok(CtrlMsg::Halt(time)) => {
                thread::sleep(time::Duration::from_millis(time as u64));
                continue;
            }
            Ok(CtrlMsg::Kill) => {
                println!("reached {iter} iter, killed by ctrl msg");

                return;
            }
            Err(_) => {}
        }

        if let Ok(board) = Board::new_random(&board_desc) {
            if let Ok(analysis) = analyze(&board, 0, 1) {
                let fitness = analysis.compute_fitness(&board.map, &board_desc);

                if fitness > fitness_filter {
                    fitness_filter = submit(Candidate {
                        fitness,
                        board,
                        analysis,
                    })
                }
            }
        }
    }
}

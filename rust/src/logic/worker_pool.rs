use std::{
    sync::{
        mpsc::{self},
        Mutex,
    },
    thread::{self, available_parallelism, spawn, JoinHandle},
    time::{self},
};

use sorted_vec::{partial::ReverseSortedVec, partial::SortedVec};

use crate::{
    api::{
        direction::Direction,
        main::{AutoGenOutput, BoardDescription, DartBoard, LeftRotatable},
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
    SetBestFitness(f32),
}

struct Candidate {
    pub fitness: f32,
    pub board: Board,
    pub analysis: Analysis,
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
static G_RESULT_QUEUE: Mutex<SortedVec<Candidate>> = Mutex::new(SortedVec::new());
static G_RESULT_MAX_SIZE: Mutex<usize> = Mutex::new(0);

fn submit(candidate: Candidate) -> f32 {
    println!("submiting a candidate with fitness {}", candidate.fitness);
    let mut result = G_RESULT_QUEUE.lock().unwrap();

    result.insert(candidate);

    let mut ret = 0.;

    while result.len() > *(G_RESULT_MAX_SIZE.lock().unwrap()) {
        result.remove_index(0);
        ret = result[0].fitness;
    }

    println!("  >  new fitness goal is {ret}");

    ret
}

pub fn get_new_room(entry_direction: Direction) -> AutoGenOutput {
    let ret = G_RESULT_QUEUE.lock().unwrap().pop();

    match ret {
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

            AutoGenOutput::Ok(DartBoard::new(board, candidate.analysis))
        }
    }
}

pub fn start_search(board_desc: BoardDescription, max_buffered_boards: isize) {
    {
        *(G_RESULT_MAX_SIZE.lock().unwrap()) = max_buffered_boards as usize;
    }

    let mut g_worker_queue = G_WORKER.lock().unwrap();

    if let None = *g_worker_queue {
        let paralelism = available_parallelism()
            .expect("couldnt get available parallelism")
            .get()
            / 2;

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
            Ok(CtrlMsg::SetBestFitness(fitness)) => {
                fitness_filter = fitness;
            }
            Err(_) => {}
        }

        if let Ok(board) = Board::new_random(&board_desc) {
            if let Ok(analysis) = analyze(&board, 0, 1) {
                let fitness = analysis.compute_fitness(&board.map);

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

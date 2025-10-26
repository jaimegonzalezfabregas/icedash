use std::{
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread::{self, available_parallelism, spawn},
    time::{self, Duration},
};

use crate::{
    api::main::DartBoard,
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

struct Worker {
    return_channel: mpsc::Receiver<(Analysis, Board)>,
    crtl_channel: mpsc::Sender<CtrlMsg>,
}

static G_WORKER: Mutex<VecDeque<Worker>> = Mutex::new(VecDeque::new());

pub fn get_new_room() -> DartBoard {
    let (analysis, board) = {
        let mut workers = G_WORKER.lock().unwrap();
        let workers = &mut (*workers);
        let worker = workers.pop_front().unwrap();

        let mut ret = worker
            .return_channel
            .recv_timeout(Duration::from_millis(500))
            .expect("Worker Thread did not return any boards");

        if let Some(last) = worker.return_channel.try_iter().last() {
            ret = last;
        }

        spawn(move || {
            worker
                .crtl_channel
                .send(CtrlMsg::Kill)
                .expect("could not send kill signal to child worker");
        });

        ret
    };

    spawn(move || {
        start_search();
    });

    board.print(vec![]);
    board.print(analysis.routes[0][0].solution.iter().map(|e| e.1).collect());

    let board = asthetic_cleanup(board);

    DartBoard::new(board, analysis)
}

pub fn worker_halt(millis: usize) {
    let mut workers = G_WORKER.lock().unwrap();
    let workers = &mut (*workers);
    workers.iter().for_each(|worker| {
        worker.crtl_channel.send(CtrlMsg::Halt(millis)).unwrap();
    });
}

pub fn start_search() {
    let mut ret = G_WORKER.lock().unwrap();

    while ret.len()
        < (available_parallelism()
            .expect("couldnt get available parallelism")
            .get()
            - 3)
    {
        // while ret.len() < 1 {
        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (ret_tx, ret_rx) = mpsc::channel();
        ret.push_back(Worker {
            return_channel: ret_rx,
            crtl_channel: ctrl_tx,
        });

        spawn(|| worker_thread(ret_tx, ctrl_rx));
    }
}

pub fn worker_thread(returns: Sender<(Analysis, Board)>, messenger: Receiver<CtrlMsg>) {
    let mut best_so_far = 0.;
    let mut iter = 0;
    let mut successes: i32 = 0;

    loop {
        iter += 1;
        match messenger.try_recv() {
            Ok(CtrlMsg::Halt(time)) => {
                thread::sleep(time::Duration::from_millis(time as u64));
                continue;
            }
            Ok(CtrlMsg::Kill) => {
                println!(
                    "reached {iter} iter and {successes} successes (ratio of {})",
                    successes as f32 / iter as f32
                );

                return;
            }
            Err(_) => {}
        }

        if let Ok(board) = Board::new_random() {
            if let Ok(analysis) = analyze(&board, 0, 1) {
                successes += 1;
                let fitness = analysis.compute_fitness(&board.map);
                if fitness > best_so_far {
                    best_so_far = fitness;
                    returns
                        .send((analysis, board))
                        .expect("unable to send best so far");
                }
            }
        }
    }
}

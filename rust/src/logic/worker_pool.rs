use std::{
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread::{self, available_parallelism, spawn},
    time::{self, Duration},
};

use rand::seq::IndexedRandom;
use sorted_vec::partial::ReverseSortedVec;

use crate::{
    api::main::Room,
    logic::{board::Board, creature::Creature, noise_reduction::asthetic_cleanup},
};

pub enum CtrlMsg {
    Kill,
    Halt(usize),
}

struct Worker {
    return_channel: mpsc::Receiver<Creature>,
    crtl_channel: mpsc::Sender<CtrlMsg>,
}

static G_WORKER: Mutex<VecDeque<Worker>> = Mutex::new(VecDeque::new());

pub fn get_new_room() -> Room {
    let mut ret = {
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


 
    ret.board = asthetic_cleanup(ret.board);
    ret.board.print(
        vec![]
    );
    ret.board.print(
        ret.analysis.routes[0][0]
            .solution
            .iter()
            .map(|e| e.1)
            .collect(),
    );
    println!("{:?}", ret.analysis);

    Room::Trial(ret)
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

    // while ret.len()
    //     < (available_parallelism()
    //         .expect("couldnt get available parallelism")
    //         .get()
    //         - 3)
    while ret.len() < 1
    {
        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (ret_tx, ret_rx) = mpsc::channel();
        ret.push_back(Worker {
            return_channel: ret_rx,
            crtl_channel: ctrl_tx,
        });

        spawn(|| worker_thread(ret_tx, ctrl_rx));
    }
}

pub fn worker_thread(returns: Sender<Creature>, messenger: Receiver<CtrlMsg>) {
    let mut rng = rand::rng();

    let mut population: ReverseSortedVec<Creature> = ReverseSortedVec::new();

    let mut generations = 1;

    let mut best_so_far = 0.;
    let mut iter = 0;
    let mut successes: i32 = 0;

    loop {
        match messenger.try_recv() {
            Ok(CtrlMsg::Halt(time)) => {
                println!("halting");
                thread::sleep(time::Duration::from_millis(time as u64));
                println!("resuming");
                continue;
            }
            Ok(CtrlMsg::Kill) => {
                println!("reached {generations} generations with {iter} iter and {successes} successes {}", successes as f32 / iter as f32);

                return;
            }
            Err(_) => {}
        }

        if population.len() < generations * 3 {
        iter += 1;

            if let Some(new_creature) = Creature::board_to_creature(Board::new_random()) {
                population.insert(new_creature);
                successes+=1;
                if population[0].fitness > best_so_far {
                    best_so_far = population[0].fitness;
                    returns
                        .send(population[0].clone())
                        .expect("unable to send best so far");
                }
            }
        } else if population.len() < generations * 9 {

            if let Some(new_creature) = (population[0..generations * 2].choose(&mut rng).unwrap()).mutate(0.3) {
        iter += 1;

                population.insert(new_creature);
                successes+=1;

                if population[0].fitness > best_so_far {
                    best_so_far = population[0].fitness;
                    returns
                        .send(population[0].clone())
                        .expect("unable to send best so far");
                }
            }
        } else {
            population = population[0..generations * 2]
                .into_iter()
                .cloned()
                .collect();

            let mean_fitness =
                population.iter().map(|cre| (*cre).fitness).sum::<f32>() / population.len() as f32;

            population = population
                .iter()
                .filter(|e| e.fitness > mean_fitness)
                .cloned()
                .collect();

            generations += 1;
        }
    }
}

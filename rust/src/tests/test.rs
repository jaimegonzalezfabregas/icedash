#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc,
        thread::{self, spawn},
        time::Duration,
    };

    use crate::{api::main::BoardDescription, logic::{
            noise_reduction::asthetic_cleanup,
            worker_pool::worker_thread,
        }};

    #[test]
    fn test_bench() {
        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (ret_tx, ret_rx) = mpsc::channel();

        let join_handdle = spawn(|| worker_thread(ret_tx, ctrl_rx, BoardDescription{
            size_range_min: 7,
            size_range_max: 15,
            weak_walls_percentage_min: 0,
            weak_walls_percentage_max: 0,
            pilars_percentage_min: 0,
            pilars_percentage_max: 0,
            box_percentage_min: 0,
            box_percentage_max: 0,
            vignet_percentage_min: 0,
            vignet_percentage_max: 20,
        }));

        thread::sleep(Duration::from_secs(5));

        ctrl_tx
            .send(crate::logic::worker_pool::CtrlMsg::Kill)
            .expect("couldnt stop worker");

        println!("waiting for worker stop");

        let mut ret = ret_rx
            .recv_timeout(Duration::from_millis(500))
            .expect("Worker Thread did not return any boards");

        if let Some(last) = ret_rx.try_iter().last() {
            ret = last;
        }

        let (analysis, board) = ret;

        let board = asthetic_cleanup(board, &analysis, 0);
        board.print(vec![]);
        board.print(analysis.routes[0][0].solution.iter().map(|e| e.1).collect());
        join_handdle.join().expect("worker panicked");

    }

}

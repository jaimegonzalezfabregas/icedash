#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc,
        thread::{self, spawn},
        time::Duration,
    };

    use crate::{
        api::main::{Direction, Pos},
        logic::{
            board::Board, noise_reduction::asthetic_cleanup, solver::analyze, tile_map::TileMap,
            worker_pool::worker_thread,
        },
    };

    #[test]
    fn test_bench() {
        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (ret_tx, ret_rx) = mpsc::channel();

        let join_handdle = spawn(|| worker_thread(ret_tx, ctrl_rx));

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

        ret.board = asthetic_cleanup(ret.board);
        ret.board.print(vec![]);
        ret.board.print(
            ret.analysis.routes[0][0]
                .solution
                .iter()
                .map(|e| e.1)
                .collect(),
        );
        join_handdle.join().expect("worker panicked");

        ret.analysis.print();
        println!("mutated {:?} times", ret.mutation_count);
    }

    #[test]

    fn analyze_test() {
        let board = Board {
            map: TileMap::from_print(
                "# # # E # # # # # # # 
# # #   # w # # # # # 
#         # # # # # # 
#         # # # # # # 
# #   b   # # # # # # 
# # #     # # # # # # 
#         #     # # # 
G         #     # # # 
#     w # # # # # # # 
# # # # # # # # # # # 
# # # # # # # # # # # ",
            ),
            start: Pos { x: 3, y: 0 },
            end: Pos { x: 0, y: 7 },
            start_direction: Direction::South,
            end_direction: Direction::West,
        };
        board.print(vec![]);
        let analysis = analyze(&board).expect("err");

        board.print(
            analysis.routes[0][0]
                .solution
                .iter()
                .map(|(_, b)| b)
                .cloned()
                .collect(),
        );

        analysis.print();
    }
}

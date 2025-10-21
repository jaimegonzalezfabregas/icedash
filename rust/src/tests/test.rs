#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc,
        thread::{self, spawn},
        time::Duration,
    };

    use crate::{
        api::main::{Direction, Pos, Tile},
        logic::{
            board::Board, noise_reduction::asthetic_cleanup, solver::analyze, tile_map::TileMap,
            worker_pool::worker_thread,
        },
    };

    #[test]
    fn test_bench() {
        let (ctrl_tx, ctrl_rx) = mpsc::channel();
        let (ret_tx, ret_rx) = mpsc::channel();

        spawn(|| worker_thread(ret_tx, ctrl_rx));

        thread::sleep(Duration::from_secs(5));

        ctrl_tx
            .send(crate::logic::worker_pool::CtrlMsg::Kill)
            .expect("couldnt stop worker");

        thread::sleep(Duration::from_secs(1));

        let mut ret = ret_rx
            .recv_timeout(Duration::from_millis(500))
            .expect("Worker Thread did not return any boards");

        if let Some(last) = ret_rx.try_iter().last() {
            ret = last;
        }

        ret.board = asthetic_cleanup(ret.board);
        ret.board.print(
            ret.analysis.routes[0][0]
                .solution
                .iter()
                .map(|e| e.1)
                .collect(),
        );
        println!("{:?}", ret.analysis);
        println!("mutated {:?} times", ret.mutation_count);
    }

    // #[test]

    // fn analyze_test() {
    //     let board = Board {
    //         map: TileMap(vec![
    //             vec![Tile::Wall, Tile::Entrance, Tile::Wall],
    //             vec![Tile::Gate, Tile::Ice, Tile::Wall],
    //             vec![Tile::Wall, Tile::Wall, Tile::Wall],
    //         ]),
    //         start: Pos { x: 1, y: 0 },
    //         end: Pos { x: 0, y: 1 },
    //         start_direction: Direction::South,
    //         end_direction: Direction::East,
    //     };
    //     let analysis = analyze(&board).expect("err");

    //     board.print(
    //         analysis.routes[0][0]
    //             .solution
    //             .iter()
    //             .map(|(_, b)| b)
    //             .cloned()
    //             .collect(),
    //     );
    // }
}

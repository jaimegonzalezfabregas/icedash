#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc,
        thread::{self, spawn},
        time::Duration,
    };

    use crate::logic::{noise_reduction::asthetic_cleanup, worker_pool::worker_thread};

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

//     #[test]
//     fn room_detection(){

//     //   "
//     //     # # # # # # # # 
//     //     # # #   # #   # 
//     //     #   #       # # 
//     //     # # #         E 
//     //     #       #   # # 
//     //     # .   #   # # # 
//     //     # . #       # # 
//     //     #             # 
//     //     #       #     # 
//     //     #   #         # 
//     //     # #           # 
//     //     # # #         # 
//     //     #             # 
//     //     # # # # G # # # 
//     //     "

//         let board = Board{
//             map: TileMap( vec![
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Ice,Tile::Wall,Tile::Wall,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Entrance ],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,Tile::Ice,Tile::Wall,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Ice, Tile::Wall,Tile::Ice,Tile::Wall,Tile::Wall,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice, Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Ice,Tile::Wall,],
//                 vec![ Tile::Wall,Tile::Wall,Tile::Wall,Tile::Wall,Tile::Gate, Tile::Wall,Tile::Wall,Tile::Wall,],
//             ]),
//             start: Pos { x: 7, y: 3 },
//             end: Pos{ x: 4, y: 13 },
//             reset_pos:  Pos { x: 0, y: 0 },
//             start_direction: Direction::West,
//             end_direction: Direction::South,
//         };

//         board.print(vec![]);

//         assert!(!has_rooms(&board));


// // "# # # # # # # # 
// // # # #   # #   # 
// // #   #       # # 
// // # # #         E 
// // #       #   # # 
// // # .   #   # # # 
// // # . #       # # 
// // #             # 
// // #       #     # 
// // #   #         # 
// // # #           # 
// // # # #         # 
// // #             # 
// // # # # # G # # # "
//     }
}

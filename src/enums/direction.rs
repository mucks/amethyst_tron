use crate::enums::MoveEvent;

pub enum Direction {
    Forward,  //z--
    Backward, //z++
    Left,     //x--
    Right,    //x++
}

impl Direction {
    pub fn apply_move_event(&self, move_event: MoveEvent) -> Direction {
        use Direction::*;
        use MoveEvent::*;

        match &self {
            Forward => match move_event {
                LeftClick => Left,
                RightClick => Right,
            },
            Backward => match move_event {
                LeftClick => Right,
                RightClick => Left,
            },
            Left => match move_event {
                LeftClick => Backward,
                RightClick => Forward,
            },
            Right => match move_event {
                LeftClick => Forward,
                RightClick => Backward,
            },
        }
    }
}

use bevy::utils::{HashMap, HashSet};
use hex2d::{Angle, Coordinate, Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    TurnLeft,
    TurnRight,
    StepForward,
}

impl Move {
    pub fn apply(&self, pos: Position) -> Position {
        match self {
            Move::TurnLeft => pos + Angle::Left,
            Move::TurnRight => pos + Angle::Right,
            Move::StepForward => pos + Coordinate::from(pos.dir),
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            Move::TurnLeft => 1,
            Move::TurnRight => 1,
            Move::StepForward => 1,
        }
    }
}

fn retrace(steps: &mut HashMap<Position, (Position, Move)>, mut current: Position) -> Vec<Move> {
    let mut path = Vec::default();
    loop {
        if let Some((pos, mov)) = steps.remove(&current) {
            current = pos;
            path.push(mov);
        } else {
            break;
        }
    }
    path
}

const MOVES: [Move; 3] = [Move::TurnLeft, Move::TurnRight, Move::StepForward];

pub fn a_star(
    start: Position,
    goal: Coordinate,
    valid_tiles: HashSet<Coordinate>,
) -> Option<Vec<Move>> {
    // set of discovered nodes which need to be expanded
    let mut to_search = HashSet::default();
    to_search.insert(start);

    // for each Position, stores the node immediately preceding it on the cheapest path from start
    let mut came_from = HashMap::<Position, (Position, Move)>::default();

    // the cheapest currently-known path from start to <Position>
    let mut g_score = HashMap::<Position, i32>::default();
    g_score.insert(start, 0);

    // estimated cheapest cost from start to goal via <Position>
    let mut f_score = HashMap::<Position, i32>::default();
    f_score.insert(start, start.coord.distance(goal));

    loop {
        if to_search.is_empty() {
            break;
        }

        let current = *to_search.iter().min_by_key(|x| f_score[x]).unwrap();

        if current.coord == goal {
            return Some(retrace(&mut came_from, current));
        }

        to_search.remove(&current);

        for mov in MOVES {
            let next_pos = mov.apply(current);
            if !valid_tiles.contains(&next_pos.coord) {
                continue;
            }

            let tentative_g_score = g_score[&current] + mov.cost();
            if g_score
                .get(&next_pos)
                .map_or(true, |x| tentative_g_score < *x)
            {
                came_from.insert(next_pos, (current, mov));
                g_score.insert(next_pos, tentative_g_score);
                f_score.insert(next_pos, tentative_g_score + next_pos.coord.distance(goal));

                to_search.insert(next_pos);
            }
        }
    }

    None
}

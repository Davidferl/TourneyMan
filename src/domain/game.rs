use crate::domain::team::Team;

pub struct Game {
    team1: Team,
    team2: Team,
    // state: MatchState,
    winner: Option<Team>,
}
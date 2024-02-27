use crate::domain::game::Game;

pub struct Tournament {
    name: String,
    teams: Vec<Game>,
}

impl Tournament {
    pub fn new(name: String, number_of_teams: usize) -> Tournament {
        Tournament {
            name,
            teams: Vec::with_capacity(number_of_teams),
        }
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }
}
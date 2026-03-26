/*
Strategy - Generate moves - consuming: History of prev rounds, Current Score, Round No. and produce - a move (Cooperate OR Defect)

Core - Start a Tournament, with N rounds/games, Scoreboard, Move History. For each game 2 players/strategy will provide their move. Evaluate Game result - Who gain, update score, update history. Check if tournament ends. If yes END If no REPEAT.

Simulation -> Tournament/Generation -> Game -> Dilemma

1 Simulation -> Min 1 to max_gens Tournaments/Generations.
1 Tournament/Generation -> Total Players(Total Player - 1)/2 Games.
1 Game -> 150 to 250 Dilemmas.

simulate fn takes Simulation, population, payoff -> create a tournament, run tournament, update current_gen counter, update evolutionary_history, check end_simulation fn(if true exit), change_population fn, loop to create tournament.

run_tournament takes population, payoff and TournamentReport

Player:
  uuid
  strategy

Strategy:
  strat_id
  name
  decision fn

Simulation:
  Population: vector of Player
  Payoff
  max_gens
  current_gen
  evolutionary_history
*/

/*
 * TO DO
 * find a way to solve strategy ID, allotment (Enum solution with ID from 0..N)
 *  Then implement the CLI to take a cvs or json file to create initial population and start simulation and return results in a cvs preferably or JSON.
 */

pub mod game;
pub mod population;
pub mod simulation;
mod tournament;

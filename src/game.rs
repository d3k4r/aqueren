extern crate rand;

use types::*;

pub fn new_actions() -> Vec<Action> {
    let actions: Vec<Action> = Vec::new();
    actions
}

pub fn new_game() -> Game {
    let all_tiles = (0..TILES).map(|i| {
        let row = i / COLS;
        let col = i % ROWS;
        if let Some(tile) = Tile::new(row, col) {
            tile
        } else {
            panic!("Attempted to create invalid tile ({},{})", row, col)
        }
        }).collect();
    let (starting_tiles, remaining_tiles) = choose_tiles(all_tiles, PLAYERS);
    let players = new_players(remaining_tiles);
    let slots = initial_slots(starting_tiles);
    Game {
        board: Board { slots: slots },
        players: players, 
        turn: PlayerId::One, 
        merge_decision: None
    }
}

fn choose_tiles(tiles: Vec<Tile>, count: u8) -> (Vec<Tile>, Vec<Tile>) {
    let mut remaining_tiles = tiles;
    let mut random_tiles = Vec::new();
    for _ in 0..count {
        let random_index = rand::random::<usize>() % remaining_tiles.len();
        random_tiles.push(remaining_tiles.remove(random_index));
    }
    (random_tiles, remaining_tiles)
}

fn new_players(tiles: Vec<Tile>) -> Vec<Player>{
    let init_players: Vec<Player> = Vec::new();
    let (players, _) = (0..PLAYERS)
        .fold( (init_players, tiles), | (mut v, remaining), i | {
            let (player_tiles, new_remaining) = choose_tiles(remaining, 6);
            let player = new_player(PlayerId::new(i+1).unwrap(), player_tiles);
            v.push(player);
            (v, new_remaining)
        });
    players
}

pub fn new_player(id: PlayerId, tiles: Vec<Tile>) -> Player {
    Player { id: id, money: 6000, shares: empty_shares(), tiles: tiles }
}

fn empty_shares() -> PlayerShares {
    PlayerShares { luxor: 0, tower: 0, american: 0, festival: 0, worldwide: 0, continental: 0, imperial: 0 }
}

pub fn initial_slots(starting_tiles: Vec<Tile>) -> Vec<Slot> {
    (0..ROWS).flat_map(|row| -> Vec<Slot> {
        (0..COLS).map(|col| {
            Slot { 
                row: row, 
                col: col, 
                hotel: None, 
                has_tile: has_tile_on_slot(&starting_tiles, row, col)
            } 
        }).collect()
    }).collect()
}

fn has_tile_on_slot(tiles: & Vec<Tile>, row: u8, col: u8) -> bool {
    tiles.iter().any(|t| t.row() == row && t.col() == col)
}

pub fn compute_state(last_state: &Game, actions: &Vec<Action>) -> TurnResult {
    actions.iter().fold(TurnResult::Success(last_state.clone()), |last_result, action| {
        match last_result {
            TurnResult::Success(game) => { play_turn(&game, action) }
            TurnResult::Error(_) => { last_result }
        }
    })
}

pub fn play_turn(game: &Game, action: &Action) -> TurnResult {
    match *action {
        Action::PlaceTile { ref player, ref tile } => {
            place_tile(game, player.clone(), tile)
        }
        Action::BuyStocks { ref player, ref hotel1, ref hotel2, ref hotel3 } => {
            buy_stocks(game, player.clone(), hotel1.clone(), hotel2.clone(), hotel3.clone())
        }
        _ => panic!(format!("I don't know how to play a turn with action {:?}", action))
    }
}

fn place_tile(game: &Game, player_id: PlayerId, tile: &Tile) -> TurnResult {
    if !game_player_has_turn(game, player_id.clone()) {
        let error_msg = format!("Error placing tile: player {:?} does not have turn", player_id);
        return TurnResult::Error(error_msg)
    }
    if !game_player_has_tile(game, player_id.clone(), tile) {
        let error_msg = format!("Error placing tile: player {:?} does not have tile {:?}", player_id, *tile);
        return TurnResult::Error(error_msg)
    }
    let new_players = remove_tile_from_player(game.players.clone(), player_id.clone(), tile);
    TurnResult::Success(Game {
        board: place_tile_on_board(&game.board, &tile),
        players: new_players,
        turn: game.turn.clone(),
        merge_decision: game.merge_decision.clone()
    })
}

fn remove_tile_from_player(mut players: Vec<Player>, player_id: PlayerId, tile: &Tile) -> Vec<Player> {
    let player_index = players.iter().position(|p| p.id == player_id).unwrap();
    let tile_index = players[player_index].tiles.iter().position(|t| *t == *tile).unwrap();
    players[player_index].tiles.remove(tile_index);
    players
}

fn game_player_has_turn(game: &Game, player: PlayerId) -> bool {
    return game.turn == player
}

fn game_player_has_tile(game: &Game, player: PlayerId, tile: &Tile) -> bool {
    game.players.iter()
        .find(|p| p.id == player)
        .map_or(false, |p| player_has_tile(p, tile))
}

fn player_has_tile(player: &Player, tile: &Tile) -> bool {
    player.tiles.iter().any(|t| *t == *tile)
}

fn place_tile_on_board(board: &Board, tile: &Tile) -> Board {
    let slots = board.slots
        .iter()
        .map(|s| {
            if s.row == tile.row() && s.col == tile.col() {
                Slot { row: s.row, col: s.col, has_tile: true, hotel: s.hotel.clone() }
            } else {
                s.clone()
            }
        })
        .collect();
    Board { slots: slots }
}

fn buy_stocks(game: &Game, player: PlayerId, hotel1: Option<Hotel>, hotel2: Option<Hotel>, hotel3: Option<Hotel>) -> TurnResult {
    let new_players: Vec<Player> = game.players
        .iter()
        .map(|p| {
            if p.id == player {
                player_buy_stocks(&game, &p, hotel1.clone(), hotel2.clone(), hotel3.clone())
            } else {
                p.clone()
            }
        })
        .collect();
    TurnResult::Success(Game {
        board: game.board.clone(),
        players: new_players,
        turn: game.turn.clone(),
        merge_decision: game.merge_decision.clone()
    })
}

fn player_buy_stocks(game: &Game, player: &Player, hotel1: Option<Hotel>, hotel2: Option<Hotel>, hotel3: Option<Hotel>) -> Player {
  let new_shares = vec![hotel1.clone(), hotel2.clone(), hotel3.clone()]
        .iter()
        .fold(player.shares.clone(), | shares, hotel: &Option<Hotel> | {
            match *hotel {
                Some(ref h) => { add_share(shares, h.clone()) }
                None => { shares }
            }
        });
  let total_cost = share_price(game, hotel1) + share_price(game, hotel2) + share_price(game, hotel3);
  let money_after = player.money - total_cost;
  Player { 
      id: player.id.clone(), 
      money: money_after, 
      shares: new_shares, 
      tiles: player.tiles.clone()
  }
}

fn share_price(game: &Game, hotel: Option<Hotel>) -> i32 {
  hotel.map(|h| stock_price(h.clone(), hotel_chain_size(game, h.clone()))).unwrap_or(0)
}

fn add_share(shares: PlayerShares, hotel: Hotel) -> PlayerShares {
    let mut new_shares = PlayerShares {
        luxor: shares.luxor,
        tower: shares.tower,
        american: shares.american,
        festival: shares.festival,
        worldwide: shares.worldwide,
        continental: shares.continental,
        imperial: shares.imperial
    };
    match hotel {
        Hotel::Tower =>       { new_shares.tower += 1 }
        Hotel::Luxor =>       { new_shares.luxor += 1 }
        Hotel::American =>    { new_shares.american += 1 }
        Hotel::Worldwide =>   { new_shares.worldwide += 1 }
        Hotel::Festival =>    { new_shares.festival += 1 }
        Hotel::Imperial =>    { new_shares.imperial += 1 }
        Hotel::Continental => { new_shares.continental += 1 }
    };
    new_shares
}

fn hotel_chain_size(game: &Game, hotel: Hotel) -> u8 {
  2
}

fn stock_price(hotel: Hotel, num_tiles: u8) -> i32 {
    base_price(hotel) + 100 * price_level(num_tiles) as i32
}

fn base_price(hotel: Hotel) -> i32 {
    let cheap = 200;
    let medium = 300;
    let spendy = 400;
    match hotel {
        Hotel::Tower =>       { cheap }
        Hotel::Luxor =>       { cheap }
        Hotel::American =>    { medium }
        Hotel::Worldwide =>   { medium }
        Hotel::Festival =>    { medium }
        Hotel::Imperial =>    { spendy }
        Hotel::Continental => { spendy }
    }
}

fn price_level(num_tiles: u8) -> u8 {
  if num_tiles == 2 {
      0
  } else if num_tiles == 3 {
      1
  } else if num_tiles == 4 {
      2
  } else if num_tiles == 5 {
      3
  } else if num_tiles >= 6 && num_tiles <= 10 {
      4
  } else if num_tiles >= 11 && num_tiles <= 20 {
      5
  } else if num_tiles >= 21 && num_tiles <= 30 {
      6
  } else if num_tiles >= 31 && num_tiles <= 40 {
      7
  } else { // >= 41
      8
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;

    type BoardTiles = [[i32; COLS as usize]; ROWS as usize];
    type PlayerTiles = [[(i32,i32); 6]; PLAYERS as usize];

    #[test]
    fn players_start_with_six_tiles() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.tiles.len(), 6);
        }
    }

    #[test]
    fn players_start_with_6000_in_cash() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.money, 6000);
        }
    }

    #[test]
    fn players_start_with_zero_shares() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.shares.luxor, 0);
            assert_eq!(player.shares.tower, 0);
            assert_eq!(player.shares.american, 0);
            assert_eq!(player.shares.festival, 0);
            assert_eq!(player.shares.worldwide, 0);
            assert_eq!(player.shares.continental, 0);
            assert_eq!(player.shares.imperial, 0);
        }
    }

    #[test]
    fn game_starts_with_four_placed_tiles() {
        let game = new_game();
        let board_tiles = game.board.slots.iter().filter(|s| s.has_tile).count();
        assert_eq!(board_tiles, 4)
    }

    #[test]
    fn placing_a_tile_adds_tile_to_board() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let end_tiles =   [[0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let tile_to_place = Tile::new(0,2).unwrap();
        let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place };
        match play_turn(&game, &action) {
            TurnResult::Success(game_after) => {
                assert_boards_equal(&tiles_to_board(&end_tiles), &game_after.board);
            }
            _ => {
                panic!("Placing a valid tile failed")
            }
        }
    }

    #[test]
    fn placing_a_tile_fails_if_player_does_not_have_tile() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let tile_to_place = Tile::new(5,11).unwrap();
        let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place };
        match play_turn(&game, &action) {
            TurnResult::Success(_) => {
                panic!("Placing a tile succeeded when player did not have tile")
            }
            _ => {}
        }
    }
    
    #[test]
    fn placing_a_tile_fails_if_player_does_not_have_turn() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let tile_to_place = Tile::new(1,4).unwrap();
        let action = Action::PlaceTile { player: PlayerId::Two, tile: tile_to_place };
        match play_turn(&game, &action) {
            TurnResult::Success(_) => {
                panic!("Placing a tile succeeded when player did not have turn")
            }
            _ => {}
        }
    }

    #[test]
    fn placing_a_tile_removes_tile_from_player() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let tile_to_place = Tile::new(0,2).unwrap();
        let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place.clone() };
        match play_turn(&game, &action) {
            TurnResult::Success(game_after) => {
                let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
                let has_tile = player.tiles.iter().any(|t| *t == tile_to_place);
                assert!(!has_tile, "Placed tile was still on player")
            }
            _ => {
                panic!("Placing a valid tile failed")
            }
        }
    }

    #[test]
    fn buying_stocks_reduces_player_money() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let action = Action::BuyStocks { player: PlayerId::One, hotel1: Some(Hotel::Luxor), hotel2: None, hotel3: None };
        match play_turn(&game, &action) {
            TurnResult::Success(game_after) => {
                let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
                let expected_money = 5800;
                let error_msg = format!("After buying stocks, expected player to have {:?} dollars but player had {:?} dollars", expected_money, player.money);
                assert!(player.money == expected_money, error_msg)
            }
            _ => {
                panic!("Failed buying stocks")
            }
        }
    }

    #[test]
    fn buying_stocks_gives_player_bought_stocks() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let action = Action::BuyStocks {
            player: PlayerId::One, 
            hotel1: Some(Hotel::Luxor), 
            hotel2: Some(Hotel::Luxor), 
            hotel3: Some(Hotel::Imperial)
        };
        match play_turn(&game, &action) {
            TurnResult::Success(game_after) => {
                let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
                let expected_shares = PlayerShares {
                    luxor: 2,
                    tower: 0,
                    american: 0,
                    festival: 0,
                    worldwide: 0,
                    continental: 0,
                    imperial: 1
                };
                let error_msg = format!("After buying stocks, expected player to have shares\n{:?} but player had shares\n{:?}", expected_shares, player.shares);
                assert!(player.shares == expected_shares, error_msg)
            }
            _ => {
                panic!("Failed buying stocks")
            }
        }
    }

    fn new_game_with_tiles(start_tiles: BoardTiles, player_tiles: PlayerTiles) -> Game {
        let (starting_tiles, _) = board_tiles_to_tiles(&start_tiles);
        let players = player_tiles
            .iter()
            .enumerate()
            .map(|(i, tiles)| {
                let _tiles = tiles
                    .iter()
                    .map(|&(r,c)| Tile::new(r as u8, c as u8).unwrap() )
                    .collect();
                new_player(PlayerId::new((i+1) as u8).unwrap(), _tiles)
            })
            .collect();
        let slots = initial_slots(starting_tiles);
        Game {
            board: Board { slots: slots },
            players: players, 
            turn: PlayerId::One, 
            merge_decision: None
        }
    }

    fn board_tiles_to_tiles(tiles: &BoardTiles) -> (Vec<Tile>, Vec<Tile>) {
        let mut chosen = Vec::new();
        let mut others = Vec::new();
        for row in 0..tiles.len() {
            for col in 0..tiles[0].len() {
                let tile = Tile::new(row as u8, col as u8).unwrap();
                if tiles[row][col] == 1 {
                    chosen.push(tile)
                } else {
                    others.push(tile)
                }
            }
        }
        (chosen, others)
    }

    fn assert_boards_equal(expected: &Board, actual: &Board) {
        let are_equal = expected.slots
            .iter()
            .zip(actual.slots.iter())
            .all(|(left, right)| *left == *right );
        let error_msg = format!("\n\nBoard did not have expected tiles.\n\nExpected tiles:\n{}\n\nActual tiles:\n{}\n\n", print_board(&expected), print_board(&actual));
        assert!(are_equal, error_msg);
    }

    fn tiles_to_board(tiles: &BoardTiles) -> Board {
        let board_tiles = tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                let slots: Vec<Slot> = row_tiles
                    .iter()
                    .enumerate()
                    .map(|(col, val): (usize, &i32)| {
                        let has_tile = *val == 1;
                        Slot { row: row as u8, col: col as u8, has_tile: has_tile, hotel: None }
                    })
                    .collect();
                slots
            }).collect();
        Board { slots: board_tiles }
    }

    fn row_to_char<'a>(row: u8) -> &'a str {
        let mapping = ["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        mapping[row as usize]
    }

    fn print_tile(slot: &Slot) -> String{
        format!("{}{}", row_to_char(slot.row), slot.col + 1)
    }

    fn print_board(board: &Board) -> String {
        let mut string = String::new();
        string.push_str("   1  2  3  4  5  6  7  8  9  10 11 12\n");
        for row in board.slots.chunks(COLS as usize) {
            let row_char = row_to_char(row[0].row);
            string.push_str(&format!("{}  ", row_char));
            for slot in row {
                string.push_str(&format!("{:<2} ", if slot.has_tile { '\u{25FC}' } else { '\u{25FB}' }));
            }
            string.push_str(&format!("{}\n", row_char));
        }
        string.push_str("   1  2  3  4  5  6  7  8  9  10 11 12");
        string
    }
}

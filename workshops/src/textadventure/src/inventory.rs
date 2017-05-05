use players;
use players::Player;
use players::Players;
use players::ExplorerData;
use players::GnomeData;
use players::LeprechaunData;
use players::Direction;
use board;
use board::Board;
use board::Position;
use std::io;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Thing {
    Food { name: String, energy: i32 },
    GoldCoin { denom: i32 },
    FakeCoin { denom: i32 },
    Teleporter,
    Torch,
    MagicWord { word: String, room: Position, wall: Direction },
    FakeWord { word: String }
}
impl Thing {
    pub fn is_food(&self) -> bool {
        match *self {
            Thing::Food { name: _, energy: _} => true,
            _ => false
        }
    }


    pub fn is_fake_coin(&self) -> bool {
        match *self {
            Thing::FakeCoin { denom: _ } => true,
            _ => false
        }
    }

    pub fn is_gold_coin(&self) -> bool {
        match *self {
            Thing::GoldCoin { denom: _} => true,
            _ => false
        }
    }
}

pub fn display_exp_things(exp: &ExplorerData) {
    println!("explorer has:");

    if exp.things.is_empty() {
        println!("nothing\n");
        return
    }

    for thing in exp.things.iter() {
        println!("{:?}", thing)
    }

    println!()
}

// TODO
pub fn exp_has_torch(exp: &ExplorerData) -> bool {
    exp.things.iter().any(|thing| Thing::Torch == *thing)
}

// TODO
pub fn all_magic_words(board: &Board) -> Vec<Thing> {
    vec![]
}

// TODO
pub fn all_fake_words(board: &Board) -> Vec<Thing> {
    vec![]
}

pub fn encounter_others(player: Player, others: &mut Players) -> Player {
    let _player: Player;

    match player {
        Player::Explorer(exp) => {
            _player = Player::Explorer(encounter_explorer(exp, others))
        },
        Player::Gnome(gnome) => {
            _player = Player::Gnome(encounter_gnome(gnome, others))
        },
        Player::Leprechaun(lep) => {
            _player = Player::Leprechaun(encounter_leprechaun(lep, others))
        }
    }

    _player
}

fn exchange_with_occupants<F>(pos: &Position, others: &mut Players, mut exchange: F)
where F: FnMut(Player) -> Player {
    let rotation = others.len() as i32;
    let mut index = 0;

    while index < rotation {
        match others.pop_front() {
            Some(other) => {
                index += 1;
                if players::is_occupant(&other, pos) {
                    let exchanged = exchange(other);
                    others.push_back(exchanged)
                } else {
                    others.push_back(other)
                }
            },
            None => panic!("missing other")
        }
    }
}

fn encounter_explorer(data: ExplorerData, others: &mut Players) -> ExplorerData {
    let mut exp = data;

    exchange_with_occupants(&players::get_exp_pos(&exp), others,
       |occupant: Player| {
           let _occupant : Player;

            match occupant {
                Player::Explorer(other) => _occupant = Player::Explorer(other),
                Player::Gnome(mut gnome) => {
                    shake_down(&mut gnome, &mut exp);
                    _occupant = Player::Gnome(gnome)
                },
                Player::Leprechaun(mut lep) => {
                    trick_or_treat(&mut lep, &mut exp);
                    _occupant = Player::Leprechaun(lep)
                }
            }

            _occupant
        }
    );

    exp
}

fn encounter_gnome(data: GnomeData, others: &mut Players) -> GnomeData {
    let mut gnome = data;

    exchange_with_occupants(&players::get_gnome_pos(&gnome), others,
       |occupant: Player| {
           let _occupant : Player;

            match occupant {
                Player::Explorer(mut exp) => {
                    shake_down(&mut gnome, &mut exp);
                    _occupant = Player::Explorer(exp)
                },
                Player::Gnome(other) => _occupant = Player::Gnome(other),
                Player::Leprechaun(lep) => _occupant = Player::Leprechaun(lep)
            }

            _occupant
        }
    );

    gnome
}

fn encounter_leprechaun(data: LeprechaunData, others: &mut Players) -> LeprechaunData {
    let mut lep = data;

    exchange_with_occupants(&players::get_lep_pos(&lep), others,
       |occupant: Player| {
           let _occupant : Player;

            match occupant {
                Player::Explorer(mut exp) => {
                    trick_or_treat(&mut lep, &mut exp);
                    _occupant = Player::Explorer(exp)
                },
                Player::Gnome(gnome) => _occupant = Player::Gnome(gnome),
                Player::Leprechaun(other) => _occupant = Player::Leprechaun(other)
            }

            _occupant
        }
    );

    lep
}

fn should_rob_exp(gnome: &mut GnomeData, exp: &mut ExplorerData) -> bool {
    gnome.things.iter().any(|thing| thing.is_fake_coin()) ||
        !exp.things.iter().any(|thing| thing.is_gold_coin())
}

fn ask_player_for_shake_down(gnome: &mut GnomeData, exp: &mut ExplorerData) -> Vec<Thing> {
    let mut input = String::new();
    let items : Vec<Thing>;
    let has_fake_coin = exp.things.iter().any(|thing| thing.is_fake_coin());
    if has_fake_coin {
        loop {
            println!("Enter letter command: Give gnome [E]verything, [G]old coin or [F]ake coin");

            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(why) => {
                    println!("Failed to read line: {:?}", why);
                    input.clear();
                }
            }

            match input.trim().to_uppercase().chars().nth(0) {
                Some(command) => {
                    match command {
                        'E' => {
                            items = exp.things.clone();
                            exp.things = vec![];
                            break
                        },
                        'G' => {
                            let itemPos = exp.things.iter()
                                                .position(|thing| thing.is_gold_coin())
                                                .expect("Gold coin isn't in exp's things!");
                            let item = exp.things.remove(itemPos);
                            items = vec![item];
                            break
                        },
                        'F' => {
                            let itemPos = exp.things.iter()
                                                .position(|thing| thing.is_fake_coin())
                                                .expect("Fake coin isn't in exp's things!");
                            let item = exp.things.remove(itemPos);
                            items = vec![item];
                            break
                        },
                        _ => println!("Invalid command")
                    }
                },
                None => println!("Ignoring leading whitespace")
            }
            input.clear()
        }
    } else {
        println!("Enter letter command: Give gnome [E]verything or [G]old coin");
        items = vec![]
    }
    items
}

// TODO
fn shake_down(gnome: &mut GnomeData, exp: &mut ExplorerData) {
    // if room_has_torch(&pos, board) || inventory::exp_has_torch(&exp) {
    //     let empty = pos_to_room(&pos, board).contents.is_empty();
    //
    //     while !empty {
    //         inventory::display_exp_things(&exp);
    //         display_room_contents(pos_to_room(&pos, board));
    //
    //         println!("Enter letter command: Pick up [F]ood [C]oins tele[P]orter [T]orch or [D]one");
    //
    //         match io::stdin().read_line(&mut input) {
    //             Ok(_) => (),
    //             Err(why) => {
    //                 println!("Failed to read line: {:?}", why);
    //                 input.clear();
    //                 continue
    //             }
    //         }
    //
    //         match input.trim().to_uppercase().chars().nth(0) {
    //             Some(command) => {
    //                 match command {
    //                     'F' => { exp_pick_up_food(&mut exp, board); exp_eat_food(&mut exp) },
    //                     'C' => exp_pick_up_coins(&mut exp, board),
    //                     'P' => exp_pick_up_teleporter(&mut exp, board),
    //                     'T' => exp_pick_up_torch(&mut exp, board),
    //                     'D' => break,
    //                     _ => println!("Invalid command")
    //                 }
    //             },
    //             None => println!("Ignoring leading whitespace")
    //         }
    //
    //         input.clear()
    //     }
    // } else {
    //     inventory::display_exp_things(&exp)
    // }
}

// TODO
fn trick_or_treat(lep: &mut LeprechaunData, exp: &mut ExplorerData) {
}

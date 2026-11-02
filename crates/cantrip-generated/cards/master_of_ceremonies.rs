// Master of Ceremonies
// status: planned
// requires: condition VotedFor
// At the beginning of your upkeep, each opponent chooses money, friends, or secrets. For each player who chose money, you and that player each create a Treasure token. For each player who chose friends, you and that player each create a 1/1 green and white Citizen creature token. For each player who chose secrets, you and that player each draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_6 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_9 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Citizen).name, token_words: token!(Citizen).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Citizen]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(17)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_11 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Citizen).name, token_words: token!(Citizen).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Citizen]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(17)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let card_1 = g.card_id(&CARD); let chosen_1: u8 = cantrip_mechanics::choose::option(g, player_1, asker.source, card_1, 0..3)?;
 each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
planned!("depends-on-an-instruction-with-a-gap" => g.begin_simultaneous());
let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_3 = Vec::new();
let count_3_0 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let def_3_0 = g.card_id(&token_4::CARD);
for player in players_3 {
tokens_3.extend(std::iter::repeat((def_3_0,player)).take(count_3_0 as usize));
} let words_3 = Words::NONE;
let created_3 = g.create_tokens_worded(&tokens_3, false, None, words_3)?;
let players_5 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() }; let mut tokens_5 = Vec::new();
let count_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_5_0 = g.card_id(&token_6::CARD);
for player in players_5 {
tokens_5.extend(std::iter::repeat((def_5_0,player)).take(count_5_0 as usize));
} let words_5 = Words::NONE;
let created_5 = g.create_tokens_worded(&tokens_5, false, None, words_5)?;
planned!("depends-on-an-instruction-with-a-gap" => g.end_simultaneous());
g.end_sentence();
planned!("depends-on-an-instruction-with-a-gap" => g.begin_simultaneous());
let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_8 = Vec::new();
let count_8_0 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let def_8_0 = g.card_id(&token_9::CARD);
for player in players_8 {
tokens_8.extend(std::iter::repeat((def_8_0,player)).take(count_8_0 as usize));
} let words_8 = g.words(asker.source);
let created_8 = g.create_tokens_worded(&tokens_8, false, None, words_8)?;
let players_10 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() }; let mut tokens_10 = Vec::new();
let count_10_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_10_0 = g.card_id(&token_11::CARD);
for player in players_10 {
tokens_10.extend(std::iter::repeat((def_10_0,player)).take(count_10_0 as usize));
} let words_10 = g.words(asker.source);
let created_10 = g.create_tokens_worded(&tokens_10, false, None, words_10)?;
planned!("depends-on-an-instruction-with-a-gap" => g.end_simultaneous());
g.end_sentence();
planned!("depends-on-an-instruction-with-a-gap" => g.begin_simultaneous());
let amount_13 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let players_13 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_13 { g.draw(player, amount_13); }
let amount_14 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_14 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
for player in players_14 { g.draw(player, amount_14); }
planned!("depends-on-an-instruction-with-a-gap" => g.end_simultaneous());
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Master of Ceremonies", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rhino,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, each opponent chooses money, friends, or secrets. For each player who chose money, you and that player each create a Treasure token. For each player who chose friends, you and that player each create a 1/1 green and white Citizen creature token. For each player who chose secrets, you and that player each draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Money", "Friends", "Secrets"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_4::CARD,&token_6::CARD,&token_9::CARD,&token_11::CARD];

// Typhoid Mary, Fractured
// status: planned
// requires: modal announcement
// requires: modes chosen at random
// Whenever Typhoid Mary attacks, choose one at random. If you discarded a card this turn, you choose one instead.
// • Mary — Create a Treasure token.
// • Typhoid Mary — Draw a card.
// • Bloody Mary — Each opponent loses 2 life and you gain 2 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
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
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = Words::NONE;
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
fn ability_0_mode_0_open(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1_open(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.gain_life(player, amount_4); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_2_open(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
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
fn ability_0_mode_3_inner_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_5 = Vec::new();
let count_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_5_0 = g.card_id(&token_6::CARD);
for player in players_5 {
tokens_5.extend(std::iter::repeat((def_5_0,player)).take(count_5_0 as usize));
} let words_5 = Words::NONE;
let created_5 = g.create_tokens_worded(&tokens_5, false, None, words_5)?;
g.end_sentence();
 Ok(()) }
fn ability_0_mode_3_inner_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.draw(player, amount_7); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_3_inner_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_8 { g.lose_life(player, amount_8); }
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.gain_life(player, amount_9); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODE_3_INNER_MODES: ModalDef = planned_value!("modes chosen at random", modes(&[mode(ability_0_mode_3_inner_mode_0, "Mary — Create a Treasure token.").targets(&[]),mode(ability_0_mode_3_inner_mode_1, "Typhoid Mary — Draw a card.").targets(&[]),mode(ability_0_mode_3_inner_mode_2, "Bloody Mary — Each opponent loses 2 life and you gain 2 life.").targets(&[])]).choose(1, 1) => modes(&[mode(ability_0_mode_3_inner_mode_0, "Mary — Create a Treasure token.").targets(&[]),mode(ability_0_mode_3_inner_mode_1, "Typhoid Mary — Draw a card.").targets(&[]),mode(ability_0_mode_3_inner_mode_2, "Bloody Mary — Each opponent loses 2 life and you gain 2 life.").targets(&[])]).choose(1, 1).at_random());
fn ability_0_mode_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
 Ok(()) }
fn ability_0_mode_3_open(q: &dyn Query, asker: &Asker) -> bool { !((q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && !*undefined { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64)) }
#[allow(unused_variables)] fn ability_0_allowed(q: &dyn Query, asker: &Asker, mode: u8) -> bool { match mode { 0 => ability_0_mode_0_open(q, asker), 1 => ability_0_mode_1_open(q, asker), 2 => ability_0_mode_2_open(q, asker), 3 => ability_0_mode_3_open(q, asker), _ => true } }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Create tokens.").targets(&[]),mode(ability_0_mode_1, "Draw 1 cards.").targets(&[]),mode(ability_0_mode_2, "Lose 2 life. Gain 2 life.").targets(&[]),planned_value!("modal announcement", mode(ability_0_mode_3, "").targets(&[]) => mode(ability_0_mode_3, "").targets(&[]).then_modes(&ABILITY_0_MODE_3_INNER_MODES))]).choose(1, 1).allowed(ability_0_allowed);

pub static CARD: CardDef = CardDef { name: "Typhoid Mary, Fractured", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Mutant,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), &ABILITY_0_MODES, "Whenever Typhoid Mary attacks, choose one at random. If you discarded a card this turn, you choose one instead. Mary — Create a Treasure token; or Typhoid Mary — Draw a card; or Bloody Mary — Each opponent loses 2 life and you gain 2 life.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_6::CARD];

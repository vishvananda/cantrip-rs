// Gemstone Caverns
// status: planned
// requires: beginning the game on the battlefield with terms
// If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.
// {T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_stated(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; !(Some(asker.controller).is_some_and(|player: PlayerId| q.state().turn.starting == player)) } {
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
planned!("operation-begin-on-battlefield" => cantrip_mechanics::statics::begin_on_battlefield(g, asker.source, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[EnterFlag::EntersWithACounter(CounterType::LuckCounter)])?);
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Exile, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let objects_3 = { let q: &dyn Query = g; selected_2.clone() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend(selected_2.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_7.extend(hoisted_4.clone());
hoisted_8.extend(hoisted_5.clone());
hoisted_9.extend({ let q: &dyn Query = g; hoisted_6.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Luck))) > (0i64)) } {
let mana_players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_11 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_11 {
if mana_11 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), 1)?;
g.add_mana(player, colors[0].mana(), mana_11);}
}
g.end_sentence();
} else {
let mana_players_13 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_13_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_13 {
g.add_mana(player, Mana::C, mana_13_0);}
g.end_sentence();
}
 Ok(()) }
planned_item!("beginning-the-game-on-the-battlefield-with-terms" => opening_hand: Some(cantrip_mechanics::statics::opening_hand(ability_0_stated)));

pub static CARD: CardDef = CardDef { name: "Gemstone Caverns", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana), "{T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

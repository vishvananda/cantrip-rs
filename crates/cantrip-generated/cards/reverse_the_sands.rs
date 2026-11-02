// Reverse the Sands
// status: ready
// Redistribute any number of players' life totals. (Each of those players gets one life total back.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let totals_0: Vec<i32> = players_0.iter().map(|&p| g.state().player(p).life).collect(); let mut open_0 = players_0.clone(); let mut sets_0: Vec<(PlayerId, i32)> = Vec::new(); for &total in &totals_0 { let who = match open_0.as_slice() { [one] => Some(*one), _ => cantrip_mechanics::choose::player(g, asker.controller, asker.source, open_0.clone(), "GetLifeTotal", total.clamp(0, 65535) as u16)? }; if let Some(who) = who { open_0.retain(|&p| p != who); sets_0.push((who, total)); } } cantrip_api::instruction(g, |g| { for &(who, total) in &sets_0 { g.set_life_total(who, total); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reverse the Sands", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

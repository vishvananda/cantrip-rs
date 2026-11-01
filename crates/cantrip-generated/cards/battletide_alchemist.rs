// Battletide Alchemist
// status: planned
// requires: prevention a player may decline
// If a source would deal damage to a player, you may prevent X of that damage, where X is the number of Clerics you control.
#[allow(unused_imports)] use cantrip_api::*;
static INSTEAD_0: Question = Question::new("Instead", "", "If you don't, the event goes on as it was.", "Do this instead", "Don't");
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Player(p) if true) { return true; } }
 false }
#[allow(unused_variables)] fn ability_0_amount(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> u16 { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Cleric)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 }
planned_item!("prevention-a-player-may-decline" => preventions: PreventionDef { hint: "If a source would deal damage to a player, you may prevent X of that damage, where X is the number of Clerics you control.", applies: ability_0_applies, amount: Some(ability_0_amount), optional: Some(&INSTEAD_0) });

pub static CARD: CardDef = CardDef { name: "Battletide Alchemist", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kithkin,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

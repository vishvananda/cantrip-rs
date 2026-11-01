// Swans of Bryn Argoll
// status: planned
// requires: prevention with an effect besides
// Flying
// If a source would deal damage to this creature, prevent that damage. The source's controller draws cards equal to the damage prevented this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let amount_0 = { let q: &dyn Query = g; ((done_prevented as i64)).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(ObjectId(data[1])).filter(|o| *o != ObjectId::NONE).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, e: &cantrip_api::DamageEvent, prevented: u16) { let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }
planned_item!("prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If a source would deal damage to this creature, prevent that damage. The source's controller draws cards equal to the damage prevented this way.", applies: ability_1_applies, amount: None, then: Some(ability_1_then) });

pub static CARD: CardDef = CardDef { name: "Swans of Bryn Argoll", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::U)),Some((Mana::W, Mana::U)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

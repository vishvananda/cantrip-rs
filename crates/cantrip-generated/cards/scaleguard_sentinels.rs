// Scaleguard Sentinels
// status: planned
// requires: engine: no reveal cost
// As an additional cost to cast this spell, you may reveal a Dragon card from your hand.
// This creature enters with a +1/+1 counter on it if you revealed a Dragon card or controlled a Dragon as you cast this spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| (planned_value!("engine: no reveal cost", false => q.as_cast(asker, "AsCast { player: ControllerOf(EventSubject), action: Revealed(Subtype(Dragon)) }")) || q.history().casts().any(|c| (c.spell == asker.source || q.state().obj(c.spell).became == Some(asker.source)) && c.fact != 0))) } {
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
}
 Ok(()) }
#[allow(unused_variables)] fn kept_as_cast(q: &dyn Query, asker: &Asker) -> u32 { (i64::from((q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon))).collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| q.controller(o) == asker.controller))).max(0) as u32 }

pub static CARD: CardDef = CardDef { name: "Scaleguard Sentinels", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, cast_fact: Some(kept_as_cast), labels: &["Cast without revealing", "Reveal a Dragon card from your hand"], optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no reveal cost", Cost::NONE => Cost { reveal: Some((1, ability_0_0)), ..Cost::NONE })))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

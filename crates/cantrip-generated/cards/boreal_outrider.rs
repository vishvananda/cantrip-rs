// Boreal Outrider
// status: planned
// requires: object state SnowManaOfOwnColorSpent
// requires: operation ChangeSpell
// Whenever you cast a creature spell, if {S} of any of that spell's colors was spent to cast it, that creature enters with an additional +1/+1 counter on it. ({S} is mana from a snow source.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Creature) && subject.object().is_some_and(|candidate: ObjectId| planned_value!("object state SnowManaOfOwnColorSpent", false => q.state().obj(candidate).snow_spent.any_of(q.chars(candidate).colors))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| planned_value!("object state SnowManaOfOwnColorSpent", false => q.state().obj(candidate).snow_spent.any_of(q.chars(candidate).colors))) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { cantrip_mechanics::spell_changes::change(g, asker, spell, &[SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)]); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Boreal Outrider", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: true }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast a creature spell, if {S} of any of that spell's colors was spent to cast it, that creature enters with an additional +1/+1 counter on it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

// Elvish Refueler
// status: planned
// requires: an ability activated as though it had not been
// During your turn, as long as you haven't activated an exhaust ability this turn, you may activate exhaust abilities as though they haven't been activated.
// Exhaust — {1}{G}: Put a +1/+1 counter on this creature. (Activate each exhaust ability only once.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("an-ability-activated-as-though-it-had-not-been" => #[allow(unused_variables)] fn ability_0_0_0_abilities_0(q: &dyn Query, asker: &Asker, player: PlayerId, permanent: ObjectId, ability: AbilityRef) -> bool { let def = q.activated_ability(ability); ((Some(asker.controller) == Some(q.state().turn.active) && !(q.history().this_turn().iter().any(|r| matches!(r.event, Event::AbilityActivated { ability, player, .. } if Some(player) == Some(asker.controller) && q.activated_ability(ability).is_some_and(|d| d.exhaust)))))) && player == asker.controller && def.is_some_and(|d| d.exhaust) });
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
planned_item!("an-ability-activated-as-though-it-had-not-been" => activate_again: ability_0_0_0_abilities_0);

pub static CARD: CardDef = CardDef { name: "Elvish Refueler", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "Exhaust — {1}{G}: Put a +1/+1 counter on this creature.").targets(&[]).exhaust().once()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

// Kari Zev, Crew of Two
// status: planned
// requires: operation Conjure
// Menace, Haste
// Whenever Kari Zev attacks while you don't control a legendary Monkey, conjure a card named Ragavan, Nimble Pilferer onto the battlefield tapped and attacking. At the beginning of the next end step, if that card is on the battlefield, return it to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) && !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).supertypes.legendary && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Monkey)))).count() >= 1) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0)).get(0 as usize).copied().is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0)).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-conjure" => for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Ragavan, Nimble Pilferer"), { let q: &dyn Query = g; 1i64 }, ConjureTo::Entering(EventZone::Battlefield, ConjureSpot::Random, &[&[EnterFlag::EntersTapped, EnterFlag::EntersAttacking]]))?; });
g.end_sentence();
g.delayed_trigger(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kari Zev, Crew of Two", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pirate]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever Kari Zev attacks while you don't control a legendary Monkey, conjure a card named Ragavan, Nimble Pilferer onto the battlefield tapped and attacking. At the beginning of the next end step, if that card is on the battlefield, return it to its owner's hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "At the beginning of the next end step, if that card is on the battlefield, return it to its owner's hand.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

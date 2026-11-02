// Leori, Sparktouched Hunter
// status: planned
// requires: a choice of PlaneswalkerType outside its body
// Flying, vigilance
// Whenever Leori deals combat damage to a player, choose a planeswalker type. Until end of turn, whenever you activate an ability of a planeswalker of that type, copy that ability. You may choose new targets for the copies.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let subject = Target::Player(*player); let serial = match cantrip_mechanics::triggers::activated_ability_on_stack(q, event) { Some(Target::Ability(serial)) => serial, _ => 0 }; let data = [id.0, serial]; if def.is_some() && Some(asker.controller) == Some(*player) && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && (q.chars(id).is(CardType::Planeswalker) && planned_value!("a choice of PlaneswalkerType outside its body", None::<Subtype> => q.state().chosen_subtype(q.linked_to(asker))).is_some_and(|chosen| q.chars(id).has_subtype(chosen)))) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = Target::Ability(item.data[1]); g.copy_ability(t, copier_3, true)?;
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let choices_0 = Subtype::ALL.into_iter().filter(|&kind| kind.card_types() == TypeSet::of(&[CardType::Planeswalker])).collect::<Vec<Subtype>>(); let Answer::Subtype(chosen_0) = g.ask(player_0, RequestKind::ChooseSubtype { source: asker.source, choices: choices_0, purpose: WordUse::Chosen })? else { return Ok(()) };
planned!("a-choice-of-planeswalker-type-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Subtype(chosen_0)););
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Leori, Sparktouched Hunter", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Leori deals combat damage to a player, choose a planeswalker type. Until end of turn, whenever you activate an ability of a planeswalker of that type, copy that ability. You may choose new targets for the copies.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ABILITY_ACTIVATED.0), watch_2_event), watch_2, "Until end of turn, whenever you activate an ability of a planeswalker of that type, copy that ability.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

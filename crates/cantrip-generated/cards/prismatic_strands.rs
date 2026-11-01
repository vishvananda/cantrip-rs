// Prismatic Strands
// status: planned
// requires: a choice of Color outside its body
// Prevent all damage that sources of the color of your choice would deal this turn.
// Flashback—Tap an untapped white creature you control. (You may cast this card from your graveyard for its flashback cost. Then exile it.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && planned_value!("a choice of Color outside its body", None::<Color> => q.state().chosen_color(q.linked_to(asker))).is_some_and(|chosen| q.chars(source).colors.has(chosen)) && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: asker.source, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
planned!("a-choice-of-color-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Color(chosen_0)););
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.state().obj(id).tapped) && q.chars(id).colors.has(q.words(asker.source).color(Color::White)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
#[allow(unused_variables)] fn ability_1_require(q: &dyn Query, asker: &Asker) -> bool { (q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery)) }

pub static CARD: CardDef = CardDef { name: "Prismatic Strands", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all damage that sources of the color of your choice would deal this turn.", damage: "damage", of_it: None, applies: made_2_applies }], ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(tap_untapped(1, ability_1_3)), ZoneKind::Graveyard, "Flashback—Tap an untapped white creature you control.").only_from().exile_after().condition(ability_1_require)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

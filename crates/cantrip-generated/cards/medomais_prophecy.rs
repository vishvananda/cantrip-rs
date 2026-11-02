// Medomai's Prophecy
// status: planned
// requires: a choice of CardName outside its body
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I — Scry 2.
// II — Choose a card name.
// III — When you cast a spell with the chosen name for the first time this turn, draw two cards.
// IV — Look at the top card of each player's library.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_1)) = g.ask(player_1, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
planned!("a-choice-of-card-name-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Card(chosen_1)););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked_to(asker))).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen)) { return Some(Fired { subject, data }); } }
 None }
fn watch_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_5 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; g.look_at(player_5, &objects_5);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Medomai's Prophecy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Scry 2.").targets(&[]),chapter(2, ability_0_1, "II — Choose a card name.").targets(&[]),chapter(3, ability_0_2, "III — When you cast a spell with the chosen name for the first time this turn, draw two cards.").targets(&[]),chapter(4, ability_0_3, "IV — Look at the top card of each player's library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_3_event), watch_3, "When you cast a spell with the chosen name for the first time this turn, draw two cards.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

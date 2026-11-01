// Alchemist's Gambit
// status: planned
// requires: duration ExtraTurn
// requires: operation GameEffect
// Cleave {4}{U}{U}{R} (You may cast this spell for its cleave cost. If you do, remove the words in square brackets.)
// Take an extra turn after this one. During that turn, damage can't be prevented. [At the beginning of that turn's end step, you lose the game.]
// Exile Alchemist's Gambit.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let turn_0 = g.add_extra_turn({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
g.end_sentence();
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, planned_value!("duration ExtraTurn", Duration::Indefinite => Duration::ExtraTurn(turn_0)), GameEffect::DamageCantBePrevented));
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
fn delayed_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let turn_3 = g.add_extra_turn({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
g.end_sentence();
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, planned_value!("duration ExtraTurn", Duration::Indefinite => Duration::ExtraTurn(turn_3)), GameEffect::DamageCantBePrevented));
g.end_sentence();
if let Some(turn) = turn_3 { g.delayed_trigger(DelayedWhen::ExtraTurnStep { step: Step::End, turn }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_8 = g.exile_all(&objects_8);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.way == 1 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }

pub static CARD: CardDef = CardDef { name: "Alchemist's Gambit", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_6, "Cleave {4}{U}{U}{R}")], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::cleave::way(ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Cleave {4}{U}{U}{R}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

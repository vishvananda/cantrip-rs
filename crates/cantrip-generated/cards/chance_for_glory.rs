// Chance for Glory
// status: ready
// Creatures you control gain indestructible. Take an extra turn after this one. At the beginning of that turn's end step, you lose the game.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
let turn_1 = g.add_extra_turn({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
g.end_sentence();
if let Some(turn) = turn_1 { g.delayed_trigger(DelayedWhen::ExtraTurnStep { step: Step::End, turn }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chance for Glory", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "At the beginning of that turn's end step, you lose the game.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

// Display of Power
// status: ready
// This spell can't be copied.
// Copy any number of target instant and/or sorcery spells. You may choose new targets for the copies.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut open_0: Vec<ObjectId> = { let q: &dyn Query = g; q.state().stack.iter().filter(|i| i.kind == StackKind::Spell).map(|i| i.object).filter(|&id| (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| i.kind == StackKind::Spell) && (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).contains(&id))).collect::<Vec<ObjectId>>() }; while let Some(&first) = open_0.first() { let spell = if open_0.iter().all(|&o| o == first) { Some(first) } else { let mut distinct = open_0.clone(); distinct.sort(); distinct.dedup(); cantrip_mechanics::choose::card(g, player_0, asker.source, Purpose::CopyWhich, distinct)? }; let Some(spell) = spell else { break; }; if let Some(at) = open_0.iter().position(|&o| o == spell) { open_0.remove(at); } g.copy_spell(spell, player_0, true)?; }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Display of Power", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCopied]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

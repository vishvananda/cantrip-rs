// Shorecrasher Elemental
// status: planned
// requires: move arrival FaceDown
// {U}: Exile this creature, then return it to the battlefield face down under its owner's control.
// {1}: This creature gets +1/-1 or -1/+1 until end of turn.
// Megamorph {4}{U} (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its megamorph cost and put a +1/+1 counter on it.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let objects_1 = ({ let q: &dyn Query = g; ((moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_1 { let controller = g.state().obj(o).owner; moved.extend(planned_value!("move arrival FaceDown", None::<ObjectId> => g.put_onto_battlefield_face_down(o, controller, FaceDown::Bare, false)?)); } moved };
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let picked_2 = cantrip_mechanics::choose::option(g, player_2, asker.source, item.ability.card, 0..2)? - 0; let chosen_pt_0: (i64, i64) = [(1i64, -1i64), (-1i64, 1i64)][picked_2 as usize];
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::Pump((chosen_pt_0.0).clamp(-32768, 32767) as i16, (chosen_pt_0.1).clamp(-32768, 32767) as i16) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Shorecrasher Elemental", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{U}: Exile this creature, then return it to the battlefield face down under its owner's control.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{1}: This creature gets +1/-1 or -1/+1 until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::megamorph::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Megamorph {4}{U}")], labels: &["+1/-1", "-1/+1"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

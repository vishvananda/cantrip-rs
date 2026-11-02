// Expose the Culprit
// status: ready
// Choose one or both —
// • Turn target face-down creature face up.
// • Exile any number of face-up creatures you control with disguise in a face-down pile, shuffle that pile, then cloak them. (To cloak a card, put it onto the battlefield face down as a 2/2 creature with ward {2}. Turn it face up any time for its mana cost if it's a creature card.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.state().obj(id).face_down && q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.turn_face_up(object, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).face_down) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && (q.chars(id).keywords.has(Keyword::Disguise)))).collect::<Vec<ObjectId>>() }; let mut objects_1: Vec<ObjectId> = Vec::new(); while !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); objects_1.push(pool_1.remove(at)); } let moved_1 = g.exile_face_down(&objects_1, None);
g.end_sentence();
let controller_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let cards_2 = { let q: &dyn Query = g; moved_1.clone() }; let entered_2: Vec<ObjectId> = g.put_all_onto_battlefield_face_down(&cards_2, controller_2, FaceDown::Cloaked, false)?.into_iter().flatten().collect();
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Turn target face-down creature face up.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "Exile any number of face-up creatures you control with disguise in a face-down pile, shuffle that pile, then cloak them.").targets(&[])]).choose(1, 2);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Expose the Culprit", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

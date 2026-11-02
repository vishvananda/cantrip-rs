// Kindle the Inner Flame
// status: ready
// Create a token that's a copy of target creature you control, except it has haste and "At the beginning of the end step, sacrifice this token."
// Flashback—{1}{R}, Behold three Elementals. (You may cast this card from your graveyard for its flashback cost. Then exile it. To behold an Elemental, choose an Elemental you control or reveal an Elemental card from your hand.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn granted_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_2 { let controller = g.controller(object); if players_2.contains(&controller) && g.can_sacrifice(controller, object) { picks_2.push((controller, object)); } }
let sacrificed_2 = g.sacrifice_each(&picks_2);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_0 = { let q: &dyn Query = g; original_0.map(|original: ObjectId| q.copiable_values(original).with_keyword(Keyword::Haste).with_ability(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match values_0 { Some(values) => g.create_token_copies_tapped(values, player_0, count_0, false)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_behold_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental)) }
#[allow(unused_variables)] fn ability_1_require(q: &dyn Query, asker: &Asker) -> bool { ((q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery))) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Kindle the Inner Flame", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Kindred,CardType::Sorcery]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_1_event), granted_1, "At the beginning of the end step, sacrifice this token.").targets(&[])], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(cantrip_mechanics::behold::cost_of(3, ability_1_behold_3, "Elemental")), ZoneKind::Graveyard, "Flashback—{1}{R}, Behold three Elementals.").only_from().exile_after().condition(ability_1_require)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

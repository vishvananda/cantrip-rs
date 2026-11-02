// Assemble from Parts
// status: planned
// requires: engine: no perpetual effects
// requires: engine: no record of what paid a cost
// Target creature card in your graveyard perpetually gains "{1}{B}{B}, Exile this card from your graveyard: Create a token that's a copy of it, except it's a 4/4 black Zombie in addition to its other types. Shuffle the exiled card into its owner's library. Activate only as a sorcery."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn perpetual_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let original_3 = { let q: &dyn Query = g; ((planned_value!("engine: no record of what paid a cost", Vec::<ObjectId>::new() => q.paid_objects(item, [1]).to_vec())).into_iter().filter_map(|o: ObjectId| q.state().obj(o).became).collect::<Vec<ObjectId>>()).get(0).copied() }; let values_3 = { let q: &dyn Query = g; original_3.map(|original: ObjectId| q.copiable_values(original).sized(4, 4).colored(ColorSet::NONE.with(q.words(asker.source).color(Color::Black))).with_subtype(q.words(asker.source).subtype(Subtype::Zombie))) }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_3 = match values_3 { Some(values) => g.create_token_copies_tapped(values, player_3, count_3, false)?, None => Vec::new() };
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (((planned_value!("engine: no record of what paid a cost", Vec::<ObjectId>::new() => q.paid_objects(item, [1]).to_vec())).into_iter().filter_map(|o: ObjectId| q.state().obj(o).became).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_4 = g.put_into_library(&objects_4, LibraryPlace::ShuffledIn);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
for &object in &objects_1 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Assemble from Parts", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(EXILE_THIS), perpetual_2, "{1}{B}{B}, Exile this card from your graveyard: Create a token that's a copy of it, except it's a 4/4 black Zombie in addition to its other types. Shuffle the exiled card into its owner's library. Activate only as a sorcery.").targets(&[]).sorcery_speed().from(ZoneKind::Graveyard)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

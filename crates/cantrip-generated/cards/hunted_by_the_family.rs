// Hunted by The Family
// status: planned
// requires: villainous choice
// Choose up to four target creatures you don't control. For each of them, that creature's controller faces a villainous choice — That creature becomes a 1/1 white Human creature and loses all abilities, or you create a token that's a copy of it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 4)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for bound_0 in { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() } {
planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }, asker.source, &["Change characteristics until end of turn.", "Create tokens."])?);
let decider_1 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") };
let offered_1: Vec<u8> = [(0u8, true), (1u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_1 = item.ability.card; let picked_1 = if offered_1.len() > 1 { offered_1[cantrip_mechanics::choose::one_of(g, decider_1, asker.source, offered_1.iter().map(|&option| Label::Card(card_1, option)).collect())? as usize] } else { offered_1.first().copied().unwrap_or(u8::MAX) }; match picked_1 {
0 => {
let objects_2 = { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((1i64) as i16, (1i64) as i16) };
let kind_2_1_0 = { let q: &dyn Query = g; EffectKind::SetColors(q.words(asker.source).colors(ColorSet(1))) };
let kind_2_2_0 = { let q: &dyn Query = g; EffectKind::SetCreatureType(q.words(asker.source).subtype(Subtype::Human)) };
let kind_2_3_0 = { let q: &dyn Query = g; EffectKind::SetTypes(TypeSet::of(&[CardType::Creature])) };
let kind_2_4_0 = { let q: &dyn Query = g; EffectKind::LoseAllAbilities };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2); g.add_effect(source, object, kind_2_1_0, duration_2); g.add_effect(source, object, kind_2_2_0, duration_2); g.add_effect(source, object, kind_2_3_0, duration_2); g.add_effect(source, object, kind_2_4_0, duration_2);  } }); }
g.end_sentence();
hoisted_3.extend(objects_2.clone());
}
1 => {
let original_4 = { let q: &dyn Query = g; Some(bound_0) }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_4 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let created_4 = match original_4 { Some(original) => g.create_token_copies(original, player_4, count_4)?, None => Vec::new() };
g.end_sentence();
hoisted_5.extend(created_4.clone());
}
_ => {}
}
g.end_sentence();
hoisted_6.extend(hoisted_3.clone());
hoisted_7.extend(hoisted_5.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Hunted by The Family", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["That creature becomes a 1/1 white Human creature and loses all abilities", "You create a token that's a copy of it"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

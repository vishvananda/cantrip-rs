// Arcane Heist
// status: ready
// You may cast target instant or sorcery card from an opponent's graveyard without paying its mana cost. If that spell would be put into their graveyard, exile it instead.
// Cipher (Then you may exile this spell card encoded on a creature you control. Whenever that creature deals combat damage to a player, its controller may cast a copy of the encoded card without paying its mana cost.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().obj(id).owner))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let caster_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = 1; let mut played_0: Vec<ObjectId> = Vec::new();
while left_0 > 0 {
let mut open_0: Vec<ObjectId> = Vec::new(); for &c in &pool_0 { let ways = g.cast_ways_during_resolution_by(c, caster_0); if ways.iter().any(|w| true) { open_0.push(c); } }
let picked_0 = if open_0.is_empty() { None } else if cantrip_mechanics::effects::ask_may(g, caster_0, open_0[0], Purpose::CastFree)? { Some(open_0[0]) } else { None };
let Some(card_0) = picked_0 else { break; };
let ways_0: Vec<CastWay> = g.cast_ways_during_resolution_by(card_0, caster_0).into_iter().filter(|w| true).collect();
let done_0 = { let way = cantrip_mechanics::choose::cast_way(g, caster_0, card_0, ways_0)?.map(|w| w.way); g.cast_during_resolution_as(card_0, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_0), exile_instead: ExileInstead::Spell, ..CastTerms::default() })? };
if done_0 { left_0 -= 1; played_0.push(g.state().obj(card_0).became.unwrap_or(card_0)); } else { break; } pool_0.retain(|&c| c != card_0);
}
g.end_sentence();
cantrip_mechanics::cipher::encode(g, item)?;
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Arcane Heist", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[cantrip_mechanics::cipher::STATIC], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::cipher::TRIGGER], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

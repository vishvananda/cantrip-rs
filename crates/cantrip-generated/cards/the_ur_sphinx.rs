// The Ur-Sphinx
// status: planned
// requires: source-zone scope on Static
// Eminence — As long as The Ur-Sphinx is in the command zone or on the battlefield, other Sphinx spells you cast cost {1} less to cast.
// Flying
// Whenever one or more Sphinxes you control attack, each player mills that many cards. For each player, you may cast a card that player milled this way without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_stated_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (!(Some(asker.source) == Some(spell)) && q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Sphinx))) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Sphinx)) && Some(asker.controller) == Some(q.controller(id))) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (q.event_objects(item).len() as i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
for owner_1 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (milled_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
pool_2.retain(|&c| g.state().obj(c).owner == owner_1);
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!false || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::CastFree, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::CastFree)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if true { break; } if done_2 || !(true && true) { pool_2.retain(|&c| c != card_2); }
}
}
g.end_sentence();
 Ok(()) }
planned_item!("source-zone-scope-on-static" => cost_changes: cantrip_mechanics::zones::scoped(&[EventZone::Command, EventZone::Battlefield], None, ability_0_stated_0_rule));

pub static CARD: CardDef = CardDef { name: "The Ur-Sphinx", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [1, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx,Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever one or more Sphinxes you control attack, each player mills that many cards. For each player, you may cast a card that player milled this way without paying its mana cost.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

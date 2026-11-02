// Grist, the Hunger Tide
// status: planned
// requires: source-zone scope on Static
// As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition to its other types.
// [+1]: Create a 1/1 black and green Insect creature token, then mill a card. If an Insect card was milled this way, put a loyalty counter on Grist and repeat this process.
// [−2]: You may sacrifice a creature. When you do, destroy target creature or planeswalker.
// [−5]: Each opponent loses life equal to the number of creature cards in your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_stated_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); source == affected }
#[allow(unused_variables)] fn ability_0_stated_apply_2(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = chars.types.with(CardType::Creature);
chars.subtypes = chars.subtypes.with(q.words(asker.source).subtype(Subtype::Insect)); }
#[allow(unused_variables)] fn ability_0_stated_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = Some(p); chars.toughness = Some(t); }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Insect).name, token_words: token!(Insect).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(20)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_0 = false;
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mills_3: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_3)).collect(); let milled_3 = g.mill_together(&mills_3);
g.end_sentence();
if { let q: &dyn Query = g; (((milled_3.clone()).into_iter().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Insect))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::Loyalty, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
again_0 = true;
g.end_sentence();
hoisted_6.extend(applied_4.clone());
}
g.end_sentence();
hoisted_7.extend(milled_3.clone());
hoisted_8.extend(hoisted_6.clone());
hoisted_9.extend(created_1.clone());
hoisted_10.extend({ let q: &dyn Query = g; milled_3.clone() });
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_13_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static REFLEXIVE_13_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_13_target_0)];
fn reflexive_13(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_14 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_14 = g.destroy_all(&objects_14);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let objects_11 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_11: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_11 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_11 { let eligible: Vec<ObjectId> = objects_11.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_11.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_11.clone(), &[]))? }; for object in taken { picks_11.push((player, object)); } }
let sacrificed_11 = g.sacrifice_each(&picks_11);
if !sacrificed_11.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_15.extend(sacrificed_11.clone());
hoisted_16.extend({ let q: &dyn Query = g; objects_11.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });

}
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_17 = { let q: &dyn Query = g; ((((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>()).len() as i64).min(i32::MAX.into()) as i32 }; let players_17 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_17 { g.lose_life(player, amount_17); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("source-zone-scope-on-static" => statics: cantrip_mechanics::zones::scoped(&[EventZone::Hand, EventZone::Library, EventZone::Graveyard, EventZone::Exile, EventZone::Stack, EventZone::Command], None, static_ability(Layer::Type, ability_0_stated_affects, ability_0_stated_apply_2, "As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition to its other types.").own().everywhere()));
planned_item!("source-zone-scope-on-static" => statics: cantrip_mechanics::zones::scoped(&[EventZone::Hand, EventZone::Library, EventZone::Graveyard, EventZone::Exile, EventZone::Stack, EventZone::Command], None, static_ability(Layer::PtSet, ability_0_stated_affects, ability_0_stated_apply_0, "As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition to its other types.").continues().own().everywhere()));

pub static CARD: CardDef = CardDef { name: "Grist, the Hunger Tide", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Grist]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Create a 1/1 black and green Insect creature token, then mill a card. If an Insect card was milled this way, put a loyalty counter on Grist and repeat this process.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_2, "−2: You may sacrifice a creature. When you do, destroy target creature or planeswalker.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-5)), ability_3, "−5: Each opponent loses life equal to the number of creature cards in your graveyard.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_13, "When you do, destroy target creature or planeswalker.").targets(REFLEXIVE_13_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];

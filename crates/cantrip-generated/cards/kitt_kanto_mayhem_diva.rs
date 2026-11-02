// Kitt Kanto, Mayhem Diva
// status: ready
// When Kitt Kanto enters, create a 1/1 green and white Citizen creature token.
// At the beginning of combat on each player's turn, you may tap two untapped creatures you control. When you do, target creature that player controls gets +2/+2 and gains trample until end of turn. Goad that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Citizen).name, token_words: token!(Citizen).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Citizen]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(17)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && true { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_6_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && q.event_subject().player() == Some(q.controller(id))))) }
static REFLEXIVE_6_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_6_target_0)];
fn reflexive_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_7 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::EndOfTurn;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::Pump((2i64) as i16, (2i64) as i16) };
let kind_7_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Trample])) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7); g.add_effect(source, object, kind_7_1_0, duration_7);  } }); }
g.end_sentence();
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::Tap, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; (selected_3.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_4: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_4);
if !turned_4.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; item.subject.player() }.map_or(Target::None, Target::Player)); }
g.end_sentence();
hoisted_9.extend(turned_4.clone());
hoisted_10.extend(selected_3.clone());
hoisted_11.extend(objects_4.clone());
hoisted_12.extend({ let q: &dyn Query = g; turned_4.clone() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Kitt Kanto, Mayhem Diva", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Bard,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Kitt Kanto enters, create a 1/1 green and white Citizen creature token.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of combat on each player's turn, you may tap two untapped creatures you control. When you do, target creature that player controls gets +2/+2 and gains trample until end of turn. Goad that creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_6, "When you do, target creature that player controls gets +2/+2 and gains trample until end of turn. Goad that creature.").targets(REFLEXIVE_6_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

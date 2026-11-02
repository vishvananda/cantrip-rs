// Harrowing Swarm
// status: planned
// requires: a keyword's cost changed
// requires: granted Static
// Manifest dread. Then each face-down creature you control gains "This permanent costs {2} less to turn face up" and "When this permanent is turned face up, if it's a creature, put a +1/+1 counter on it."
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn granted_2_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { true && Some(asker.source) == Some(card) });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn granted_2_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
fn granted_3_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).is(CardType::Creature)) }
#[allow(unused_variables)] fn granted_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TurnedFaceUp { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn granted_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let entered_0: Vec<ObjectId> = cantrip_mechanics::manifest_dread::manifest_dread_object(g, player_0, asker.source)?.into_iter().collect();
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).face_down && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddAbility(planned_value!("granted Static", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
let kind_1_1_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1); g.add_effect(source, object, kind_1_1_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("granted-static" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, keyword_costs: &[KeywordCostRule { keyword: CostedKeyword::TurnFaceUp, applies: granted_2_0_keyword_cards_0, change: KeywordCostChange::Reduce(Cost::generic(2)), per: granted_2_0_keyword_per_0, first_each_turn: false }], ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Harrowing Swarm", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::TURNED_FACE_UP.0), granted_3_event), granted_3, "When this permanent is turned face up, if it's a creature, put a +1/+1 counter on it.").targets(&[]).condition(granted_3_if)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

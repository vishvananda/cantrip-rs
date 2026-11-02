// Elrond of the White Council
// status: planned
// requires: condition VotedFor
// requires: granted Static
// requires: numeric expression Votes
// requires: operation RevealChoices
// requires: operation Vote
// Secret council — When Elrond enters, each player secretly votes for fellowship or aid, then those votes are revealed. For each fellowship vote, the voter chooses a creature they control. You gain control of each creature chosen this way, and they gain "This creature can't attack its owner." Then for each aid vote, put a +1/+1 counter on each creature you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_8_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { !(attacker == asker.source && (match defender { Target::Player(p) => (Some(asker.source).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => false && q.chars(id).is(CardType::Planeswalker) && (Some(asker.source).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let voters_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_1, cantrip_mechanics::vote::Ballot::Words(&["Fellowship", "Aid"]), true)?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Word));
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let selected_4_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::ChooseAsEnters, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
 each_5[index_3] = selected_4.clone(); all_5.extend(each_5[index_3].iter().copied());
   } 
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; (all_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::Indefinite;
let kind_6_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6);  } }); }
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; (objects_6.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::Indefinite;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::AddAbility(planned_value!("granted Static", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7);  } }); }
g.end_sentence();
let mut counters_9: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (planned_value!("numeric expression Votes", 0i64 => g.votes().count(Word))).min(i16::MAX.into()) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_9.push((object, CounterKind::PlusOne, amount)); } }
let had_9: Vec<_> = counters_9.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_9); let applied_9: Vec<ObjectId> = counters_9.iter().zip(had_9).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
planned_item!("granted-static" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[granted_8_0_rule], untap_bans: &[], enters: None, ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Elrond of the White Council", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Secret council — When Elrond enters, each player secretly votes for fellowship or aid, then those votes are revealed. For each fellowship vote, the voter chooses a creature they control. You gain control of each creature chosen this way, and they gain \"This creature can't attack its owner.\" Then for each aid vote, put a +1/+1 counter on each creature you control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

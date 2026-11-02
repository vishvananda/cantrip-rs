// Damocles Base, Sword of Kang
// status: planned
// requires: villainous choice
// Flying, deathtouch
// Whenever Damocles Base deals combat damage to a player, that player faces a villainous choice — They sacrifice a nontoken creature of their choice, or they lose 2 life and you draw two cards.
// Crew 3
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }, asker.source, &["", "Lose 2 life. Draw 2 cards."])?);
let decider_1 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") };
let offered_1: Vec<u8> = [(0u8, { let q: &dyn Query = g; { let players: Vec<PlayerId> = Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>(); (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| players.contains(&q.obj(o).controller)) } }), (1u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_1 = item.ability.card; let picked_1 = if offered_1.len() > 1 { offered_1[cantrip_mechanics::choose::one_of(g, decider_1, asker.source, offered_1.iter().map(|&option| Label::Card(card_1, option)).collect())? as usize] } else { offered_1.first().copied().unwrap_or(u8::MAX) }; match picked_1 {
0 => {
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_2 { let eligible: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_2.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_2.clone(), &[]))? }; for object in taken { picks_2.push((player, object)); } }
let sacrificed_2 = g.sacrifice_each(&picks_2);
g.end_sentence();
hoisted_3.extend(sacrificed_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
1 => {
let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() };
let mut done_5: i64 = 0; for player in players_5 { let before = i64::from(g.state().player(player).life); g.lose_life(player, amount_5); done_5 += (i64::from(g.state().player(player).life) - before).abs(); }
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Damocles Base, Sword of Kang", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Damocles Base deals combat damage to a player, that player faces a villainous choice — They sacrifice a nontoken creature of their choice, or they lose 2 life and you draw two cards.").targets(&[]),cantrip_mechanics::crew::crew(3, "Crew 3")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["They sacrifice a nontoken creature of their choice", "They lose 2 life and you draw two cards"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

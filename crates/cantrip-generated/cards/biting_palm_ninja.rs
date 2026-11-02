// Biting-Palm Ninja
// status: ready
// Ninjutsu {2}{B}
// This creature enters with a menace counter on it.
// Whenever this creature deals combat damage to a player, you may remove a menace counter from it. When you do, that player reveals their hand and you choose a nonland card from it. Exile that card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Menace as usize] = entering.counters[CounterKind::Menace as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; ((item.subject.player().into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_5: Vec<PlayerId> = objects_5.iter().map(|o| g.state().obj(*o).owner).collect(); owners_5.dedup(); for owner in owners_5 { let cards: Vec<ObjectId> = objects_5.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; ((item.subject.player().into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::Exile, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
let objects_7 = { let q: &dyn Query = g; selected_6.clone() }; let moved_7 = g.exile_all(&objects_7);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if ({ let q: &dyn Query = g; item.subject.object() }).is_some_and(|o| cantrip_mechanics::targets::permanent(g, o) && g.obj(o).counters(CounterKind::Menace) > 0) && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::Menace, amount_2_0)); } });
if removed_2 > 0 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)) }.map_or(Target::None, Target::Player)); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Biting-Palm Ninja", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Ninja]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::ninjutsu::ninjutsu(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Ninjutsu {2}{B}"),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever this creature deals combat damage to a player, you may remove a menace counter from it. When you do, that player reveals their hand and you choose a nonland card from it. Exile that card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "When you do, that player reveals their hand and you choose a nonland card from it. Exile that card.").targets(&[])], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

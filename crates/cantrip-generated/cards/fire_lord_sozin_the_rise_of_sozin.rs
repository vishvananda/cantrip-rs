// Fire Lord Sozin // The Rise of Sozin
// status: ready
// Menace, firebending 3 (Whenever this creature attacks, add {R}{R}{R}. This mana lasts until end of combat.)
// Whenever Fire Lord Sozin deals combat damage to a player, you may pay {X}. When you do, put any number of target creature cards with total mana value X or less from that player's graveyard onto the battlefield under your control.
// (As this Saga enters and after your draw step, add a lore counter.)
// I — Destroy all creatures.
// II — Choose a card name. Search target opponent's graveyard, hand, and library for up to four cards with that name and exile them. Then that player shuffles.
// III — Exile this Saga, then return it to the battlefield transformed under your control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_1_target_0)];
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_1)) = g.ask(player_1, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
g.end_sentence();
let searcher_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") };
let mut left_2: u32 = { let q: &dyn Query = g; (4i64).max(0) as u32 };
let mut found_2: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_2)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_1).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_2, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_2), &shown, can))?; left_2 -= taken.len() as u32; found_2.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_2)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_1).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_2, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_2), &shown, can))?; left_2 -= taken.len() as u32; found_2.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_2); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| Some(chosen_1).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, id, chosen))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::UpTo(left_2), &shown, can))?; found_2.extend(taken); }
let objects_3 = { let q: &dyn Query = g; found_2.clone() }; let moved_3 = g.exile_all(&objects_3);
for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_6 = ({ let q: &dyn Query = g; ((moved_5.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_6 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_6 { let controller = player_6; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Rise of Sozin", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Destroy all creatures.").targets(&[]),chapter(2, ability_0_1, "II — Choose a card name. Search target opponent's graveyard, hand, and library for up to four cards with that name and exile them. Then that player shuffles.").targets(ABILITY_0_1_TARGETS),chapter(3, ability_0_2, "III — Exile this Saga, then return it to the battlefield transformed under your control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && q.event_subject().player() == Some(q.state().obj(id).owner))))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_mechanics::select::total_mana_value_at_most(TargetSpec::COUNTED))];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_5 = ({ let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_5 = { let rows_5: Vec<(ObjectId, PlayerId, bool)> = objects_5.iter().map(|&o| (o, player_5, false)).collect(); let came_5 = g.put_all_onto_battlefield_as(&rows_5)?; came_5.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
'optional_0: {
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let x_2 = g.may_pay_x(payer_2, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?; let paid_2 = x_2.is_some(); let bound_2: i64 = i64::from(x_2.unwrap_or(0));
if !paid_2 { break 'optional_0; }
if paid_2 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)) }.map_or(Target::None, Target::Player), (bound_2).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Fire Lord Sozin", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(4)), power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::firebending::trigger::<3>("Firebending"),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Fire Lord Sozin deals combat damage to a player, you may pay {X}. When you do, put any number of target creature cards with total mana value X or less from that player's graveyard onto the battlefield under your control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "Whenever Fire Lord Sozin deals combat damage to a player, you may pay {X}. When you do, put any number of target creature cards with total mana value X or less from that player's graveyard onto the battlefield under your control.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}

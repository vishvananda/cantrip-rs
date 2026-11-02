// Runed Crown
// status: ready
// When this Equipment enters, you may search your library, hand, and/or graveyard for a Rune card and put it onto the battlefield attached to this Equipment. If you search your library this way, shuffle.
// Equipped creature gets +1/+1.
// Equip {2}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static ASK_1: Question = Question::new("SearchLibrary", "search your library as well?", "Then shuffle. If you don't, you search only the other zones", "Search it too", "Not the library");
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = searcher_1; let may_1 = g.may_search(searcher_1, owner_1); let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1: Vec<ObjectId> = (if may_1 { cantrip_mechanics::library::searched_cards(g, owner_1) } else { Vec::new() }).into_iter().chain(g.state().zone(Zone::graveyard(owner_1)).iter().copied()).chain(g.state().zone(Zone::hand(owner_1)).iter().copied()).collect::<Vec<ObjectId>>(); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Rune))).collect() };
let zones_1: Vec<ObjectId> = g.state().zone(Zone::graveyard(owner_1)).iter().chain(g.state().zone(Zone::hand(owner_1)).iter()).copied().collect(); let searched_1 = cantrip_mechanics::library::ask_may_search(g, searcher_1, asker.source, &ASK_1)?; let found_1: Vec<ObjectId> = if searched_1 { { let rule = cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1.clone()); let found = if shown_1.is_empty() { Vec::new() } else { match g.ask(searcher_1, RequestKind::SearchLibrary { source: asker.source, library: shown_1.clone(), rule: Box::new(rule), destination: SearchDestination::Battlefield })? { Answer::Cards(cards) => cards, _ => Vec::new() } }; if may_1 { g.searched(searcher_1, owner_1); } found } } else { let rule = { let in_zones: Vec<ObjectId> = can_1.iter().copied().filter(|o| zones_1.contains(o)).collect(); cantrip_mechanics::select::Selection::listed(rule_count_1, &zones_1, in_zones) }; let any = cantrip_api::select::first_answer(&*g, &rule, searcher_1, asker.source, &zones_1, true).is_some_and(|taken| !taken.is_empty()); if any { cantrip_mechanics::choose::select(g, searcher_1, asker.source, Purpose::PutOntoBattlefield, zones_1.clone(), rule)? } else { Vec::new() } };
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; found_1.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_2 { let controller = player_2; if let Some(host) = { let q: &dyn Query = g; Some(asker.source) } { moved.extend(g.put_onto_battlefield_attached(o, controller, host)?); } } moved };
if { let q: &dyn Query = g; searched_1 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(found_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_7 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_7 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_7 { cantrip_api::instruction(g, |g| { for attachment in attachments_7 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Runed Crown", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this Equipment enters, you may search your library, hand, and/or graveyard for a Rune card and put it onto the battlefield attached to this Equipment. If you search your library this way, shuffle.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "Equip {2}").targets(ABILITY_2_TARGETS).equip().sorcery_speed()], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Equipped creature gets +1/+1.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

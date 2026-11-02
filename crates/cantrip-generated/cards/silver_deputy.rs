// Silver Deputy
// status: ready
// When this creature enters, you may search your library for a basic land card or a Desert card, reveal it, then shuffle and put it on top.
// {T}: Target creature you control gets +1/+0 until end of turn. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.may_search(decider_0, decider_0) && cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1 = cantrip_mechanics::library::searched_cards(g, owner_1); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| ((q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Desert)))).collect() }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
let objects_2 = { let q: &dyn Query = g; found_1.clone() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
let library_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_3: Vec<ObjectId> = { let q: &dyn Query = g; found_1.clone() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_3)).collect(); let placed_3 = cards_3.clone();
let chooser_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_3 = cantrip_mechanics::library::ask_library_order(g, chooser_3, asker.source, &cards_3, DestinationKind::LibraryTop)?; g.arrange_library(library_3, &order_3, &[]);
g.end_sentence();
hoisted_4.extend(placed_3.clone());
hoisted_5.extend(found_1.clone());
hoisted_6.extend(objects_2.clone());
hoisted_7.extend({ let q: &dyn Query = g; placed_3.clone() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_8 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::EndOfTurn;
let kind_8_0_0 = { let q: &dyn Query = g; EffectKind::Pump((1i64) as i16, (0i64) as i16) };
if let Some(source) = source_8 { cantrip_api::instruction(g, |g| { for &object in &objects_8 { g.add_effect(source, object, kind_8_0_0, duration_8);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Silver Deputy", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Mercenary]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may search your library for a basic land card or a Desert card, reveal it, then shuffle and put it on top.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Target creature you control gets +1/+0 until end of turn. Activate only as a sorcery.").targets(ABILITY_1_TARGETS).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

// Karoo
// status: ready
// This land enters tapped.
// When this land enters, sacrifice it unless you return an untapped Plains you control to its owner's hand.
// {T}: Add {C}{W}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)? {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let selected_3 = if selected_3_candidates.len() as u64 >= u64::from(selected_3_count) { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutIntoHand, selected_3_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_3_count), &[]))? } else { Vec::new() };
let objects_2 = { let q: &dyn Query = g; selected_3.clone() }; let moved_2 = g.return_to_hand(&objects_2);
let paid_2 = !objects_2.is_empty();
(paid_2, { let q: &dyn Query = g; (moved_2).clone() }, { let q: &dyn Query = g; (objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_3).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let objects_4 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_4 { let controller = g.controller(object); if players_4.contains(&controller) && g.can_sacrifice(controller, object) { picks_4.push((controller, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
g.end_sentence();
hoisted_5.extend(sacrificed_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
let mana_7_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_7 {
g.add_mana(player, Mana::W, mana_7_0);g.add_mana(player, Mana::C, mana_7_1);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Karoo", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this land enters, sacrifice it unless you return an untapped Plains you control to its owner's hand.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([1, 0, 0, 0, 0, 1]), ..add_by(ability_2_mana) }, "{T}: Add {C}{W}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

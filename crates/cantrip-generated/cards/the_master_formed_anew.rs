// The Master, Formed Anew
// status: planned
// requires: engine: a two-word subtype in a card's type line
// Body Thief — When you cast this spell, you may exile a creature you control and put a takeover counter on it.
// You may have The Master enter as a copy of a creature card in exile with a takeover counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
if !selected_0.is_empty() { 
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (moved_1.clone()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::Takeover, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_3.extend(moved_1.clone());
hoisted_4.extend(selected_0.clone());
hoisted_5.extend(applied_2.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); const ASK_7: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_7: Vec<ObjectId> = ({ let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (i64::from(q.state().obj(id).counters(CounterKind::Takeover))) > (0i64))).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_7) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_7, &ASK_7)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_7).map(|original: ObjectId| q.copiable_values(original)) }; let picked_7 = vec![chosen_7];
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Master, Formed Anew", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[planned_value!("engine: a two-word subtype in a card's type line", Subtype::TimeLord => Subtype::TimeLord),Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Body Thief — When you cast this spell, you may exile a creature you control and put a takeover counter on it.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_1_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

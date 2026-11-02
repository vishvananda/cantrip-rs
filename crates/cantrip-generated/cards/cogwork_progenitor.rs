// Cogwork Progenitor
// status: planned
// requires: engine: no perpetual effects
// At the beginning of your end step, you may exile another artifact you control or an artifact card in your graveyard. If you do, seek an artifact card. That card perpetually has base power and toughness 1/1 and becomes a Gnome creature in addition to its other types.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn paid_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))) }
#[allow(unused_variables)] fn paid_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_permanents(1, paid_1)), &PAY_QUESTION)? || g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_from_graveyard_of(1, paid_2)), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let mut pool_3 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_3: Vec<ObjectId> = Vec::new(); while picked_3.len() < count_3 && !pool_3.is_empty() { let at = g.choose_at_random(pool_3.len()); picked_3.push(pool_3.remove(at)); }
let objects_4 = { let q: &dyn Query = g; picked_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_6: Vec<ObjectId> = { let q: &dyn Query = g; (moved_4.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_6 { let kind_6_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::SetPt((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_6_0); let kind_6_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Gnome))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_6_1); let kind_6_2 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddTypes(TypeSet::of(&[CardType::Creature]))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_6_2); });
g.end_sentence();
hoisted_7.extend(moved_4.clone());
hoisted_8.extend(picked_3.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Cogwork Progenitor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Gnome]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, you may exile another artifact you control or an artifact card in your graveyard. If you do, seek an artifact card. That card perpetually has base power and toughness 1/1 and becomes a Gnome creature in addition to its other types.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

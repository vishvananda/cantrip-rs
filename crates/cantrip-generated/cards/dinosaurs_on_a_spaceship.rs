// Dinosaurs on a Spaceship
// status: planned
// requires: engine: no event for a counter being removed
// Vigilance, trample
// Other Dinosaurs you control get +1/+1 and have vigilance and trample.
// Suspend 4—{3}{R}{W}
// Whenever a time counter is removed from this card while it's exiled, create a 2/2 red and white Dinosaur creature token with flying and haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Dinosaur)) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Vigilance); }
#[allow(unused_variables)] fn ability_2_apply_2(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Trample); }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dinosaur).name, token_words: token!(Dinosaur).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(9)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-event-for-a-counter-being-removed" => matcher: #[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersRemoved { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if Some(asker.source) == Some(id) && *kind == CounterKind::Time { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Dinosaurs on a Spaceship", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::suspend::ability(ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Suspend"),triggered(when_asked(planned_value!("engine: no event for a counter being removed", EventMask(0) => EventMask(EventMask::COUNTERS_REMOVED.0)), ability_4_event), ability_4, "Whenever a time counter is removed from this card while it's exiled, create a 2/2 red and white Dinosaur creature token with flying and haste.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Exile]))], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_1, "Other Dinosaurs you control get +1/+1 and have vigilance and trample."),static_ability(Layer::Ability, ability_2_affects, ability_2_apply_2, "Other Dinosaurs you control get +1/+1 and have vigilance and trample."),static_ability(Layer::PtModify, ability_2_affects, ability_2_apply_0, "Other Dinosaurs you control get +1/+1 and have vigilance and trample.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, suspend: Some(SuspendDef { count: 4, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] } }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

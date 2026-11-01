// The Book of Vile Darkness
// status: planned
// requires: engine: no cost moves these objects
// requires: engine: no record of what paid a cost
// requires: token with the abilities of other objects
// At the beginning of your end step, if you lost 2 or more life this turn, create a 2/2 black Zombie creature token.
// {T}, Exile The Book of Vile Darkness and artifacts you control named Eye of Vecna and Hand of Vecna: Create Vecna, a legendary 8/8 black Zombie God creature token with indestructible and all triggered abilities of the exiled cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta < 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()) >= (2i64) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Zombie).name, token_words: token!(Zombie).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Vecna", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(4)), power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let created_2: Vec<ObjectId> = planned_value!("token with the abilities of other objects", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::Filled(g.card_id(&token_3::CARD), TokenFill { color: None, creature_type: None, power_toughness: None, abilities_of: Some(({ let q: &dyn Query = g; (planned_value!("engine: no record of what paid a cost", Vec::<ObjectId>::new() => q.paid_objects(item, [1]).to_vec())).into_iter().filter_map(|o: ObjectId| q.state().obj(o).became).collect::<Vec<ObjectId>>() }, &[CheckHasable::TriggeredAbility])) }))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_4(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)) && q.chars(id).name == "Eye of Vecna") }
#[allow(unused_variables)] fn ability_1_5(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)) && q.chars(id).name == "Hand of Vecna") }

pub static CARD: CardDef = CardDef { name: "The Book of Vile Darkness", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Book]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, if you lost 2 or more life this turn, create a 2/2 black Zombie creature token.").targets(&[]).condition(ability_0_if),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(EXILE_THIS.and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Battlefield, to: CostDestination::Exile, count: CostCount::Exactly(1), filter: ability_1_4 }), ..Cost::NONE })).and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Battlefield, to: CostDestination::Exile, count: CostCount::Exactly(1), filter: ability_1_5 }), ..Cost::NONE }))), ability_1, "{T}, Exile The Book of Vile Darkness and artifacts you control named Eye of Vecna and Hand of Vecna: Create Vecna, a legendary 8/8 black Zombie God creature token with indestructible and all triggered abilities of the exiled cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_3::CARD];

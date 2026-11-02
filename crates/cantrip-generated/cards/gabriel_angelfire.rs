// Gabriel Angelfire
// status: planned
// requires: a choice of Ability
// requires: a choice of Ability outside its body
// requires: duration UntilNext upkeep
// At the beginning of your upkeep, choose flying, first strike, trample, or rampage 3. Gabriel Angelfire gains that ability until your next upkeep. (Whenever a creature with rampage 3 becomes blocked, it gets +3/+3 until end of turn for each creature blocking it beyond the first.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("a-choice-of-ability" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Ability, ChoiceTerms { options: &["Ability(Keyword(Flying))", "Ability(Keyword(FirstStrike))", "Ability(Keyword(Trample))", "Ability(Mechanic { mechanic: Rampage, cost: None, amount: Some(Constant(3)), filter: None })"], except: &[], count: Some(1), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
planned!("a-choice-of-ability-outside-its-body" => { let objects: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; cantrip_mechanics::effects::grant(g, item, &objects, &[Grant::ChosenAbility], planned_value!("duration UntilNext upkeep", Duration::Indefinite => Duration::UntilNextUpkeepOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })))? });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gabriel Angelfire", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, choose flying, first strike, trample, or rampage 3. Gabriel Angelfire gains that ability until your next upkeep.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

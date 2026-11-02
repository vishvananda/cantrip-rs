// Shapeshifter
// status: planned
// requires: a choice of Number made again
// As this creature enters, choose a number between 0 and 7.
// At the beginning of your upkeep, you may choose a number between 0 and 7.
// Shapeshifter's power is equal to the last chosen number and its toughness is equal to 7 minus that number.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let least_0 = 0u32; let most_0 = 7u32; cantrip_mechanics::choose::number_as_enters(g, entering, least_0, most_0, "Choose a number")?; let chosen_0: u32 = entering.chosen_number.unwrap_or(0);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let least_2 = 0u32; let most_2 = 7u32; let chosen_2: u32 = cantrip_mechanics::choose::number(g, player_2, asker.source, least_2, most_2, "Choose a number")?;
planned!("a-choice-of-number-made-again" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Number(chosen_2)););
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.power = Some((q.state().chosen_number(q.linked_to(asker.source)).map_or(0i64, i64::from)).clamp(-32768, 32767) as i16);
chars.toughness = Some(((7i64 - q.state().chosen_number(q.linked_to(asker.source)).map_or(0i64, i64::from))).clamp(-32768, 32767) as i16); }

pub static CARD: CardDef = CardDef { name: "Shapeshifter", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, you may choose a number between 0 and 7.").targets(&[])], statics: &[static_ability(Layer::PtSet, cantrip_mechanics::statics::itself, ability_2_apply_0, "Shapeshifter's power is equal to the last chosen number and its toughness is equal to 7 minus that number.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

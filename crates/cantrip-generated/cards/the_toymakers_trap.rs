// The Toymaker's Trap
// status: planned
// requires: condition Outcome
// requires: operation ChooseValueWith
// requires: operation Guess
// requires: operation RevealChoices
// requires: player reference Named
// At the beginning of your upkeep, secretly choose a number between 1 and 5 that hasn't been chosen. If you do, an opponent guesses which number you chose, then you reveal the number you chose. If they guessed wrong, they lose life equal to the number they guessed and you draw a card. If they guessed right, sacrifice this enchantment.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let chosen_0 = planned_value!("operation ChooseValueWith", 0u32 => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Number, ChoiceTerms { options: &["Comparison(GreaterOrEqual, Constant(1))", "Comparison(LessOrEqual, Constant(5))"], except: &[], count: None, secret: true, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if { let q: &dyn Query = g; ((i64::from(chosen_0) as i64)) >= (1i64) } {
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
planned!("operation-guess" => cantrip_mechanics::guess::guess(g, { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }, asker.source, None, None, None)?);

g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Number));
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition Outcome", false => q.outcome(asker, Outcome::GuessedWrong)) } {
let amount_4 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", 0i64 => result_1)).clamp(0, 2147483647) as i32 }; let players_4 = { let q: &dyn Query = g; planned_value!("player reference Named", None::<PlayerId> => q.named_player(asker, 1, 0)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.lose_life(player, amount_4); }
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
} else {
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_6: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_6 { let controller = g.controller(object); if players_6.contains(&controller) && g.can_sacrifice(controller, object) { picks_6.push((controller, object)); } }
let sacrificed_6 = g.sacrifice_each(&picks_6);
g.end_sentence();
hoisted_7.extend(sacrificed_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_9.extend(hoisted_7.clone());
hoisted_10.extend({ let q: &dyn Query = g; hoisted_8.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Toymaker's Trap", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, secretly choose a number between 1 and 5 that hasn't been chosen. If you do, an opponent guesses which number you chose, then you reveal the number you chose. If they guessed wrong, they lose life equal to the number they guessed and you draw a card. If they guessed right, sacrifice this enchantment.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

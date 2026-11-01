// Nature Shields Its Own
// status: planned
// requires: a token entering blocking
// requires: card type Scheme
// requires: layout Scheme
// requires: operation AbandonScheme
// requires: supertype Ongoing
// (An ongoing scheme remains face up until it's abandoned.)
// Whenever a creature attacks and isn't blocked, if you're the defending player, create a 0/1 green Plant creature token that's blocking that creature.
// When four or more creatures attack you, abandon this scheme at end of combat.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Unblocked { attacker, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if q.chars(id).is(CardType::Creature) && (({ let mut all = Some(asker.controller).into_iter().collect::<Vec<PlayerId>>(); let other = subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) > (0i64) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Plant).name, token_words: token!(Plant).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Plant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; (({ let mut all = Some(asker.controller).into_iter().collect::<Vec<PlayerId>>(); let other = item.subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) > (0i64) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
planned!("a-token-entering-blocking" => if let Some(attacker) = { let q: &dyn Query = g; item.subject.object() } { g.enter_blocking(&created_0, attacker); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && match *defender { Target::Player(p) => Some(asker.controller) == Some(p), _ => false } && (q.state().attackers.iter().filter(|a| { let (id, defender) = (a.id, &a.target); q.chars(id).is(CardType::Creature) && match *defender { Target::Player(p) => Some(asker.controller) == Some(p), _ => false } && a.defending == *defending }).count() as i64) >= (4i64) { return Some(Fired { subject, data }); } }
 None }
fn delayed_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
g.delayed_trigger(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
 Ok(()) }
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Nature Shields Its Own", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype Ongoing", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, ongoing: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::UNBLOCKED.0), ability_0_event), ability_0, "Whenever a creature attacks and isn't blocked, if you're the defending player, create a 0/1 green Plant creature token that's blocking that creature.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "When four or more creatures attack you, abandon this scheme at end of combat.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_3, "When four or more creatures attack you, abandon this scheme at end of combat.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

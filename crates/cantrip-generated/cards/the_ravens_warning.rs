// The Raven's Warning
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Create a 1/1 blue Bird creature token with flying. You gain 2 life.
// II — Whenever one or more creatures you control with flying deal combat damage to a player this turn, look at that player's hand and draw a card.
// III — You may put a card you own from outside the game on top of your library.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bird).name, token_words: token!(Bird).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source)) && q.chars(source).keywords.has(Keyword::Flying)) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn watch_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_5 = { let q: &dyn Query = g; ((Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; g.look_at(player_5, &objects_5);
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_8_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::PutOnTop, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
if !selected_8.is_empty() { 
let objects_9 = ({ let q: &dyn Query = g; selected_8.clone() }).into_iter().rev().collect::<Vec<ObjectId>>(); let moved_9 = g.put_into_library(&objects_9, LibraryPlace::Top);
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Raven's Warning", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Create a 1/1 blue Bird creature token with flying. You gain 2 life.").targets(&[]),chapter(2, ability_0_1, "II — Whenever one or more creatures you control with flying deal combat damage to a player this turn, look at that player's hand and draw a card.").targets(&[]),chapter(3, ability_0_2, "III — You may put a card you own from outside the game on top of your library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), watch_4_event), watch_4, "Whenever one or more creatures you control with flying deal combat damage to a player this turn, look at that player's hand and draw a card.").targets(&[]).once_per_batch().batch_by_data()], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

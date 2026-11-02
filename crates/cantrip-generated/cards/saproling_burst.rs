// Saproling Burst
// status: ready
// Fading 7 (This enchantment enters with seven fade counters on it. At the beginning of your upkeep, remove a fade counter from it. If you can't, sacrifice it.)
// Remove a fade counter from this enchantment: Create a green Saproling creature token. It has "This token's power and toughness are each equal to the number of fade counters on Saproling Burst."
// When this enchantment leaves the battlefield, destroy all tokens created with this enchantment. They can't be regenerated.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((q.history().all().iter().find_map(|r| match (r.event, r.cause) { (Event::Entered { permanent, .. }, Cause::Ability { source, .. }) if permanent == asker.source => Some(source), _ => None }).filter(|&o| q.state().obj(o).became.is_none()).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Fade))).sum::<i64>()).clamp(-32768, 32767) as i16, ((q.history().all().iter().find_map(|r| match (r.event, r.cause) { (Event::Entered { permanent, .. }, Cause::Ability { source, .. }) if permanent == asker.source => Some(source), _ => None }).filter(|&o| q.state().obj(o).became.is_none()).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Fade))).sum::<i64>()).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }

pub static CARD: CardDef = CardDef { name: token!(Saproling).name, token_words: token!(Saproling).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Saproling]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[characteristic_defining(Layer::PtDefine, ability_0_apply_0, "This token's power and toughness are each equal to the number of fade counters on Saproling Burst.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| Some(asker.source).is_some_and(|m: ObjectId| q.state().obj(id).entered_with == Some(m) || q.history().all().iter().any(|r| matches!(r.event, Event::Entered { permanent, .. } if permanent == id) && (r.cause == Cause::Spell(m) || matches!(r.cause, Cause::Ability { source, .. } if source == m))))).collect::<Vec<ObjectId>>() }; let destroyed_2 = g.destroy_all_no_regen(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Saproling Burst", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::fading::upkeep("Fading"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(remove_counters(CounterKind::Fade, 1)), ability_1, "Remove a fade counter from this enchantment: Create a green Saproling creature token. It has \"This token's power and toughness are each equal to the number of fade counters on Saproling Burst.\"").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this enchantment leaves the battlefield, destroy all tokens created with this enchantment. They can't be regenerated.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(cantrip_mechanics::fading::enters::<7>), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

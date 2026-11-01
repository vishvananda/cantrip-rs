// Maular, the Next Evolution
// status: planned
// requires: a deck rule a commander sets
// Rulebreaker — A deck with this commander can have creature cards with mana value 7 or greater of any color identity and any basic land cards.
// Whenever a creature you control with mana value 7 or greater attacks, double its power and toughness until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && (i64::from(q.mana_value(id))) >= (7i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kinds_0 = { let mut all = Vec::new(); for &object in &objects_0 { let kind = { let q: &dyn Query = g; EffectKind::Pump((Some(object).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))) as i16, (Some(object).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).toughness.unwrap_or(0)))) as i16) }; all.push((object, kind));  } all }; if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_0 { g.add_effect(source, object, kind, duration_0); } }); }
g.end_sentence();
 Ok(()) }
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyCards);
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyBasicLandCards);

pub static CARD: CardDef = CardDef { name: "Maular, the Next Evolution", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur,Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever a creature you control with mana value 7 or greater attacks, double its power and toughness until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

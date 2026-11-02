// Arboreal Alliance
// status: ready
// When this enchantment enters, create an X/X green Treefolk creature token.
// Whenever you attack with one or more Elves, populate. (Create a token that's a copy of a creature token you control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Treefolk).name, token_words: token!(Treefolk).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Treefolk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_0 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))).clamp(0, 32767) as i16 }; let toughness_0 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))).clamp(0, 32767) as i16 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_0 = Copiable::of(g.card_id(&token_1::CARD)).sized(power_0, toughness_0); values_0.words = g.words(asker.source); let mut created_0: Vec<ObjectId> = Vec::new(); for player in players_0 { created_0.extend(g.create_token_copies_tapped(values_0, player, count_0, false)?); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elf)) && Some(asker.controller) == Some(q.controller(id)) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut populated_2: Vec<ObjectId> = Vec::new();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let tokens: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&o| g.obj(o).token && g.chars(o).is(CardType::Creature)).collect(); if let Some(token) = cantrip_mechanics::choose::card(g, player, asker.source, Purpose::CopyWhich, tokens)? { let values = g.copiable_values(token); populated_2.extend(g.create_token_copies_tapped(values, player, 1, false)?); }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arboreal Alliance", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, create an X/X green Treefolk creature token.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever you attack with one or more Elves, populate.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];

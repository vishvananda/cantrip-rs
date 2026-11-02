// Imperial Mask
// status: ready
// When this enchantment enters, if it's not a token, each of your teammates creates a token that's a copy of this enchantment.
// You have hexproof. (You can't be the target of spells or abilities your opponents control.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| !(q.state().obj(candidate).token)) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let original_0 = { let q: &dyn Query = g; Some(asker.source) }; let values_0 = { let q: &dyn Query = g; original_0.map(|original: ObjectId| q.copiable_values(original)) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; { let mut all = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0) == Some(asker.controller)).collect::<Vec<PlayerId>>(); let other = { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }; all.retain(|p: &PlayerId| other.contains(p)); all } }; let mut copies_0: Vec<(Copiable, PlayerId)> = Vec::new(); if let Some(values) = values_0 { for &player in &players_0 { for _ in 0..count_0 { copies_0.push((values.clone(), player)); } } } let created_0 = g.create_token_copies_together(&copies_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }

pub static CARD: CardDef = CardDef { name: "Imperial Mask", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, if it's not a token, each of your teammates creates a token that's a copy of this enchantment.").targets(&[]).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_1_0_rule }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

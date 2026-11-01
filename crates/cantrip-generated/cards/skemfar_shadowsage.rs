// Skemfar Shadowsage
// status: ready
// When this creature enters, choose one —
// • Each opponent loses X life, where X is the greatest number of creatures you control that have a creature type in common.
// • You gain X life, where X is the greatest number of creatures you control that have a creature type in common.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ({ let those: Vec<ObjectId> = q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>(); those.iter().flat_map(|&a| q.chars(a).subtypes.iter().filter(|s| s.card_types().has(CardType::Creature)).collect::<Vec<Subtype>>()).map(|s| those.iter().filter(|&&b| q.chars(b).has_subtype(s)).count()).max().unwrap_or(0) as i64 }).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; ({ let those: Vec<ObjectId> = q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>(); those.iter().flat_map(|&a| q.chars(a).subtypes.iter().filter(|s| s.card_types().has(CardType::Creature)).collect::<Vec<Subtype>>()).map(|s| those.iter().filter(|&&b| q.chars(b).has_subtype(s)).count()).max().unwrap_or(0) as i64 }).clamp(0, 2147483647) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Each opponent loses X life, where X is the greatest number of creatures you control that have a creature type in common.").targets(&[]),mode(ability_0_mode_1, "You gain X life, where X is the greatest number of creatures you control that have a creature type in common.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Skemfar Shadowsage", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), &ABILITY_0_MODES, "When this creature enters, choose one — Each opponent loses X life, where X is the greatest number of creatures you control that have a creature type in common; or You gain X life, where X is the greatest number of creatures you control that have a creature type in common.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

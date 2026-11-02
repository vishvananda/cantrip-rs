// Rakdos, the Showstopper
// status: ready
// Flying, trample
// When Rakdos enters, flip a coin for each creature that isn't a Demon, Devil, or Imp. Destroy each creature whose coin comes up tails.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut flips_0: Vec<(ObjectId, bool)> = Vec::new(); let mut done_0: i64 = 0; for object in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Demon)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Devil)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Imp)))))).collect::<Vec<ObjectId>>() } { let flip = g.flip_coin_for_heads(player_0); if flip { done_0 += 1; } flips_0.push((object, flip)); }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && flips_0.iter().any(|&(flipped, heads)| flipped == id && heads == false))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rakdos, the Showstopper", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Rakdos enters, flip a coin for each creature that isn't a Demon, Devil, or Imp. Destroy each creature whose coin comes up tails.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

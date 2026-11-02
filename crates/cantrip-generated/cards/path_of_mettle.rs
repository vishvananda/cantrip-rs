// Path of Mettle // Metzali, Tower of Triumph
// status: ready
// (Transforms from Path of Mettle.)
// {T}: Add one mana of any color.
// {1}{R}, {T}: Metzali deals 2 damage to each opponent.
// {2}{W}, {T}: Choose a creature at random that attacked this turn. Destroy that creature.
// When Path of Mettle enters, it deals 1 damage to each creature that doesn't have first strike, double strike, vigilance, or haste.
// Whenever you attack with at least two creatures that have first strike, double strike, vigilance, and/or haste, transform Path of Mettle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_0 = { let q: &dyn Query = g; item.subject.object() }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(q.chars(id).keywords.has(Keyword::FirstStrike)) && !(q.chars(id).keywords.has(Keyword::DoubleStrike)) && !(q.chars(id).keywords.has(Keyword::Vigilance)) && !(q.chars(id).keywords.has(Keyword::Haste)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && (q.chars(id).keywords.has(Keyword::FirstStrike) || q.chars(id).keywords.has(Keyword::DoubleStrike) || q.chars(id).keywords.has(Keyword::Vigilance) || q.chars(id).keywords.has(Keyword::Haste))) && Some(asker.controller) == Some(q.controller(id)) && true && (q.state().attackers.iter().filter(|a| { let (id, defender) = (a.id, &a.target); (q.chars(id).is(CardType::Creature) && (q.chars(id).keywords.has(Keyword::FirstStrike) || q.chars(id).keywords.has(Keyword::DoubleStrike) || q.chars(id).keywords.has(Keyword::Vigilance) || q.chars(id).keywords.has(Keyword::Haste))) && Some(asker.controller) == Some(q.controller(id)) && true }).count() as i64) >= (2i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Path of Mettle", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Path of Mettle enters, it deals 1 damage to each creature that doesn't have first strike, double strike, vigilance, or haste.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever you attack with at least two creatures that have first strike, double strike, vigilance, and/or haste, transform Path of Mettle.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_1 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let mut attackers: Vec<ObjectId> = q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, known, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(id) == Some(id) { return Some(Fired { subject, data }); } }  None }).filter_map(|f: Fired| f.subject.object()).collect(); attackers.sort(); attackers.dedup(); attackers.len() as i64 }) >= (1i64))).collect::<Vec<ObjectId>>() }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_2: Vec<ObjectId> = Vec::new(); while picked_2.len() < count_2 && !pool_2.is_empty() { let at = g.choose_at_random(pool_2.len()); picked_2.push(pool_2.remove(at)); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (picked_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Metzali, Tower of Triumph", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "{T}: Add one mana of any color."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{1}{R}, {T}: Metzali deals 2 damage to each opponent.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{2}{W}, {T}: Choose a creature at random that attacked this turn. Destroy that creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}

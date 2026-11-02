// Goldbug, Scrappy Scout // Goldbug, Humanity's Ally
// status: planned
// requires: engine: no LivingMetal
// requires: engine: no MoreThanMeetsTheEye
// Living metal (During your turn, this Vehicle is also a creature.)
// Human spells you control can't be countered.
// Whenever Goldbug and at least one Human attack, draw a card and convert Goldbug.
// More Than Meets the Eye {W}{U} (You may cast this card converted for {W}{U}.)
// Prevent all combat damage that would be dealt to attacking Humans you control.
// Whenever you cast your second spell each turn, convert Goldbug.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && matches!(*target, Target::Object(o) if ((q.state().attackers.iter().any(|a| a.id == o) || q.was_attacking(o)) && q.chars(o).has_subtype(q.words(asker.source).subtype(Subtype::Human)) && Some(asker.controller) == Some(q.controller(o)))) { return true; } }
 false }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true && (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true { return Some(Fired { subject, data }); } }  None }).filter(|f: &Fired| f.data[0] == data[0]).count() as i64 + 1) == (2i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-more-than-meets-the-eye" => ways: cantrip_mechanics::more_than_meets_the_eye::way(ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "More Than Meets the Eye {W}{U}"));

pub static CARD: CardDef = CardDef { name: "Goldbug, Humanity's Ally", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast your second spell each turn, convert Goldbug.").targets(&[])], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all combat damage that would be dealt to attacking Humans you control", applies: ability_1_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Human)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::CantBeCountered); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Human)) && true && q.state().attackers.iter().any(|a| Some(a.id) == Some(asker.source)) && (q.state().attackers.iter().filter(|a| { let (id, defender) = (a.id, &a.target); q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Human)) && true && Some(a.id) != Some(asker.source) }).count() as i64) >= (1i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-living-metal" => statics: cantrip_mechanics::living_metal::ON_YOUR_TURN);

pub static CARD: CardDef = CardDef { name: "Goldbug, Scrappy Scout", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(3)), power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever Goldbug and at least one Human attack, draw a card and convert Goldbug.").targets(&[]).once_per_batch()], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Human spells you control can't be countered.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}

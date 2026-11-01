// Angelic Enforcer // Enduring Angel
// status: ready
// Flying
// You have hexproof.
// Angelic Enforcer's power and toughness are each equal to your life total.
// Whenever this creature attacks, double your life total.
// Flying, double strike
// You have hexproof.
// If your life total would be reduced to 0 or less, instead transform this creature and your life total becomes 3. Then if this creature didn't transform this way, you lose the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::LifeChange(l) = proposal { let amount = q.state().player(l.player).life.saturating_add(l.amount); let subject = Target::Player(l.player); let data = [amount.max(0) as u32, u32::MAX]; if l.amount < 0 && Some(asker.controller) == Some(l.player) && (i64::from(amount)) <= (0i64) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_3_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::LifeChange(seen) = &seen { let l = seen; let amount = q.state().player(l.player).life.saturating_add(l.amount); let subject = Target::Player(l.player); let data = [amount.max(0) as u32, u32::MAX]; if let Proposal::LifeChange(l) = proposal { l.amount = 0; } } }
#[allow(unused_variables, unused_mut)] fn ability_3_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform(object)?; }
let amount_1 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_1 { g.set_life_total(player, amount_1); }
if { let q: &dyn Query = g; !(Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).transformed)) } {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
}
 Ok(()) }
#[allow(unused_variables)] fn ability_3_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::LifeChange(l) = before { let q: &dyn Query = g; let amount = q.state().player(l.player).life.saturating_add(l.amount); let subject = Target::Player(l.player); let data = [amount.max(0) as u32, u32::MAX]; let _ = ability_3_then_steps(g, asker, subject, data); } }

pub static CARD: CardDef = CardDef { name: "Enduring Angel", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::DoubleStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::LifeChange, ability_3_applies, ability_3_apply, "If your life total would be reduced to 0 or less, instead transform this creature and your life total becomes 3. Then if this creature didn't transform this way, you lose the game.").then(ability_3_then)], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_2_0_rule }], back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).clamp(-32768, 32767) as i16, (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((2i64 * Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life)))).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for player in players_0 { g.set_life_total(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Angelic Enforcer", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_3_event), ability_3, "Whenever this creature attacks, double your life total.").targets(&[])], statics: &[characteristic_defining(Layer::PtDefine, ability_2_apply_0, "Angelic Enforcer's power and toughness are each equal to your life total.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_1_0_rule }], ..CardDef::BLANK };
}

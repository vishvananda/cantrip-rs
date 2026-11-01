// Mirri, Weatherlight Duelist
// status: planned
// requires: player rule for a while BlockerLimit
// First strike
// Whenever Mirri attacks, each opponent can't block with more than one creature this combat.
// As long as Mirri is tapped, no more than one creature can attack you each combat.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-blocker-limit" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::BlockerLimit(1i64)), Duration::EndOfCombat); } });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_limit(q: &dyn Query, asker: &Asker, attacked: Target) -> Option<u32> { (attacked == Target::Player(asker.controller) && Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).tapped)).then_some(1) }

pub static CARD: CardDef = CardDef { name: "Mirri, Weatherlight Duelist", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Mirri attacks, each opponent can't block with more than one creature this combat.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attack_limits: &[ability_2_0_limit], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
